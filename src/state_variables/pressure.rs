use bevy::prelude::*;

use crate::constants::STANDARD_ATMOSPHERE_PRESSURE;

pub struct PressurePlugin;

impl Plugin for PressurePlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

/// Generic pressure component (no real particular design or setup).
///
/// Attaches to a component which requires calculations of pressure.
/// Allows for default pressure (atmosphere pressure) to be quickly referenced.
#[derive(Component, Debug)]
pub struct Pressure(f32);

impl Pressure {
    /// Units in Pascals. 1 Pascal = 1 newton per square metre. 1 atm = 101,325 Pascals. [Pascal](https://en.wikipedia.org/wiki/Pascal_(unit))
    pub fn new(pressure: f32) -> Self {
        Self(pressure)
    }

    /// Helper function for returning pressure value.
    pub fn pressure(&self) -> f32 {
        self.0
    }
}

impl Default for Pressure {
    /// Atmosphere pressure.
    /// 1 atm = 101,325 Pascals (101.325 MPa). "Mean sea-level pressure".
    fn default() -> Self {
        Self(STANDARD_ATMOSPHERE_PRESSURE)
    }
}
