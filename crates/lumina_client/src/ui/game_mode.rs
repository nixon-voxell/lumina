use bevy::prelude::*;
use lumina_ui::prelude::*;
use velyst::prelude::*;
use velyst::typst::foundations;
use velyst::typst_element::prelude::*;

pub(super) struct GameModeUiPlugin;

impl Plugin for GameModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_typst_asset::<GameMode>()
            .compile_typst_func::<GameMode, MainFunc>()
            .init_resource::<MainFunc>()
            .add_systems(
                Update,
                (
                    push_to_main_window::<MainFunc>(),
                    interactable_func::<MainFunc>,
                ),
            );
    }
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main", layer = 1)]
struct MainFunc {
    data: foundations::Dict,
    #[typst_func(named)]
    hovered_button: Option<TypLabel>,
    #[typst_func(named)]
    hovered_animation: f64,
}

impl InteractableFunc for MainFunc {
    fn hovered_button(&mut self, hovered_button: Option<TypLabel>, hovered_animation: f64) {
        self.hovered_button = hovered_button;
        self.hovered_animation = hovered_animation;
    }
}

#[derive(TypstPath)]
#[typst_path = "typst/client/game_mode.typ"]
struct GameMode;
