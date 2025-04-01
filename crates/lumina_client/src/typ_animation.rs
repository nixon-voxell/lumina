use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_motiongfx::motiongfx_core::UpdateSequenceSet;
use bevy_motiongfx::prelude::*;
use bevy_vello::vello::kurbo;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;
use velyst::typst_vello;

pub struct TypAnimationPlugin<T: TypstFunc>(PhantomData<T>);

impl<T: TypstFunc> Plugin for TypAnimationPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_component::<LabelScaleFade, f32>.in_set(UpdateSequenceSet),
                animate_label::<T>,
            ),
        );
    }
}

impl<T: TypstFunc> Default for TypAnimationPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// WARN: LabelScaleFade animates across functions, which means it
// doesn't care which function the label belongs to.
fn animate_label<T: TypstFunc>(
    q_labels: Query<&LabelScaleFade, Changed<LabelScaleFade>>,
    mut scene: ResMut<VelystScene<T>>,
) {
    for anim_label in q_labels.iter() {
        scene.post_process_map.insert(
            anim_label.label,
            typst_vello::PostProcess {
                transform: Some(kurbo::Affine::scale(anim_label.scale() as f64)),
                layer: Some(typst_vello::Layer {
                    alpha: anim_label.fade(),
                    ..default()
                }),
                ..default()
            },
        );
    }
}

/// Scale and fade the labelled content via time.
#[derive(Component, Clone, Copy)]
pub struct LabelScaleFade {
    pub label: TypLabel,
    /// Animation time between min and max value.
    pub time: f32,
    /// Min and max scale.
    pub scale: Range,
    /// Min and max fade.
    pub fade: Range,
}

impl LabelScaleFade {
    pub fn new(name: &str) -> Self {
        Self {
            label: TypLabel::construct(name.into()),
            time: 0.0,
            scale: Range(0.8, 1.0),
            fade: Range(0.0, 1.0),
        }
    }

    pub fn fade(&self) -> f32 {
        self.fade.lerp(self.time)
    }

    pub fn scale(&self) -> f32 {
        self.scale.lerp(self.time)
    }
}

#[derive(Clone, Copy)]
pub struct Range(pub f32, pub f32);

impl Range {
    pub fn lerp(&self, t: f32) -> f32 {
        self.0.lerp(self.1, t)
    }
}
