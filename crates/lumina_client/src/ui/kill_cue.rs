use bevy::prelude::*;
use client::*;
use lightyear::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

use crate::{player::LocalPlayerInfo, screens::Screen};

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
            .add_systems(OnEnter(Screen::LocalLobby), reset_ui)
            .add_systems(
                Update,
                (animate_kill_cue, reset_streak).run_if(run_condition),
            );
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
        let message = event.message();
        let name = get_name(&message.killed_id);
        // Assign name and restart animation.
        func.name = name;
        func.animate = 0.0;
        func.streak = message.streak_count;
    }

    func.animate = func.animate.lerp(1.0, delta);
}

/// Reset streak on local spaceship death.
fn reset_streak(
    q_dead: DeadQuery<()>,
    local_player_info: LocalPlayerInfo,
    mut func: ResMut<MainFunc>,
) {
    if let Some(entity) = local_player_info.get(PlayerInfoType::Spaceship) {
        if q_dead.contains(entity) {
            func.streak = 0;
        }
    }
}

fn reset_ui(mut func: ResMut<MainFunc>) {
    *func = MainFunc::default();
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    name: &'static str,
    animate: f64,
    streak: u8,
}

impl Default for MainFunc {
    fn default() -> Self {
        Self {
            name: "",
            animate: 1.0,
            streak: 0,
        }
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/kill_cue.typ"]
struct KillCueUi;
