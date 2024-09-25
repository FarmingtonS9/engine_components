//Practice designing engines by using components.
//Steam engine
//Diesel engine
//Petrol engine
//Gas engine (LPG/LNG)

use bevy::prelude::*;
//use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};
use physics::displacement::Displacement;
use physics::velocity::Velocity;
use rand::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

/// Camera component.
mod camera;
/// Contains the components found in machines.
mod components;
/// Constants found in natural sciences.
mod constants;
/// Debugging component. Used to visualise and record how the system is working.
mod debug;
/// Contains components of physics.
mod physics;
/// Contains state variables used in calculations for systems.
mod state_variables;

use components::{
    beam::*, boiler::*, chain::*, cylinder::*, pipe::*, piston::*, pump::*, tank::*, valves::*,
    well::*,
};

use camera::CameraPlugin;
use debug::DebugPlugin;
use state_variables::{
    pressure::PressurePlugin, temperature::TemperaturePlugin, volume::DimensionPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SickleUiPlugin)
        .add_plugins(CameraPlugin)
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugPlugin)
        .add_plugins(BoilerPlugin)
        .add_plugins(TemperaturePlugin)
        .add_systems(Update, displacement)
        .run();
}

//Testing my ability to use components to make a thing.
//Subject to change.
#[derive(Bundle)]
struct AtmosphericSteamEngine {
    beam: Beam,
    boiler: Boiler,
    chain: Chain,
    cylinder: components::cylinder::Cylinder,
    pipe: Pipe,
    piston: Piston,
    pump: Pump,
    tank: Tank,
    valve: Valve,
    well: Well,
}

fn displacement(mut transform_query: Query<&mut Transform>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for mut transform in transform_query.iter_mut() {
        // 1. Store the initial translation.
        let initial_x_translation = transform.translation.x;
        let initial_y_translation = transform.translation.y;
        let initial_translation = Vec2::new(initial_x_translation, initial_y_translation);
        // 2. Calculate the time difference between Updates.
        let time_difference = time.delta_seconds();
        // 3. Generate a random number.
        let x_rng = (rng.gen_range(-100..=100) as f32) / 100.;
        let y_rng = (rng.gen_range(-100..=100) as f32) / 100.;
        // 4. Set the rng number as the new transformation.
        transform.translation.x = 10.;
        transform.translation.y = 10.;
        // 5. Calculate the displacement of the transformation (final transformation - initial transform).
        let displacement =
            Displacement::new(transform.translation.truncate(), initial_translation).displacement();
        // 6. Calculate the velocity of the Transform component.
        let velocity = Velocity::new(displacement, time_difference).velocity();
        info!("Displacement: {displacement} m, Time difference: {time_difference} s, Velocity: {velocity} m/s")
    }
}
