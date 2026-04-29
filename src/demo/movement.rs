//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (apply_movement)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_systems(
        FixedUpdate,
        (apply_screen_wrap)
            .chain()
            .in_set(AppSystems::PostUpdate)
            .in_set(PausableSystems),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut MovementController)>,
) {
    let dt = time.delta_secs();

    for (mut controller, movement) in &mut query {
        controller.translation = Some(movement.intent * movement.max_speed * dt);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut KinematicCharacterController), With<ScreenWrap>>,
) {
    let size = window.size();
    let half = size / 2.0;

    for (transform, mut controller) in &mut query {
        let pos = transform.translation.xy();
        let mut wrapped = pos;

        if pos.x > half.x {
            wrapped.x = -half.x;
        } else if pos.x < -half.x {
            wrapped.x = half.x;
        }

        if pos.y > half.y {
            wrapped.y = -half.y;
        } else if pos.y < -half.y {
            wrapped.y = half.y;
        }

        if wrapped != pos {
            let correction = wrapped - pos;

            // 👇 KEY CHANGE: accumulate instead of overwrite
            let current = controller.translation.unwrap_or(Vec2::ZERO);
            controller.translation = Some(current + correction);
        }
    }
}
