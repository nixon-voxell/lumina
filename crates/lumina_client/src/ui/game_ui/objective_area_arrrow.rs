use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};

use crate::camera::GameCamera;
use crate::player::LocalPlayerInfo;
use crate::ui::Screen;

pub(super) struct ObjectiveArrowUiPlugin;

impl Plugin for ObjectiveArrowUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<ObjectiveArrowUi>()
            .compile_typst_func::<ObjectiveArrowUi, MainFunc>()
            .push_to_main_window::<ObjectiveArrowUi, MainFunc, _>(
                MainWindowSet::Foreground,
                in_state(Screen::InGame),
            )
            .init_resource::<MainFunc>()
            .init_resource::<TargetPosition>()
            .add_systems(OnEnter(Screen::LocalLobby), reset_target_position)
            .add_systems(Update, update_taget_position)
            .add_systems(
                PostUpdate,
                update_arrow
                    .in_set(MainWindowTransformSyncSet)
                    .run_if(in_state(Screen::InGame)),
            );
    }
}

/// Update the [`MainFunc`].
fn update_arrow(
    q_global_transforms: Query<&GlobalTransform>,
    q_game_camera: Query<(&GlobalTransform, &OrthographicProjection), With<GameCamera>>,
    mut func: ResMut<MainFunc>,
    local_player_info: LocalPlayerInfo,
    target_position: Res<TargetPosition>,
    time: Res<Time>,
    mut transparency: Local<f64>,
) {
    const MIN_DIST: f32 = 1200.0;
    const FADE_SPEED: f64 = 4.0;

    let fade_delta = time.delta_seconds_f64() * FADE_SPEED;
    let TargetPosition(Some(target_position)) = *target_position else {
        return;
    };

    let Ok((camera_transform, projection)) = q_game_camera.get_single() else {
        return;
    };

    if let Some(spaceship_transform) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_global_transforms.get(e).ok())
    {
        let spaceship_position = spaceship_transform.translation().xy();

        let diff = target_position - spaceship_position;
        let dist = diff.length();
        let direction = diff.normalize_or_zero().as_dvec2();

        if dist > MIN_DIST {
            *transparency = transparency.lerp(0.0, fade_delta);
        } else {
            *transparency = transparency.lerp(1.0, fade_delta);
        }

        let camera_diff = (camera_transform.translation().xy() - spaceship_position).as_dvec2();

        func.data = dict! {
            "rotation" => direction.y.atan2(direction.x),
            "dist" => dist as f64,
            "transparency" => *transparency,
            "scale" => projection.scale as f64,
            "camera_diff_x" => camera_diff.x,
            "camera_diff_y" => camera_diff.y,
        };
    }
}

fn update_taget_position(
    mut evr_position: EventReader<MessageEvent<ObjectivePosition>>,
    mut target_position: ResMut<TargetPosition>,
) {
    for event in evr_position.read() {
        let ObjectivePosition(position) = event.message;
        **target_position = Some(position);
        info!("Got objective position: {}", position);
    }
}

fn reset_target_position(mut target_position: ResMut<TargetPosition>) {
    **target_position = None;
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main")]
pub struct MainFunc {
    pub data: Dict,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct TargetPosition(Option<Vec2>);

#[derive(TypstPath)]
#[typst_path = "typst/client/objective_area_arrow.typ"]
pub struct ObjectiveArrowUi;
