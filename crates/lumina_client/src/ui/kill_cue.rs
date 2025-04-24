use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::screens::Screen;

use super::game_ui::get_name;

pub(super) struct KillCueUiPlugin;

impl Plugin for KillCueUiPlugin {
    fn build(&self, app: &mut App) {
        let run_condition = in_state(Screen::InGame).or_else(in_state(Screen::MultiplayerLobby));

        app.init_resource::<MainFunc>()
            .register_typst_asset::<KillCueUi>()
            .compile_typst_func::<KillCueUi, MainFunc>()
            .push_to_main_window::<KillCueUi, MainFunc, _>(
                MainWindowSet::Foreground,
                run_condition.clone(),
            )
            .add_systems(Update, animate_kill_cue.run_if(run_condition));
    }
}

fn animate_kill_cue(
    mut events: EventReader<MessageEvent<KilledPlayer>>,
    mut func: ResMut<MainFunc>,
    time: Res<Time>,
) {
    const SPEED: f64 = 2.0;
    let delta = time.delta_seconds_f64() * SPEED;

    for event in events.read() {
        let name = get_name(&event.message.0);
        // Assign name and restart animation.
        func.name = name;
        func.animate = 0.0;
    }

    func.animate = func.animate.lerp(1.0, delta);
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    name: &'static str,
    animate: f64,
}

impl Default for MainFunc {
    fn default() -> Self {
        Self {
            name: "",
            animate: 1.0,
        }
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/kill_cue.typ"]
struct KillCueUi;
