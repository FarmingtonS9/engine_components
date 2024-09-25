use bevy::prelude::*;

// Constants
use crate::constants::STANDARD_KILOGRAM;

/// Amount of substance in an object. Intrinsic property.
///
/// Measured in kilograms (kg).
///
/// 1 kg (100g) is the standard.
#[derive(Component)]
pub struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }
    pub fn mass(&self) -> f32 {
        self.0
    }
}

impl Default for Mass {
    fn default() -> Self {
        Self(STANDARD_KILOGRAM)
    }
}
