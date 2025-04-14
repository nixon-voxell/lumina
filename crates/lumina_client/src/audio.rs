use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource as KiraAudioSouce;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::{camera::GameCamera, player::LocalPlayerId, screens::Screen};

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin);

        app.init_resource::<Background>()
            .add_audio_channel::<Background>()
            .init_resource::<SoundFx>()
            .add_audio_channel::<SoundFx>();

        app.add_systems(Startup, setup_default_channel_settings)
            .add_systems(OnEnter(Screen::MainMenu), play_main_menu_music)
            .add_systems(OnEnter(Screen::InGame), play_in_game_music)
            .add_systems(Update, button_interaction)
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
    q_weapon: Query<(&WeaponType, &PlayerId)>,
    sound_fx: Res<SoundFx>,
    channel: Res<AudioChannel<SoundFx>>,
    local_player_id: Res<LocalPlayerId>,
) {
    let fire_ammo = trigger.event();
    // let position = fire_ammo.position;

    let Ok((weapon_type, id)) = q_weapon.get(fire_ammo.weapon_entity) else {
        return;
    };

    let is_local = local_player_id.0 == *id;
    let audio_handle = match weapon_type {
        WeaponType::Cannon => sound_fx.cannon_shot.clone_weak(),
        WeaponType::GattlingGun => sound_fx.gattling_shot.clone_weak(),
    };

    if is_local {
        channel.play(audio_handle);
    }

    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("audio/Cannon.ogg"),
    //         settings: PlaybackSettings::DESPAWN.with_spatial(true),
    //     },
    //     TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
    // ));
}

fn ammo_hit(trigger: Trigger<AmmoHit>, mut commands: Commands, asset_server: Res<AssetServer>) {
    // let position = trigger.event();
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("audio/AmmoHit.ogg"),
    //         settings: PlaybackSettings::DESPAWN.with_spatial(true),
    //     },
    //     TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
    // ));
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

fn setup_default_channel_settings(
    background_channel: Res<AudioChannel<Background>>,
    // soundfx_channel: ResMut<AudioChannel<SoundFx>>,
) {
    background_channel.set_volume(0.5);
}

/// Marker for background audio channel.
#[derive(Resource)]
pub struct Background {
    main_menu: Handle<KiraAudioSouce>,
    in_game: Handle<KiraAudioSouce>,
}

impl FromWorld for Background {
    fn from_world(world: &mut World) -> Self {
        Self {
            main_menu: world.load_asset("audio/bg_music/main-menu.ogg"),
            in_game: world.load_asset("audio/bg_music/in-game.ogg"),
        }
    }
}

/// Marker for sound effects audio channel.
#[derive(Resource)]
pub struct SoundFx {
    button_click: Handle<KiraAudioSouce>,
    button_hover: Handle<KiraAudioSouce>,
    cannon_shot: Handle<KiraAudioSouce>,
    gattling_shot: Handle<KiraAudioSouce>,
}

impl FromWorld for SoundFx {
    fn from_world(world: &mut World) -> Self {
        Self {
            button_click: world.load_asset("audio/ui/button-click.ogg"),
            button_hover: world.load_asset("audio/ui/button-hover.ogg"),
            cannon_shot: world.load_asset("audio/weapon/cannon-shot.ogg"),
            gattling_shot: world.load_asset("audio/weapon/gattling-shot.ogg"),
        }
    }
}
