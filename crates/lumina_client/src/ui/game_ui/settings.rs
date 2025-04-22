use bevy::prelude::*;
use lumina_ui::prelude::*;
use lumina_shared::prelude::*;
use lumina_common::prelude::*;
use velyst::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct SettingsUiPlugin;

use crate::screens::Screen;
use crate::audio::{Background, SoundFx};
use crate::player::LocalPlayerInfo;

#[derive(Resource, Default)]
pub struct DraggingState {
    pub is_dragging: bool,
    pub initial_mouse_x: f32,
    pub slider_entity: Option<Entity>,
}

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<SettingsUi>()
            .compile_typst_func::<SettingsUi, SettingsFunc>()
            .push_to_main_window::<SettingsUi, SettingsFunc, _>(
                MainWindowSet::Foreground,
                |overlay: Res<SettingsOverlay>| overlay.visible, // Render only if visible
            )
            .init_resource::<SettingsFunc>()
            .init_resource::<DraggingState>()
            .add_systems(
                Update,
                (adjust_bgm, adjust_vfx, close_settings, leave_btn),
            );
    }
}

fn leave_btn(
    interactions: InteractionQuery,
    mut next_screen_state: ResMut<NextState<Screen>>,
    mut overlay: ResMut<SettingsOverlay>,
) {
    if interactions.pressed("btn:leave") {
        overlay.visible = false;
        next_screen_state.set(Screen::MainMenu);
    }
}

fn close_settings(
    interactions: InteractionQuery,
    mut overlay: ResMut<SettingsOverlay>) {
    if interactions.pressed("btn:close") {
        overlay.visible = false;
    }
}

fn adjust_bgm(
    interactions: InteractionQuery,
    mut dragging_state: ResMut<DraggingState>,
    bgm_channel: Res<AudioChannel<Background>>,
    q_windows: Query<&Window>,
    local_player_info: LocalPlayerInfo,
    q_actions: Query<&ActionState<PlayerAction>, With<SourceEntity>>,
) {
    let Some(action) = local_player_info
        .get(PlayerInfoType::Action)
        .and_then(|e| q_actions.get(e).ok())
    else {
        return;
    };

    if action.pressed(&PlayerAction::Attack) {
        if let Ok(window) = q_windows.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                dragging_state.is_dragging = true;
                dragging_state.initial_mouse_x = cursor_position.x;
            }
        }
    }

    if dragging_state.is_dragging && interactions.pressed("btn:bgm") {
        if let Ok(window) = q_windows.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                let delta_x = cursor_position.x - dragging_state.initial_mouse_x;

                // Map delta_x to a volume range (e.g., 0.0 to 1.0)
                let volume = (delta_x / window.width() * 10.0).clamp(0.0, 1.0);
                println!("BGM Volume: {}", delta_x);
                // bgm_channel.set_volume(volume);
            }
        }
    }
}

fn adjust_vfx(
    interactions: InteractionQuery,
    channel: Res<AudioChannel<SoundFx>>,
    mut overlay: ResMut<SettingsOverlay>,
) {
    if interactions.pressed("btn:vfx") {
        channel.stop();
        // let current_volume = channel.volume();
        // if current_volume > 0.0 {
        //     channel.set_volume(0.0); // Mute
        // } else {
        //     channel.set_volume(0.5); // Restore default volume
        // }
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "settings", layer = 1)]
pub struct SettingsFunc;

#[derive(TypstPath)]
#[typst_path = "typst/client/settings.typ"]
pub struct SettingsUi;

#[derive(Resource, Default)]
pub struct SettingsOverlay {
    pub visible: bool,
}
