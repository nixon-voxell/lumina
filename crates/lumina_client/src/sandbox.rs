use bevy::prelude::*;
use blenvy::*;
use client::*;
use lightyear::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use crate::player::LocalPlayerId;
use crate::ui::Screen;
use crate::LocalClientId;

pub(super) struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_enter_sandbox.run_if(in_state(Screen::Sandbox)),
        );
    }
}

fn handle_enter_sandbox(
    mut commands: Commands,
    mut sandbox_evr: EventReader<MessageEvent<EnterSandbox>>,
    mut transparency_evw: EventWriter<MainWindowTransparency>,
    local_client_id: Res<LocalClientId>,
    mut local_player_id: ResMut<LocalPlayerId>,
) {
    for _ in sandbox_evr.read() {
        // Set local player id to the networked version of player id.
        **local_player_id = PlayerId(**local_client_id);
        transparency_evw.send(MainWindowTransparency(1.0));

        commands.spawn((LobbyType::Sandbox.info(), SpawnBlueprint));
    }
}
