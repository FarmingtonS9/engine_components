use bevy::prelude::*;

#[derive(Component)]
pub struct Distance {
    final_position: f32,
    initial_position: f32,
}

impl Distance {
    pub fn new(final_position: f32, initial_position: f32) -> Self {
        Self {
            final_position,
            initial_position,
        }
    }

    /// Calculcates and returns the difference in distance between the initial position and final position.
    pub fn distance(&self) -> f32 {
        self.final_position - self.initial_position
    }
}
