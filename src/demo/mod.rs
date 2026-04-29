//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
pub mod hud;
pub mod level;
pub mod loot;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
        animation::plugin,
        level::plugin,
        loot::plugin,
        movement::plugin,
        player::plugin,
    ));
}
