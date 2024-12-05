use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_enoki::prelude::*;
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EnokiPlugin, ShaderUtilsPlugin))
        .add_plugins(Particle2dMaterialPlugin::<AmmoHitMaterial>::default());

    app.add_systems(Startup, setup)
        .add_systems(Update, spawn_spark);

    app.run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<AmmoHitMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::Srgba(Srgba::hex("19181A").unwrap()).into(),
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::default(),
    ));
    commands.spawn((
        ParticleSpawnerBundle {
            state: ParticleSpawnerState {
                active: false,
                ..default()
            },
            effect: asset_server.load("enoki/gun_sparks.ron"),
            material: DEFAULT_MATERIAL,
            ..default()
        },
        OneShot::Deactivate,
    ));
    commands.spawn((
        ParticleSpawnerBundle {
            state: ParticleSpawnerState {
                active: false,
                ..default()
            },
            effect: asset_server.load("enoki/ammo_hit.ron"),
            material: materials.add(AmmoHitMaterial::default()),
            ..default()
        },
        OneShot::Deactivate,
    ));
}

fn spawn_spark(
    mut q_particle_states: Query<(&mut ParticleSpawnerState, Option<&Handle<AmmoHitMaterial>>)>,
    // mut materials: ResMut<Assets<AmmoHitMaterial>>,
    button: Res<ButtonInput<MouseButton>>,
) {
    if button.just_pressed(MouseButton::Left) {
        for (mut particle_states, material) in q_particle_states.iter_mut() {
            particle_states.active = true;

            // if let Some(material) = material.and_then(|handle| materials.get_mut(handle)) {
            //     material.variation = rand::random::<f32>() * 1000.0;
            // }
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct AmmoHitMaterial {}

impl Particle2dMaterial for AmmoHitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enoki/ammo_hit.wgsl".into()
    }
}
