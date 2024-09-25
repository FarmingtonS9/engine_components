use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

//Program-wide constants
use crate::state_variables::{pressure::Pressure, temperature::Temperature, volume::Dimension};
//Local constants
const MOL: f32 = 0.00000000001;

pub struct BoilerPlugin;

impl Plugin for BoilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, firebox_setup)
            .add_systems(PostStartup, firebox_dimension_setup);
    }
}

#[derive(Component)]
pub struct Boiler;

#[derive(Component)]
pub struct Firebox;

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
    /// Pressure of the system in Pascals (Newtons per square metre). Default is set to atmospheric pressure at sea level.
    pressure: Pressure,
    temperature: Temperature,
    /// Amount of substance in the firebox. Measured in mols
    substance: Substance,
}

fn firebox_setup(mut commands: Commands) {
    let firebox = commands.spawn((
        FireboxBundle {
            component_name: Boiler,
            dimensions: Dimension::new(Vec2::new(100., 100.)),
            pressure: Pressure::default(),
            temperature: Temperature::default(),
            substance: Substance::default(),
        },
        Firebox,
    ));
}

fn firebox_dimension_setup(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    dimension_query: Query<&Dimension>,
) {
    let firebox_dimension_query = dimension_query.single();
    let firebox_dimension = (
        firebox_dimension_query.dim_x(),
        firebox_dimension_query.dim_y(),
    );
    let firebox_shape =
        Mesh2dHandle(mesh.add(Rectangle::new(firebox_dimension.0, firebox_dimension.1)));
    let firebox_shape_colour = Color::srgb(0.5, 0.5, 0.5);
    commands.spawn(MaterialMesh2dBundle {
        mesh: firebox_shape,
        material: material.add(firebox_shape_colour),
        transform: Transform::IDENTITY.with_scale(Vec3::splat(1.)),
        ..default()
    });
}

fn adjust_mol_with_keyboard(
    mut substance_query: Query<&mut Substance>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut substance in substance_query.iter_mut() {
        for key in keyboard_input.get_pressed() {
            if *key == KeyCode::KeyW {
                substance.0 += 0.20
            }
            if *key == KeyCode::KeyS {
                substance.0 -= 0.20
            }
        }
    }
}

#[derive(Component)]
pub struct Substance(f32);

impl Substance {
    fn new(mol: f32) -> Self {
        Self(mol)
    }
}

impl Default for Substance {
    fn default() -> Self {
        Self(MOL)
    }
}
