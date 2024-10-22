use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_motiongfx::prelude::*;
use velyst::prelude::*;
use velyst::typst_element::prelude::*;

pub const WINDOW_FADE_DURATION: f32 = 1.0;

pub(super) struct MainWindowUiPlugin;

impl Plugin for MainWindowUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MainWindowSet::Background,
                MainWindowSet::Default,
                MainWindowSet::Foreground,
            )
                .chain(),
        );

        app.register_typst_asset::<MainWindowUi>()
            .compile_typst_func::<MainWindowUi, MainWindowFunc>()
            .render_typst_func::<MainWindowFunc>()
            .init_resource::<MainWindowFunc>()
            .init_resource::<MainWindowTransparency>()
            .add_systems(
                Update,
                (
                    main_window_width_height,
                    fade_transparency.before(VelystSet::Compile),
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

/// Animate transparency fade in and out based on value from [`MainWindowTransparency`].
fn fade_transparency(
    transparency: Res<MainWindowTransparency>,
    time: Res<Time>,
    mut func: ResMut<MainWindowFunc>,
    mut animate_time: Local<f32>,
    mut start_transparency: Local<f32>,
) {
    if transparency.is_changed() {
        *animate_time = 0.0;
        *start_transparency = func.transparency as f32;
    }

    func.transparency = f32::lerp(
        *start_transparency,
        **transparency,
        ease::cubic::ease_in_out(*animate_time / WINDOW_FADE_DURATION),
    ) as f64;

    *animate_time = f32::min(*animate_time + time.delta_seconds(), WINDOW_FADE_DURATION);
}

/// Cleared and refresh every frame.
fn clear_main_window_body(mut func: ResMut<MainWindowFunc>) {
    func.body.clear();
}

/// 1.0 for full transparency and 0.0 for full opaque.
#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct MainWindowTransparency(pub f32);

#[derive(TypstFunc, Resource, Default)]
#[typst_func(name = "main_window", layer = 1)]
pub struct MainWindowFunc {
    width: f64,
    height: f64,
    pub body: Vec<Content>,
    pub transparency: f64,
}

#[derive(TypstPath)]
#[typst_path = "typst/main_window.typ"]
struct MainWindowUi;

/// Helper for ordering and pushing content to MainWindowFunc.

pub fn _push_to_main_window_background<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::Background)
}

pub fn push_to_main_window<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::Default)
}

pub fn push_to_main_window_foreground<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_unordered::<F>().in_set(MainWindowSet::Foreground)
}

/// Push content to the main window in no particular order.
fn push_to_main_window_unordered<F: TypstFunc>() -> SystemConfigs {
    push_to_main_window_impl::<F>
        .into_configs()
        .before(VelystSet::Compile)
}

/// Push content of a typst function to the main window.
///
/// NOTE: The content shown will be 1 frame behind.
fn push_to_main_window_impl<F: TypstFunc>(
    content: Res<TypstContent<F>>,
    mut main_func: ResMut<MainWindowFunc>,
) {
    main_func.body.push(content.clone());
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainWindowSet {
    Background,
    Default,
    Foreground,
}
