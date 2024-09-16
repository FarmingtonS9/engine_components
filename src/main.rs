//Practice designing engines by using components.
//Steam engine
//Diesel engine
//Petrol engine
//Gas engine (LPG/LNG)

use bevy::prelude::*;
//use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};

mod camera;
mod components;
mod constants;
mod debug;

use components::{
    beam::*, boiler::*, chain::*, cylinder::*, pipe::*, piston::*, pump::*, tank::*, valves::*,
    well::*,
};

use camera::CameraPlugin;
use debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugPlugin)
        .add_plugins(BoilerPlugin)
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
