use bevy::prelude::*;
use velyst::prelude::*;

use super::main_window::push_to_main_window_foreground;
use super::{can_show_content, CanShowContent};

pub(super) struct PerfMetricsUiPlugin;

impl Plugin for PerfMetricsUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<PerfMetricsUi>()
            .compile_typst_func::<PerfMetricsUi, PerfMetricsFunc>()
            .init_resource::<PerfMetricsFunc>()
            .init_resource::<CanShowContent<PerfMetricsFunc>>()
            .add_systems(
                Update,
                (
                    perf_metrics,
                    push_to_main_window_foreground::<PerfMetricsFunc>()
                        .run_if(can_show_content::<PerfMetricsFunc>),
                ),
            );
    }
}

fn perf_metrics(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut perf_func: ResMut<PerfMetricsFunc>,
    mut show: ResMut<CanShowContent<PerfMetricsFunc>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        **show = !**show;
    }

    if **show {
        perf_func.fps = (1.0 / time.delta_seconds_f64() * 100.0).round() / 100.0;
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "perf_metrics", layer = 1)]
pub struct PerfMetricsFunc {
    fps: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/perf_metrics.typ"]
struct PerfMetricsUi;
