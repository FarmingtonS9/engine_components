//Practice designing engines by using components.
//Steam engine
//Diesel engine
//Petrol engine
//Gas engine (LPG/LNG)

use bevy::prelude::*;

mod camera;
mod components;

use components::{
    beam::*, boiler::*, chain::*, cylinder::*, pipe::*, piston::*, pump::*, tank::*, valves::*,
    well::*,
};

use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
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
