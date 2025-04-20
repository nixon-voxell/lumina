use bevy::prelude::*;
use lumina_common::prelude::*;
use lumina_shared::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;

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
    q_spaceships: Query<(Entity, Has<Dead>), With<SourceEntity>>,
    local_player_info: LocalPlayerInfo,
    mut timer: Local<Option<f32>>,
) {
    // Try to get the local player's spaceship entity
    let Some(spaceship_entity) = local_player_info.get(PlayerInfoType::Spaceship) else {
        debug!("No spaceship entity found for local player");
        func.is_dead = false;
        func.elapsed_time = 0.0;
        func.remaining_time = 5.0;
        func.percentage = 0.0;
        *timer = None;
        return;
    };

    // Check if spaceship is dead
    if let Ok((_, is_dead)) = q_spaceships.get(spaceship_entity) {
        if is_dead {
            // If newly dead, initialize the timer
            if timer.is_none() {
                *timer = Some(0.0);
                info!("Respawn cue started for local player");
            }

            // Update the timer
            if let Some(current_time) = timer.as_mut() {
                *current_time += time.delta_seconds();

                let total_time = 5.0;
                let elapsed = *current_time;
                let remaining = (total_time - elapsed).max(0.0);
                let percentage = (elapsed / total_time).clamp(0.0, 1.0);

                func.is_dead = true;
                func.elapsed_time = elapsed as f64;
                func.remaining_time = remaining as f64;
                func.percentage = percentage as f64;
            }
        } else {
            // Reset if alive
            if func.is_dead {
                info!("Respawn cue hidden for local player");
            }
            func.is_dead = false;
            func.elapsed_time = 0.0;
            func.remaining_time = 5.0;
            func.percentage = 0.0;
            *timer = None;
        }
    } else {
        // Reset if the spaceship entity is not found
        debug!("Failed to query spaceship entity {:?}", spaceship_entity);
        func.is_dead = false;
        func.elapsed_time = 0.0;
        func.remaining_time = 5.0;
        func.percentage = 0.0;
        *timer = None;
    }

    func.dummy_update = func.dummy_update.wrapping_add(1);
}

#[derive(TypstFunc, Resource)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    is_dead: bool,
    elapsed_time: f64,
    remaining_time: f64,
    percentage: f64,
    dummy_update: u8,
}

impl Default for MainFunc {
    fn default() -> Self {
        Self {
            is_dead: false,
            elapsed_time: 0.0,
            remaining_time: 5.0,
            percentage: 0.0,
            dummy_update: 0,
        }
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/respawn_cue.typ"]
struct RespawnCue;
