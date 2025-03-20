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
            .add_systems(
                PostUpdate,
                ammo_vfx_visibility.after(VisibilitySystems::VisibilityPropagate),
            )
            .observe(ammo_hit_vfx);
    }
}

// A hack for making all vfx still visible even if some of them are off screen.
fn ammo_vfx_visibility(mut q_viz: Query<&mut ViewVisibility, With<ParticleEffectInstance>>) {
    for mut viz in q_viz.iter_mut() {
        viz.set();
    }
}

fn ammo_hit_vfx(
    trigger: Trigger<AmmoHit>,
    mut commands: Commands,
    material_handle: Res<AmmoHitMaterialHandle>,
) {
    let ammo_hit = trigger.event();
    commands.trigger(DespawnVfx {
        vfx_type: DespawnVfxType::AmmoHit,
        transform: Transform::from_translation(ammo_hit.extend(10.0)),
        material: material_handle.clone_weak(),
    });
}

fn setup_ammo_vfx(mut commands: Commands, mut materials: ResMut<Assets<AmmoHitMaterial>>) {
    commands.insert_resource(AmmoHitMaterialHandle(
        materials.add(AmmoHitMaterial::default()),
    ));
}

#[derive(Resource, Deref)]
struct AmmoHitMaterialHandle(Handle<AmmoHitMaterial>);
