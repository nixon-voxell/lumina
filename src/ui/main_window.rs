use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

pub(super) struct MainWindowUiPlugin;

impl Plugin for MainWindowUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MainWindowSet::First,
                MainWindowSet::Main,
                MainWindowSet::Last,
            )
                .chain(),
        );

        app.register_typst_asset::<MainWindowUi>()
            .compile_typst_func::<MainWindowUi, MainWindowFunc>()
            .render_typst_func::<MainWindowFunc>()
            .init_resource::<MainWindowFunc>()
            .add_systems(
                Update,
                (
                    main_window_width_height,
                    clear_main_window_body.after(VelystSet::Render),
                ),
            );
    }
}

/// Set window width and height if there are any changes.
fn main_window_width_height(
    q_window: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut func: ResMut<MainWindowFunc>,
) {
    let Ok(window) = q_window.get_single() else {
        return;
    };

    func.width = window.width() as f64;
    func.height = window.height() as f64;
}

/// Cleared and refresh every frame.
fn clear_main_window_body(mut func: ResMut<MainWindowFunc>) {
    func.body.clear();
}

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main_window", layer = 1)]
pub struct MainWindowFunc {
    width: f64,
    height: f64,
    pub body: Vec<Content>,
}

#[derive(TypstPath)]
#[typst_path = "typst/main_window.typ"]
struct MainWindowUi;

/// Helper for ordering and pushing content to MainWindowFunc.

pub fn push_to_main_window_first<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::First)
}

pub fn push_to_main_window<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::Main)
}

pub fn push_to_main_window_last<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::Last)
}

/// Push content to the main window in no particular order.
fn push_to_main_window_unordered<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_impl::<F>
        .into_configs()
        .before(VelystSet::Compile)
}

/// Push content of a typst function to the main window.
/// NOTE: The content shown will be 1 frame behind.
fn push_to_main_window_impl<F: TypstFunc>(
    content: Res<TypstContent<F>>,
    mut main_func: ResMut<MainWindowFunc>,
) {
    main_func.body.push(content.clone());
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainWindowSet {
    First,
    Main,
    Last,
}
