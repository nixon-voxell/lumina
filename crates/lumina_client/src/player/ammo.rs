use bevy::prelude::*;
use bevy::render::view::VisibilitySystems;
use bevy_enoki::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Particle2dMaterialPlugin::<AmmoHitMaterial>::default())
            .add_systems(Startup, setup_ammo_vfx)
            .add_systems(Update, ammo_hit_vfx)
            .add_systems(
                PostUpdate,
                ammo_vfx_visibility.after(VisibilitySystems::VisibilityPropagate),
            );
    }
}

// A hack for making all vfx still visible even if some of them are off screen.
fn ammo_vfx_visibility(mut q_viz: Query<&mut ViewVisibility, With<ParticleEffectInstance>>) {
    for mut viz in q_viz.iter_mut() {
        viz.set();
    }
}

fn ammo_hit_vfx(
    mut commands: Commands,
    material_handle: Res<AmmoHitMaterialHandle>,
    mut evr_ammo_hit: EventReader<AmmoHit>,
) {
    for ammo_hit in evr_ammo_hit.read() {
        commands.trigger(DespawnVfx {
            vfx_type: DespawnVfxType::AmmoHit,
            transform: Transform::from_translation(ammo_hit.extend(10.0)),
            material: material_handle.clone_weak(),
        });
    }
}

fn setup_ammo_vfx(mut commands: Commands, mut materials: ResMut<Assets<AmmoHitMaterial>>) {
    commands.insert_resource(AmmoHitMaterialHandle(
        materials.add(AmmoHitMaterial::default()),
    ));
}

#[derive(Resource, Deref)]
struct AmmoHitMaterialHandle(Handle<AmmoHitMaterial>);
