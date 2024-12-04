use bevy::audio::*;
use bevy::prelude::*;
use lumina_shared::prelude::*;

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bg_music)
            .add_systems(Update, cannon_fire);
    }
}

fn bg_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/BgMusic.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
    });
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
