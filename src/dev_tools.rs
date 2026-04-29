//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

use bevy_rapier2d::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins((RapierDebugRenderPlugin {
        enabled: false,
        ..default()
    },));

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (toggle_debug_ui, toggle_debug_render).run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_debug_render(mut debug_context: ResMut<DebugRenderContext>) {
    debug_context.enabled = !debug_context.enabled;
}
