use bevy::prelude::*;

/// Position is the object's location at any given time.
///
/// NOTE: Position is better represented by the [Transform] component. Position exists because I was testing the "turtle-all-the-way-down" approach of components and bundles.
#[derive(Component)]
pub struct Position(Vec2);

/// Displacement is the change in [Position]. The difference between the final position and initial position.
///
/// In this 2D world, the change in position is the change in the `x` and `y` axis values.
pub fn displacement(mut position_query: Query<&mut Position>) {
    for mut position in position_query.iter_mut() {}
}

impl Position {
    pub fn new(position: Vec2) -> Self {
        Self(position)
    }
    /// Helper function for returning the `x` position of the object.
    pub fn x_pos(&self) -> f32 {
        self.0.x
    }
    /// Helper function for returning the `y` position of the object.
    pub fn y_pos(&self) -> f32 {
        self.0.y
    }

    /// Converts a Position component to a (default) Transform component. Consumes the Position component.
    pub fn to_transform(self) -> Transform {
        Transform {
            translation: Vec3 {
                x: self.x_pos(),
                y: self.y_pos(),
                ..default()
            },
            ..default()
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}
