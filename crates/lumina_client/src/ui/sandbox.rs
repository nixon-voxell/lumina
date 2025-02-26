use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use super::Screen;

pub(super) struct SandboxUiPlugin;

impl Plugin for SandboxUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageEvent<LobbyData>>()
            .register_typst_asset::<SandboxUi>()
            .compile_typst_func::<SandboxUi, SandboxFunc>()
            .init_resource::<SandboxFunc>()
            .add_systems(
                Update,
                (
                    push_to_main_window::<SandboxFunc>(),
                    interactable_func::<SandboxFunc>,
                    exit_sandbox_btn,
                )
                    .run_if(in_state(Screen::Sandbox)),
            )
            .add_systems(Update, handle_sandbox_data);
    }
}

fn exit_sandbox_btn(
    interactions: InteractionQuery,
    mut connection_manager: ResMut<ConnectionManager>,
    mut next_screen_state: ResMut<NextState<Screen>>,
) {
    if interactions.pressed("btn:exit-sandbox") {
        let _ = connection_manager.send_message::<OrdReliableChannel, _>(&ExitLobby);
        next_screen_state.set(Screen::LocalLobby);
    }
}

fn handle_sandbox_data(
    mut lobby_data_evr: EventReader<MessageEvent<LobbyData>>,
    mut sandbox_func: ResMut<SandboxFunc>,
) {
    for event in lobby_data_evr.read() {
        let data = event.message();
        sandbox_func.room_id = Some(data.room_id.0);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "sandbox", layer = 1)]
pub(super) struct SandboxFunc {
    hovered_button: Option<TypLabel>,
    hovered_animation: f64,
    pub room_id: Option<u64>,
}

impl InteractableFunc for SandboxFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/sandbox.typ"]
struct SandboxUi;
