//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

pub(super) fn log_transition<S: States>(app: &mut App) {
    // Log state transitions in dev builds
    app.add_systems(Update, log_transitions::<S>);
}
