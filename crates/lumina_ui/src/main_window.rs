use bevy::prelude::*;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::window::PrimaryWindow;
use bevy_motiongfx::prelude::*;
use velyst::prelude::*;
use velyst::renderer::compile_typst_func;
use velyst::typst_element::prelude::*;

pub const WINDOW_FADE_DURATION: f32 = 1.0;

pub(super) struct MainWindowUiPlugin;

impl Plugin for MainWindowUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                MainWindowSet::Background,
                MainWindowSet::Default,
                MainWindowSet::Foreground,
            )
                .chain()
                // Makes sure that transform sensitive updates are in sync.
                .after(propagate_transforms)
                .after(sync_simple_transforms),
        );

        app.register_typst_asset::<MainWindowUi>()
            .compile_typst_func::<MainWindowUi, MainWindowFunc>()
            .render_typst_func::<MainWindowFunc>()
            .init_resource::<MainWindowFunc>()
            .add_event::<MainWindowTransparency>()
            .add_systems(Last, clear_main_window_body)
            .add_systems(Update, (main_window_width_height, fade_transparency));
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
    mut evr_transparency: EventReader<MainWindowTransparency>,
    time: Res<Time>,
    mut func: ResMut<MainWindowFunc>,
    mut animate_time: Local<f32>,
    mut start_transparency: Local<f32>,
    mut target_transparency: Local<f32>,
) {
    for transparency in evr_transparency.read() {
        *target_transparency = **transparency;
        *animate_time = 0.0;
        *start_transparency = func.transparency as f32;
    }

    func.transparency = f32::lerp(
        *start_transparency,
        *target_transparency,
        ease::cubic::ease_in_out(*animate_time / WINDOW_FADE_DURATION),
    ) as f64;

    *animate_time = f32::min(*animate_time + time.delta_seconds(), WINDOW_FADE_DURATION);
}

/// Clear the main window and refresh every frame.
fn clear_main_window_body(mut func: ResMut<MainWindowFunc>) {
    func.body.clear();
}

/// 1.0 for full transparency and 0.0 for full opaque.
#[derive(Event, Default, Debug, Deref, DerefMut)]
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

/// App extension for ordering and pushing content to MainWindowFunc.
pub trait MainWindowAppExt {
    fn push_to_main_window<Path: TypstPath, Func: TypstFunc, M>(
        &mut self,
        window_set: MainWindowSet,
        condition: impl Condition<M>,
    ) -> &mut Self;
}

impl MainWindowAppExt for App {
    /// Push [`TypstContent`] to the main window.
    ///
    /// The [`TypstContent`] will only be pushed:
    /// - After the [`TypstFunc`] is compiled.
    /// - Before the [`MainWindowFunc`] is compiled.
    fn push_to_main_window<Path: TypstPath, Func: TypstFunc, M>(
        &mut self,
        window_set: MainWindowSet,
        condition: impl Condition<M>,
    ) -> &mut Self {
        self.add_systems(
            PostUpdate,
            push_to_main_window_impl::<Func>
                .after(compile_typst_func::<Path, Func>)
                .before(compile_typst_func::<MainWindowUi, MainWindowFunc>)
                .in_set(window_set)
                .run_if(condition),
        )
    }
}

/// Push content of a typst function to the main window.
fn push_to_main_window_impl<F: TypstFunc>(
    content: Res<TypstContent<F>>,
    mut main_func: ResMut<MainWindowFunc>,
) {
    main_func.body.push(content.clone());
}

pub fn always_run() -> bool {
    true
}

#[derive(SystemSet, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainWindowSet {
    Background,
    #[default]
    Default,
    Foreground,
}
