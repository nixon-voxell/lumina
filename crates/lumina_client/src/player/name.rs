use bevy::prelude::*;
use bevy_radiance_cascades::NoRadiance;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;

use crate::LocalClientId;

use super::CachedGameStat;

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
    mut q_names: Query<(&SpaceshipName, &mut Style, Entity)>,
) {
    for (name, mut style, entity) in q_names.iter_mut() {
        let Ok(translation) = q_global_transforms.get(**name).map(|g| g.translation()) else {
            commands.entity(entity).despawn();
            continue;
        };

        style.left = Val::Px(translation.x);
        style.top = Val::Px(translation.y - OFFSET);
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
    game_stat: Res<CachedGameStat>,
    color_palette: Res<ColorPalette>,
) {
    let CachedGameStat {
        team_type: Some(local_team_type),
        ..
    } = *game_stat
    else {
        return;
    };

    for (team_type, id, _global_transform, entity) in q_spaceships.iter() {
        let name = NAMES[id.to_bits() as usize % NAMES.len()];
        let color = if *team_type == local_team_type {
            Color::linear_rgba(0.0, 4.0, 4.0, 1.0)
        } else {
            color_palette.red.with_luminance(4.0)
        };

        commands
            .spawn((
                SpaceshipName(entity),
                Text2dBundle {
                    text: Text::from_section(name, TextStyle { color, ..default() }),
                    // transform: Transform::from_translation(
                    //     (global_transform.translation()
                    //         - Vec3::Y * OFFSET
                    //         - camera_transform.translation())
                    //         / proj.scale,
                    // ),
                    text_anchor: bevy::sprite::Anchor::TopCenter,
                    ..default()
                },
                NoRadiance,
                // RenderLayers::layer(1),
            ))
            .insert(NodeBundle::default());

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
