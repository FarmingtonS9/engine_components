use bevy::prelude::*;

#[derive(Component)]
pub struct Pump;

//Main pump
#[derive(Component)]
pub struct MainPump;

//Condensate pump
#[derive(Component)]
pub struct CondensatePump;

//Auxiliary pump
#[derive(Component)]
pub struct AuxiliaryPump;
