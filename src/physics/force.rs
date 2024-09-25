use bevy::prelude::*;

use super::{acceleration::Acceleration, mass::Mass};

pub struct ForcePlugin;

#[derive(Bundle)]
struct ForceBundle {
    mass: Mass,
    acceleration: Acceleration,
}
