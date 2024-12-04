// TODO: Return to main menu button.

use bevy::prelude::*;
use lumina_shared::prelude::TeamType;
use lumina_ui::prelude::*;
use strum::IntoEnumIterator;
// use strum
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

use crate::player::{CachedGameStat, GameStat};

use super::Screen;

pub(super) struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameOverUi>()
            .compile_typst_func::<GameOverUi, GameOverFunc>()
            .init_resource::<GameOverFunc>()
            .add_systems(OnEnter(Screen::GameOver), set_game_over_values)
            .add_systems(
                Update,
                (
                    push_to_main_window::<GameOverFunc>(),
                    interactable_func::<GameOverFunc>,
                    main_menu_btn,
                    set_game_over_values.run_if(resource_changed::<CachedGameStat>),
                )
                    .run_if(in_state(Screen::GameOver)),
            );
    }
}

fn set_game_over_values(game_stat: Res<CachedGameStat>, mut func: ResMut<GameOverFunc>) {
    if let CachedGameStat(GameStat {
        team_type: Some(team_type),
        game_score: Some(game_score),
    }) = *game_stat
    {
        func.local_team_index = team_type as u8;

        func.team_names = TeamType::iter().map(|t| t.into()).collect();
        func.team_scores = game_score.scores.to_vec();
    }
}

fn main_menu_btn(interactions: InteractionQuery, mut next_screen_state: ResMut<NextState<Screen>>) {
    if interactions.pressed("btn:main-menu") {
        next_screen_state.set(Screen::MainMenu);
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "game_over", layer = 1)]
pub struct GameOverFunc {
    hovered_button: Option<TypLabel>,
    hovered_animation: f64,
    pub local_team_index: u8,
    pub team_names: Vec<&'static str>,
    pub team_scores: Vec<u8>,
}

impl InteractableFunc for GameOverFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_over.typ"]
struct GameOverUi;
