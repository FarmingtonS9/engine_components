use std::ops::Sub;

use bevy::prelude::*;

/// Displacement is the change in [Position] of an object.
///
/// Vector quantity; contains a direction and magnitude.
#[derive(Component)]
pub struct Displacement {
    //Note for future: f32 works for 1-dimension. 2-dimension will need a Vec2 and so-on.
    final_position: Vec2,
    initial_position: Vec2,
}

impl Displacement {
    pub fn new(final_position: Vec2, initial_position: Vec2) -> Self {
        Self {
            final_position,
            initial_position,
        }
    }

    pub fn displacement(&self) -> Vec2 {
        Vec2::new(
            self.final_position.x - self.initial_position.x,
            self.final_position.y - self.initial_position.y,
        )
    }
}
