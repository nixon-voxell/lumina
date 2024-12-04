use bevy::audio::*;
use bevy::prelude::*;
use lumina_shared::prelude::*;

use crate::ui::Screen;

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // Prespawn the bg music entity
        app.world_mut().spawn(BgMusic);

        app.add_systems(OnEnter(Screen::MainMenu), play_main_menu_bg)
            .add_systems(OnEnter(Screen::InGame), play_in_game_bg)
            .add_systems(Update, cannon_fire);
    }
}

fn cannon_fire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_fire_ammo: EventReader<FireAmmo>,
) {
    for fire_ammo in evr_fire_ammo.read() {
        let position = fire_ammo.position;
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/Cannon.ogg"),
                settings: PlaybackSettings::DESPAWN.with_spatial(true),
            },
            TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.0)),
        ));
    }
}

fn play_main_menu_bg(
    mut commands: Commands,
    q_bg_music: Query<Entity, With<BgMusic>>,
    asset_server: Res<AssetServer>,
) {
    let entity = q_bg_music.single();

    commands.entity(entity).insert(AudioBundle {
        source: asset_server.load("audio/bg_music/MainMenu.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
    });
}

fn play_in_game_bg(
    mut commands: Commands,
    q_bg_music: Query<Entity, With<BgMusic>>,
    asset_server: Res<AssetServer>,
) {
    println!("\n\nin game bg");
    let entity = q_bg_music.single();

    commands.entity(entity).insert(AudioBundle {
        source: asset_server.load("audio/bg_music/InGame.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
    });
}

#[derive(Component)]
pub struct BgMusic;
