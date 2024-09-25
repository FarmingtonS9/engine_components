use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    distance: f32,
    time: f32,
}

impl Speed {
    pub fn new(distance: f32, time: f32) -> Self {
        Self { distance, time }
    }

    pub fn speed(&self) -> f32 {
        self.distance / self.time
    }
}
