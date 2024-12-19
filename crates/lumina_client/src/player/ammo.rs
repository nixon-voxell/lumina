use bevy::prelude::*;
use bevy::render::view::VisibilitySystems;
use bevy_enoki::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

use crate::camera::CameraShake;

pub(super) struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Particle2dMaterialPlugin::<AmmoHitMaterial>::default())
            .add_systems(Startup, setup_ammo_vfx)
            .add_systems(Update, (hit_cam_shake, ammo_hit_vfx))
            .add_systems(
                PostUpdate,
                ammo_vfx_visibility.after(VisibilitySystems::VisibilityPropagate),
            );
    }
}

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

fn hit_cam_shake(mut evr_ammo_hit: EventReader<AmmoHit>, mut camera_shake: ResMut<CameraShake>) {
    for _ in evr_ammo_hit.read() {
        camera_shake.add_trauma_with_threshold(0.3, 0.4);
    }
}

fn setup_ammo_vfx(mut commands: Commands, mut materials: ResMut<Assets<AmmoHitMaterial>>) {
    commands.insert_resource(AmmoHitMaterialHandle(
        materials.add(AmmoHitMaterial::default()),
    ));
}

#[derive(Resource, Deref)]
struct AmmoHitMaterialHandle(Handle<AmmoHitMaterial>);
