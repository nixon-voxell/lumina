use std::time::Duration;

use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::{camera::GameCamera, player::LocalPlayerId, screens::Screen};

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin);

        app.insert_resource(SpatialAudio {
            max_distance: 1600.,
        })
        .init_resource::<Background>()
        .add_audio_channel::<Background>()
        .init_resource::<SoundFx>()
        .add_audio_channel::<SoundFx>()
        .init_resource::<EmitterPool>();

        app.add_systems(Startup, setup_default_channel_settings)
            .add_systems(OnEnter(Screen::MainMenu), play_main_menu_music)
            .add_systems(OnEnter(Screen::InGame), play_in_game_music)
            .add_systems(
                Update,
                (
                    setup_audio_emitter::<Or<(With<Weapon>, With<Spaceship>)>>,
                    button_interaction,
                    return_emitter_pool,
                ),
            )
            .observe(init_audio_receiver)
            .observe(fire_ammo)
            .observe(ammo_hit);
    }
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
    let position = trigger.event();
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

fn play_main_menu_music(background: Res<Background>, channel: Res<AudioChannel<Background>>) {
    channel.stop();
    channel
        .play(background.main_menu.clone_weak())
        .fade_in(AudioTween::new(
            Duration::from_secs(1),
            AudioEasing::InOutPowi(2),
        ))
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
    q_criteria: Query<Entity, (With<SourceEntity>, Without<AudioEmitter>, Filter)>,
) {
    for entity in q_criteria.iter() {
        commands.entity(entity).insert(AudioEmitter::default());
    }
}

fn setup_default_channel_settings(
    background_channel: Res<AudioChannel<Background>>,
    // soundfx_channel: ResMut<AudioChannel<SoundFx>>,
) {
    background_channel.set_volume(0.5);
}

fn return_emitter_pool(
    q_emitters: Query<&AudioEmitter, Changed<AudioEmitter>>,
    mut pool: ResMut<EmitterPool>,
) {
    let unused_entities = pool
        .used()
        .iter()
        .filter(|entity| {
            // When emitter instances are empty, cound it as unused.
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
        ammo_hit: "audio/weapon/ammo-hit.ogg",
        cannon_shot: "audio/weapon/cannon-shot.ogg",
        gattling_shot: "audio/weapon/gattling-shot.ogg",
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
