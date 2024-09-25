use crate::constants::STANDARD_TEMPERATURE_NIST;
use bevy::prelude::*;

pub struct TemperaturePlugin;

impl Plugin for TemperaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, adjust_temperature_with_keyboard);
    }
}

fn adjust_temperature_with_keyboard(
    mut temperature_query: Query<&mut Temperature>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut temperature in temperature_query.iter_mut() {
        for key in keyboard_input.get_pressed() {
            if *key == KeyCode::KeyT {
                temperature.0 += 0.20
            }
            if *key == KeyCode::KeyG {
                temperature.0 -= 0.20
            }
        }
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
