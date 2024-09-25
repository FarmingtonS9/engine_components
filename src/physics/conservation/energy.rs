use bevy::prelude::*;

// TODO
// Either it is a bundle or its component. To figure out.
#[derive(Component)]
pub struct Energy {
    kinetic_energy: f32,
    potential_energy: f32,
}

impl Energy {
    pub fn new(kinetic_energy: f32, potential_energy: f32) -> Self {
        Self {
            kinetic_energy,
            potential_energy,
        }
    }

    pub fn energy(&self) -> f32 {
        self.kinetic_energy + self.potential_energy
    }
}

#[derive(Component)]
pub struct KineticEnergy {
    mass: f32,
    velocity: f32,
}

impl KineticEnergy {
    pub fn new(mass: f32, velocity: f32) -> Self {
        Self { mass, velocity }
    }

    pub fn kinetic_energy(&self) -> f32 {
        (0.5 * self.mass) * (self.velocity.powf(2.))
    }
}

#[derive(Component)]
pub struct PotentialEnergy {
    mass: f32,
    height: f32,
    gravitational_potential: f32,
}

impl PotentialEnergy {
    pub fn new(mass: f32, height: f32, gravitational_potential: f32) -> Self {
        Self {
            mass,
            height,
            gravitational_potential,
        }
    }

    pub fn potential_energy(&self) -> f32 {
        self.mass * self.gravitational_potential * self.height
    }
}
