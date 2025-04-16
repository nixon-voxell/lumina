use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Mesh2dHandle;
use bevy_enoki::prelude::*;
use bevy_motiongfx::prelude::ease;
use blenvy::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_vfx::prelude::*;

pub(super) struct AbilityVfxPlugin;

impl Plugin for AbilityVfxPlugin {
    fn build(&self, app: &mut App) {
        app
            // Vfx cue timer for all the abilities.
            .add_plugins((
                VfxCueTimerPlugin::<ShadowAbilityConfig>::default(),
                VfxCueTimerPlugin::<HealAbilityConfig>::default(),
            ))
            // Record spaceship origin colors.
            .add_plugins(OriginColorsPlugin::<(
                Added<BlueprintInstanceReady>,
                With<Spaceship>,
            )>::default())
            .add_plugins((shadow_vfx, heal_vfx));
    }
}

// SHADOW
// ==============================
fn shadow_vfx(app: &mut App) {
    app.add_systems(Update, (cue_in_shadow_vfx, cue_out_shadow_vfx))
        .observe(cue_out_finished_shadow_vfx);
}

/// Apply shadow ability effect for spaceships.
fn cue_in_shadow_vfx(
    q_shadows: Query<(&VfxCueInTimer, &ShadowAbilityConfig, &OriginColors)>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (cue_in, config, origin_colors) in q_shadows.iter() {
        let strength = config.ability().strength;
        let transition =
            ease::cubic::ease_in_out((cue_in.elapsed_secs() / config.cue_in_duration).min(1.0));

        for (entity, origin_color) in origin_colors.iter() {
            let Some(color_material) = q_color_materials
                .get(*entity)
                .ok()
                .and_then(|handle| color_materials.get_mut(handle))
            else {
                continue;
            };

            color_material.color =
                origin_color.lerp_that(Color::linear_rgb(strength, strength, strength), transition);
        }
    }
}

fn cue_out_shadow_vfx(
    q_shadows: Query<(&VfxCueOutTimer, &ShadowAbilityConfig, &OriginColors)>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (cue_out, config, origin_colors) in q_shadows.iter() {
        let strength = config.ability().strength;
        let transition = ease::cubic::ease_in_out(
            1.0 - (cue_out.elapsed_secs() / config.cue_out_duration).min(1.0),
        );

        for (entity, origin_color) in origin_colors.iter() {
            let Some(color_material) = q_color_materials
                .get(*entity)
                .ok()
                .and_then(|handle| color_materials.get_mut(handle))
            else {
                continue;
            };

            color_material.color =
                origin_color.lerp_that(Color::linear_rgb(strength, strength, strength), transition);
        }
    }
}

fn cue_out_finished_shadow_vfx(
    trigger: Trigger<AutoTimerFinished<VfxCueOut>>,
    q_shadows: Query<&OriginColors>,
    q_color_materials: Query<&Handle<ColorMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(origin_colors) = q_shadows.get(trigger.entity()) {
        for (entity, origin_color) in origin_colors.iter() {
            let Some(color_material) = q_color_materials
                .get(*entity)
                .ok()
                .and_then(|handle| color_materials.get_mut(handle))
            else {
                continue;
            };

            color_material.color = *origin_color;
        }
    }
}

// HEAL
// ==============================
fn heal_vfx(app: &mut App) {
    app.add_systems(
        Update,
        (
            init_heal_vfx,
            update_heal_vfx,
            cue_in_heal_vfx,
            cue_out_heal_vfx,
        ),
    )
    .observe(cue_out_finished_heal_vfx);
}

fn init_heal_vfx(
    mut commands: Commands,
    q_spaceships: Query<(&HealAbilityConfig, Entity), Added<BlueprintInstanceReady>>,
    mut meshes: ResMut<Assets<Mesh>>,
    prepass_texture: Res<MainPrepassTexture>,
    color_palette: Res<ColorPalette>,
) {
    for (config, entity) in q_spaceships.iter() {
        let mesh_handle =
            Mesh2dHandle(meshes.add(Rectangle::from_length(config.ability().radius * 4.0)));

        let vfx_entity = commands
            .spawn((
                ColorMesh2dBundle {
                    mesh: mesh_handle,
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                HealAbilityMaterial {
                    color0: color_palette.green.to_linear() * 2.0,
                    color1: color_palette.blue.to_linear() * 2.0,
                    time: 0.0,
                    screen_texture: prepass_texture.image_handle().clone_weak(),
                    camera_scale: 1.0,
                },
                RenderLayers::layer(2),
            ))
            .set_parent(entity)
            .id();

        commands.entity(entity).insert(HealVfx {
            entity: vfx_entity,
            time: 0.0,
        });
    }
}

fn update_heal_vfx(
    mut q_spaceships: Query<&HealVfx, Changed<HealVfx>>,
    mut q_materials: Query<&mut HealAbilityMaterial>,
) {
    for vfx in q_spaceships.iter_mut() {
        if let Ok(mut material) = q_materials.get_mut(vfx.entity) {
            material.time = ease::quint::ease_out(vfx.time);
        }
    }
}
fn cue_in_heal_vfx(mut q_spaceships: Query<(&mut HealVfx, &VfxCueInTimer, &HealAbilityConfig)>) {
    for (mut vfx, cue_in, config) in q_spaceships.iter_mut() {
        vfx.time = (cue_in.elapsed_secs() / config.cue_in_duration).min(1.0);
    }
}

fn cue_out_heal_vfx(mut q_spaceships: Query<(&mut HealVfx, &VfxCueOutTimer, &HealAbilityConfig)>) {
    for (mut vfx, cue_out, config) in q_spaceships.iter_mut() {
        vfx.time = 1.0 - (cue_out.elapsed_secs() / config.cue_out_duration).min(1.0);
    }
}

fn cue_out_finished_heal_vfx(
    trigger: Trigger<AutoTimerFinished<VfxCueOut>>,
    mut q_spaceships: Query<&mut HealVfx>,
) {
    if let Ok(mut vfx) = q_spaceships.get_mut(trigger.entity()) {
        vfx.time = 0.0;
    }
}

/// Vfx stats for the [`HealAbilityConfig`],
#[derive(Component, Debug, Clone, Copy)]
pub struct HealVfx {
    entity: Entity,
    time: f32,
}

/// Automatically cue in and out animation [timer][AutoTimer] depending on
/// the insertion of [`AbilityVfxIn`] and [`AbilityVfxOut`].
struct VfxCueTimerPlugin<Cue: VfxCue>(PhantomData<Cue>);

impl<Cue: VfxCue> Plugin for VfxCueTimerPlugin<Cue> {
    fn build(&self, app: &mut App) {
        // Timer for cue in and out (only need to be added once).
        if app.is_plugin_added::<AutoTimerPlugin<VfxCueIn>>() == false {
            app.add_plugins((
                AutoTimerPlugin::<VfxCueIn>::default(),
                AutoTimerPlugin::<VfxCueOut>::default(),
            ));
        }

        app.observe(cue_in::<Cue>).observe(cue_out::<Cue>);
    }
}

impl<Cue: VfxCue> Default for VfxCueTimerPlugin<Cue> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

fn cue_in<Cue: VfxCue>(
    trigger: Trigger<OnAdd, AbilityActive>,
    mut commands: Commands,
    q_cues: Query<&Cue>,
) {
    let entity = trigger.entity();
    if let Ok(cue) = q_cues.get(entity) {
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.start_auto_timer::<VfxCueIn>(Duration::from_secs_f32(cue.cue_in_duration()));
        }
    }
}

fn cue_out<Cue: VfxCue>(
    trigger: Trigger<OnRemove, AbilityActive>,
    mut commands: Commands,
    q_cues: Query<&Cue>,
) {
    let entity = trigger.entity();
    if let Ok(cue) = q_cues.get(entity) {
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.start_auto_timer::<VfxCueOut>(Duration::from_secs_f32(cue.cue_out_duration()))
                // Stop the cue in vfx animation before playing the cue out.
                .remove::<VfxCueInTimer>();
        }
    }
}

pub trait VfxCue: Component {
    /// Duration for vfx to animate in.
    fn cue_in_duration(&self) -> f32;

    /// Duration for vfx to animate out.
    fn cue_out_duration(&self) -> f32;
}

impl<T: ThreadSafe> VfxCue for AbilityConfig<T> {
    fn cue_in_duration(&self) -> f32 {
        self.cue_in_duration
    }

    fn cue_out_duration(&self) -> f32 {
        self.cue_out_duration
    }
}

pub type VfxCueInTimer = AutoTimer<VfxCueIn>;
pub type VfxCueOutTimer = AutoTimer<VfxCueOut>;

/// Marker for [`VfxCueInTimer`].
pub struct VfxCueIn;

/// Marker for [`VfxCueOutTimer`].
pub struct VfxCueOut;
