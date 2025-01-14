use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;

use crate::ui::Screen;

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
    // mut commands: Commands,
    mut sandbox_evr: EventReader<MessageEvent<EnterSandbox>>,
    mut transparency_evw: EventWriter<MainWindowTransparency>,
) {
    for _ in sandbox_evr.read() {
        println!("\n\nClient: enter sandbox.");
        transparency_evw.send(MainWindowTransparency(1.0));
    }
}
