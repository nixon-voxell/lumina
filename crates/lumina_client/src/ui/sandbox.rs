use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use super::Screen;

pub(super) struct SandboxUiPlugin;

impl Plugin for SandboxUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageEvent<LobbyData>>()
            .register_typst_asset::<SandboxUi>()
            .compile_typst_func::<SandboxUi, SandboxFunc>()
            .push_to_main_window::<SandboxUi, SandboxFunc, _>(
                MainWindowSet::Default,
                in_state(Screen::Sandbox),
            )
            .recompile_on_interaction::<SandboxFunc>(|func| &mut func.dummy_update)
            .init_resource::<SandboxFunc>()
            .add_systems(Update, exit_sandbox_btn.run_if(in_state(Screen::Sandbox)))
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
    mut evr_lobby_data: EventReader<MessageEvent<LobbyData>>,
    mut sandbox_func: ResMut<SandboxFunc>,
) {
    for event in evr_lobby_data.read() {
        let data = event.message();
        sandbox_func.room_id = Some(data.room_id.0);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "sandbox", layer = 1)]
pub(super) struct SandboxFunc {
    pub room_id: Option<u64>,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/sandbox.typ"]
struct SandboxUi;
