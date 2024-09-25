use bevy::prelude::*;

use super::velocity::Velocity;

pub struct AccelerationPlugin;

/// Acceleration is rate of change of velocity with respect to time.
#[derive(Component)]
pub struct Acceleration {
    velocity: f32,
    time: f32,
}

impl Acceleration {
    pub fn new(velocity: f32, time: f32) -> Self {
        Self { velocity, time }
    }

    pub fn acceleration(&self) -> f32 {
        self.velocity / self.time
    }
}
