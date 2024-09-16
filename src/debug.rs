use bevy::prelude::*;

use crate::{Pressure, Temperature};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, physical_debug);
    }
}

fn physical_debug(temp_query: Query<(&Temperature, &Pressure)>) {
    for (temperature, pressure) in temp_query.iter() {
        info!("Temperature: {:?}, Pressure: {:?}", temperature, pressure)
    }
}
