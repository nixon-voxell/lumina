use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::game::RESPAWN_DURATION;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations::{dict, Dict};

use crate::player::LocalPlayerInfo;

use super::Screen;

pub(super) struct RespawnCueUiPlugin;

impl Plugin for RespawnCueUiPlugin {
    fn build(&self, app: &mut App) {
        let run_condition = in_state(Screen::InGame).or_else(in_state(Screen::MultiplayerLobby));
        app.init_resource::<MainFunc>()
            .register_typst_asset::<RespawnCue>()
            .compile_typst_func::<RespawnCue, MainFunc>()
            .push_to_main_window::<RespawnCue, MainFunc, _>(
                MainWindowSet::Foreground,
                run_condition.clone(),
            )
            .add_systems(Update, update_respawn_timer.run_if(run_condition));
    }
}

fn update_respawn_timer(
    time: Res<Time>,
    mut func: ResMut<MainFunc>,
    q_spaceships: Query<Ref<Dead>, With<SourceEntity>>,
    local_player_info: LocalPlayerInfo,
    mut countdown: Local<f64>,
) {
    func.data = None;
    // Try to get the local player's spaceship entity.
    let Some(dead) = local_player_info
        .get(PlayerInfoType::Spaceship)
        .and_then(|e| q_spaceships.get(e).ok())
    else {
        return;
    };

    // Just died.
    if dead.is_added() {
        *countdown = RESPAWN_DURATION as f64;
    }

    // Update the timer.
    *countdown -= time.delta_seconds_f64();
    func.data = Some(dict! {
        "countdown" => *countdown,
        "percentage" => *countdown / RESPAWN_DURATION as f64
    });

    func.dummy_update = func.dummy_update.wrapping_add(1);
}

#[derive(TypstFunc, Default, Resource)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    data: Option<Dict>,
    dummy_update: u8,
}

#[derive(TypstPath)]
#[typst_path = "typst/client/respawn_cue.typ"]
struct RespawnCue;
