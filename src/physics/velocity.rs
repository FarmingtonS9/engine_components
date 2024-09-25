use bevy::prelude::*;

/// Velocity is the [Displacement] of the object over time.
///
/// Displacement is a vector quantity; it has a direction and magnitude.
#[derive(Component)]
pub struct Velocity {
    displacement: Vec2,
    time: f32,
}

impl Velocity {
    pub fn new(displacement: Vec2, time: f32) -> Self {
        Self { displacement, time }
    }

    pub fn velocity(&self) -> Vec2 {
        Vec2::new(
            self.displacement.x / self.time,
            self.displacement.y / self.time,
        )
    }
}
