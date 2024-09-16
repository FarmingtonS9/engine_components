use bevy::prelude::*;

use crate::constants::{
    IDEAL_GAS_CONSTANT, STANDARD_ATMOSPHERE_PRESSURE, STANDARD_TEMPERATURE_NIST,
};

pub struct BoilerPlugin;

impl Plugin for BoilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, firebox_setup)
            .add_systems(Update, temperature_changes_boiler)
            .add_systems(PostUpdate, pressure_changes_boiler);
    }
}

#[derive(Component)]
pub struct Boiler;

/// Basic set of components for the boiler.
/// Used in the Newcomen engine.
/// Very low pressure compared to the atmosphere but still vulnerable to boiler explosion if pressure goes too high because of the material used at the time (copper).
#[derive(Component)]
pub struct AtmosphericBoiler {
    firebox: FireboxBundle,
}

/// Contains the combustion of fuel and air.
/// The released energy is used to, typically, heat water for steam generation.
///
/// Sequence of operation:
///
/// 1. Energy released by the combustion increases temperature.
///
/// 2. Temperature increases the pressure.
///
/// 3. These parameters are contained within a known volume.
///
/// Due to the way fireboxes are deployed, pressure escapes to atmosphere through tubes (or around). This creates a partial vacuum (I believe), which draws in new air from atmosphere.
#[derive(Bundle)]
pub struct FireboxBundle {
    component_name: Boiler,
    /// Dimensions of the firebox in metres. This allows volume to be calculated.
    dimensions: Dimension,
    pressure: Pressure,
    temperature: Temperature,
}

fn firebox_setup(mut commands: Commands) {
    let firebox = commands.spawn(FireboxBundle {
        component_name: Boiler,
        dimensions: Dimension::new(Vec2::new(10., 10.)),
        pressure: Pressure::default(),
        temperature: Temperature::default(),
    });
}

fn temperature_changes_boiler(mut temperature_query: Query<&mut Temperature, With<Boiler>>) {
    for mut temperature in temperature_query.iter_mut() {
        temperature.0 = temperature.0 + 1.
    }
}

fn pressure_changes_boiler(
    mut pressure_query: Query<(&mut Pressure, &Temperature, &Dimension), With<Boiler>>,
) {
    for (mut pressure, temperature, dimension) in pressure_query.iter_mut() {
        let volume = dimension.volume();
        let mol = 1.;
        let constant = (pressure.0 * volume) / temperature.temperature();
        pressure.0 = (constant * temperature.temperature()) / volume
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

/// Generic temperature component.
///
/// Attaches to a component which requires tracking and calculations of temperature.
/// Allows for default temperature (room temperature) to be quickly referenced.
#[derive(Component, Debug)]
pub struct Temperature(f32);

impl Temperature {
    /// Units in Kelvin. Standard (room) temperature = 293.15 Kelvin.
    fn new(temperature: f32) -> Self {
        Self(temperature)
    }

    /// Helper function for returning temperature value.
    pub fn temperature(&self) -> f32 {
        self.0
    }
}

impl Default for Temperature {
    /// Standard (room) temperature.
    /// 293.15 K (20 degrees Celcius).
    fn default() -> Self {
        Self(STANDARD_TEMPERATURE_NIST)
    }
}
