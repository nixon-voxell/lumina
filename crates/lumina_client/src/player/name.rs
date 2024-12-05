use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::camera::GameCamera;
use crate::LocalClientId;

pub(super) struct NamePlugin;

impl Plugin for NamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_spaceship_names.run_if(resource_exists::<LocalClientId>),
                update_spaceship_names,
            ),
        );
    }
}

fn update_spaceship_names(
    mut commands: Commands,
    q_global_transforms: Query<&GlobalTransform>,
    mut q_names: Query<(&SpaceshipName, &mut Transform, Entity)>,
    q_game_camera: Query<(&GlobalTransform, &OrthographicProjection), With<GameCamera>>,
) {
    let Ok((camera_transform, proj)) = q_game_camera.get_single() else {
        return;
    };

    for (name, mut transform, entity) in q_names.iter_mut() {
        let Ok(global_transform) = q_global_transforms.get(**name) else {
            commands.entity(entity).despawn();
            continue;
        };

        transform.translation =
            (global_transform.translation() - Vec3::Y * OFFSET - camera_transform.translation())
                / proj.scale;
    }
}

fn init_spaceship_names(
    mut commands: Commands,
    q_spaceships: Query<
        (&TeamType, &PlayerId, &GlobalTransform, Entity),
        (
            With<Spaceship>,
            With<SourceEntity>,
            Without<SpaceshipNameInitialized>,
        ),
    >,
    q_game_camera: Query<(&GlobalTransform, &OrthographicProjection), With<GameCamera>>,
) {
    let Ok((camera_transform, proj)) = q_game_camera.get_single() else {
        return;
    };

    for (team_type, id, global_transform, entity) in q_spaceships.iter() {
        let name = NAMES[id.to_bits() as usize % NAMES.len()];
        let color = match team_type {
            TeamType::A => Color::Srgba(Srgba::hex("A9DC76").unwrap()),
            TeamType::B => Color::Srgba(Srgba::hex("FF6188").unwrap()),
        };

        commands.spawn((
            SpaceshipName(entity),
            Text2dBundle {
                text: Text::from_section(name, TextStyle { color, ..default() }),
                transform: Transform::from_translation(
                    (global_transform.translation()
                        - Vec3::Y * OFFSET
                        - camera_transform.translation())
                        / proj.scale,
                ),
                text_anchor: bevy::sprite::Anchor::TopCenter,
                ..default()
            },
            RenderLayers::layer(1),
        ));

        commands.entity(entity).insert(SpaceshipNameInitialized);
    }
}

/// Holds the entity of the spaceship.
#[derive(Component, Deref)]
pub struct SpaceshipName(Entity);

/// Component tag to identify if a spaceship has been initialized with a name.
#[derive(Component)]
pub struct SpaceshipNameInitialized;

const OFFSET: f32 = 50.0;

const NAMES: [&str; 50] = [
    "Luminara",
    "Gloam",
    "Radiant",
    "Obscura",
    "Lumis",
    "Eclipse",
    "Flare",
    "Nebule",
    "Photon",
    "Umbra",
    "Glimmer",
    "Dusk",
    "Aura",
    "Nova",
    "Eclipse",
    "Brilliance",
    "Cimmer",
    "Prism",
    "Dawn",
    "Gleam",
    "Strobe",
    "Halo",
    "Penumbra",
    "Chroma",
    "Solara",
    "Shimmer",
    "Tenebris",
    "Zenith",
    "Corona",
    "Mirage",
    "Flicker",
    "Oscura",
    "Twilight",
    "Luster",
    "Ember",
    "Glow",
    "Luxis",
    "Eclipse",
    "Radiance",
    "Silhouette",
    "Gleam",
    "Flash",
    "Scintilla",
    "Darken",
    "Shine",
    "Solis",
    "Shadow",
    "Flarescape",
    "Obscurial",
    "Luminae",
];
