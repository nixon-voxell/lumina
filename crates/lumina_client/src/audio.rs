use std::time::Duration;

use avian2d::prelude::LinearVelocity;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_motiongfx::prelude::ease;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::camera::GameCamera;
use crate::player::LocalPlayerId;
use crate::screens::Screen;

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin);

        app.init_resource::<AudioVolumeSettings>()
            .insert_resource(SpatialAudio {
                max_distance: 1600.,
            })
            .init_resource::<Background>()
            .add_audio_channel::<Background>()
            .init_resource::<SoundFx>()
            .add_audio_channel::<SoundFx>()
            .init_resource::<EmitterPool>();

        // This is required to set before the startup system/state event trigger happens.
        let audio_volume_settings = AudioVolumeSettings::default();
        app.world_mut()
            .resource_mut::<AudioChannel<Background>>()
            .set_volume(audio_volume_settings.bgm_volume);
        app.world_mut()
            .resource_mut::<AudioChannel<Background>>()
            .set_volume(audio_volume_settings.vfx_volume);

        app.add_systems(OnEnter(Screen::MainMenu), play_main_menu_music)
            .add_systems(OnEnter(Screen::InGame), play_in_game_music)
            .add_systems(
                Update,
                (
                    kill,
                    update_volume_settings.run_if(resource_changed::<AudioVolumeSettings>),
                    setup_audio_emitter::<Or<(With<Weapon>, With<Spaceship>)>>,
                    button_interaction,
                    return_emitter_pool,
                    spaceship_velocity_pitch,
                    setup_spaceship_audio,
                ),
            )
            .observe(init_audio_receiver)
            .observe(fire_ammo)
            .observe(ammo_hit)
            .observe(cleanup_removed_instances);
    }
}

fn kill(
    mut events: EventReader<MessageEvent<KilledPlayer>>,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
) {
    for event in events.read() {
        channel.play(sound_fx.kill.clone_weak());

        let mut cmd = match event.message().streak_count {
            1 => channel.play(sound_fx.target_down.clone_weak()),
            2 => channel.play(sound_fx.double_down.clone_weak()),
            3 => channel.play(sound_fx.on_fire.clone_weak()),
            4 => channel.play(sound_fx.killing_spree.clone_weak()),
            5 => channel.play(sound_fx.unstoppable.clone_weak()),
            6 => channel.play(sound_fx.dominating.clone_weak()),
            7 => channel.play(sound_fx.godlike.clone_weak()),
            // Randomly play one if we are over the voice line count.
            _ => match rand::random_range(0..4) {
                0 => channel.play(sound_fx.killing_spree.clone_weak()),
                1 => channel.play(sound_fx.unstoppable.clone_weak()),
                2 => channel.play(sound_fx.dominating.clone_weak()),
                3 => channel.play(sound_fx.godlike.clone_weak()),
                _ => unreachable!(),
            },
        };

        cmd.with_volume(3.0);
    }
}

fn fire_ammo(
    trigger: Trigger<FireAmmo>,
    mut q_weapon: Query<(&mut AudioEmitter, &WeaponType, &PlayerId)>,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
    local_player_id: Res<LocalPlayerId>,
) {
    let fire_ammo = trigger.event();

    let Ok((mut emitter, weapon_type, id)) = q_weapon.get_mut(fire_ammo.weapon_entity) else {
        return;
    };

    let is_local = local_player_id.0 == *id;
    let audio_handle = match weapon_type {
        WeaponType::Cannon => sound_fx.cannon_shot.clone_weak(),
        WeaponType::GattlingGun => sound_fx.gattling_shot.clone_weak(),
    };

    let mut play_command = channel.play(audio_handle);
    play_command.with_playback_rate(rand::random_range(0.85..=1.0));

    // Use spatial audio for other spaceships.
    if is_local == false {
        // Let the spatial audio system decide the volume.
        emitter
            .instances
            .push(play_command.with_volume(0.0).handle());
    }
}

fn ammo_hit(
    trigger: Trigger<AmmoHit>,
    mut commands: Commands,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
    mut emitter_pool: ResMut<EmitterPool>,
) {
    let position = trigger.event().position;
    let entity = emitter_pool.get_unused_or_spawn(|| commands.spawn_empty().id());

    commands.entity(entity).insert((
        TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
        AudioEmitter {
            instances: vec![channel
                .play(sound_fx.ammo_hit.clone_weak())
                .with_playback_rate(rand::random_range(0.85..=1.0))
                // Let the spatial audio system decide the volume.
                .with_volume(0.0)
                .handle()],
        },
    ));
}

/// Change spaceship audio pitch based on its [`LinearVelocity`].
fn spaceship_velocity_pitch(
    q_spaceships: Query<(&LinearVelocity, &Spaceship, &Handle<AudioInstance>), With<SourceEntity>>,
    mut instances: ResMut<Assets<AudioInstance>>,
) {
    const MAX_RATE: f32 = 3.0;

    for (velocity, spaceship, instance_handle) in q_spaceships.iter() {
        // Apply ease to zoom more towards maximal velocity and vice versa.
        let velocity_factor =
            ease::quad::ease_in_out(velocity.length() / spaceship.movement.max_linear_speed)
                .clamp(0.0, 1.0);
        let playback_rate = 1.0.lerp(MAX_RATE, velocity_factor) as f64;

        if let Some(instance) = instances.get_mut(instance_handle) {
            instance.set_playback_rate(playback_rate, AudioTween::default());
        }
    }
}

/// Setup audio instances for each spaceship.
/// Use spatial audio for non local spaceships.
fn setup_spaceship_audio(
    mut commands: Commands,
    mut q_spaceships: Query<
        (&mut AudioEmitter, &PlayerId, Entity),
        (Added<AudioEmitter>, With<Spaceship>, With<SourceEntity>),
    >,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
    local_player_id: Res<LocalPlayerId>,
) {
    for (mut emitter, id, entity) in q_spaceships.iter_mut() {
        let mut audio_cmd = channel.play(sound_fx.idle.clone_weak());
        audio_cmd.linear_fade_in(Duration::from_secs(2)).looped();

        // Apply spatial audio for non local spaceships.
        if *id != **local_player_id {
            let handle = audio_cmd.with_volume(0.0).handle();
            emitter.instances.push(handle.clone_weak());
            commands.entity(entity).insert(handle);
        } else {
            commands.entity(entity).insert(audio_cmd.handle());
        }
    }
}

fn cleanup_removed_instances(
    trigger: Trigger<OnRemove, Handle<AudioInstance>>,
    q_instances: Query<&Handle<AudioInstance>>,
    mut instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(mut instance) = q_instances
        .get(trigger.entity())
        .ok()
        .and_then(|h| instances.remove(h))
    {
        instance.stop(AudioTween::default());
    }
}

fn play_main_menu_music(background: Res<Background>, channel: Res<AudioChannel<Background>>) {
    channel.stop();
    channel
        .play(background.main_menu.clone_weak())
        .fade_in(AudioTween::new(
            Duration::from_secs(1),
            AudioEasing::InOutPowi(2),
        ))
        .with_volume(0.6)
        .looped();
}

fn play_in_game_music(background: Res<Background>, channel: Res<AudioChannel<Background>>) {
    channel.stop();
    channel
        .play(background.in_game.clone_weak())
        .fade_in(AudioTween::new(
            Duration::from_secs(1),
            AudioEasing::InOutPowi(2),
        ))
        .with_volume(0.5)
        .looped();
}

fn init_audio_receiver(trigger: Trigger<OnAdd, GameCamera>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert(AudioReceiver);
}

fn setup_audio_emitter<Filter: QueryFilter>(
    mut commands: Commands,
    q_criteria: Query<Entity, (Without<AudioEmitter>, Filter, With<SourceEntity>)>,
) {
    for entity in q_criteria.iter() {
        commands.entity(entity).insert(AudioEmitter::default());
    }
}

fn return_emitter_pool(
    q_emitters: Query<&AudioEmitter, Changed<AudioEmitter>>,
    mut pool: ResMut<EmitterPool>,
) {
    let unused_entities = pool
        .used()
        .iter()
        .filter(|entity| {
            // When emitter instances are empty, count it as unused.
            q_emitters
                .get(**entity)
                .map(|emitter| emitter.instances.is_empty())
                .unwrap_or_default()
        })
        .cloned()
        .collect::<Vec<_>>();

    for entity in unused_entities {
        pool.set_unused(entity);
    }
}

fn update_volume_settings(
    bgm_channel: Res<AudioChannel<Background>>,
    vfx_channel: Res<AudioChannel<SoundFx>>,
    settings: Res<AudioVolumeSettings>,
) {
    bgm_channel.set_volume(settings.bgm_volume);
    vfx_channel.set_volume(settings.vfx_volume);
}

fn button_interaction(
    q_interactions: Query<&Interaction, Changed<Interaction>>,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
) {
    for interaction in q_interactions.iter() {
        let handle = match interaction {
            Interaction::Pressed => sound_fx.button_click.clone_weak(),
            Interaction::Hovered => sound_fx.button_hover.clone_weak(),
            Interaction::None => continue,
        };

        channel.play(handle);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct EmitterPool(EntityPool);

#[derive(Bundle)]
pub struct EmitterBundle {
    emitter: AudioEmitter,
    spatial: SpatialBundle,
}

AudioChannelTracks!(
    /// Marker for background audio channel.
    #[derive(Resource)]
    pub struct Background {},
    tracks {
        main_menu: "audio/bg_music/main-menu.ogg",
        in_game: "audio/bg_music/in-game.ogg",
    }
);

AudioChannelTracks!(
    /// Marker for sound effects audio channel.
    #[derive(Resource)]
    pub struct SoundFx {},
    tracks {
        button_click: "audio/ui/button-click.ogg",
        button_hover: "audio/ui/button-hover.ogg",
        idle: "audio/spaceship/idle.ogg",
        ammo_hit: "audio/weapon/ammo-hit.ogg",
        cannon_shot: "audio/weapon/cannon-shot.ogg",
        gattling_shot: "audio/weapon/gattling-shot.ogg",
        kill: "audio/sfx/kill.ogg",
        target_down: "audio/streak/target_down.ogg",
        double_down: "audio/streak/double_down.ogg",
        on_fire: "audio/streak/on_fire.ogg",
        killing_spree: "audio/streak/killing_spree.ogg",
        unstoppable: "audio/streak/unstoppable.ogg",
        dominating: "audio/streak/dominating.ogg",
        godlike: "audio/streak/godlike.ogg",
    }
);

#[macro_export]
macro_rules! AudioChannelTracks {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct_name:ident {},
        tracks {
            $($field_name:ident: $audio_path:literal,)*
        }
    ) => {
        $( #[$attr] )*
        $viz struct $struct_name {
            $($field_name: Handle<::bevy_kira_audio::AudioSource>,)*
        }

        impl ::bevy::ecs::world::FromWorld for $struct_name {
            fn from_world(world: &mut World) -> Self {
                Self {
                    $($field_name: world.load_asset($audio_path),)*
                }
            }
        }
    };
}
pub use AudioChannelTracks;

#[derive(Resource)]
pub struct AudioVolumeSettings {
    pub bgm_volume: f64,
    pub vfx_volume: f64,
}

impl Default for AudioVolumeSettings {
    fn default() -> Self {
        AudioVolumeSettings {
            bgm_volume: 0.5,
            vfx_volume: 1.0,
        }
    }
}
