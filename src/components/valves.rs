use bevy::prelude::*;

#[derive(Component)]
pub struct Valve;

//Check valve
#[derive(Component)]
pub struct CheckValve;

//Snifting valve
#[derive(Component)]
pub struct SniftingValve;

//Water injection valve
#[derive(Component)]
pub struct WaterInjectionValve;

//Regulator valve
#[derive(Component)]
pub struct RegulatorValve;
