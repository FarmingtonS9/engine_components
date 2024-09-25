use bevy::prelude::*;

pub struct DimensionPlugin;

impl Plugin for DimensionPlugin {
    fn build(&self, app: &mut App) {
        todo!();
    }
}

#[derive(Component)]
pub struct Dimension(Vec2);

impl Dimension {
    pub fn new(dimension: Vec2) -> Self {
        Self(dimension)
    }

    /// Calculates the internal volume of the firebox.
    pub fn volume(&self) -> f32 {
        let length = self.dim_x();
        let height = self.dim_y();
        length * height
    }

    /// Helper function for returning x dimension.
    pub fn dim_x(&self) -> f32 {
        self.0.x
    }

    /// Helper function for returning y dimension.
    pub fn dim_y(&self) -> f32 {
        self.0.y
    }
}
