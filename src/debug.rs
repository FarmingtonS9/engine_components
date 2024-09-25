use bevy::prelude::*;

use crate::state_variables::{pressure::Pressure, temperature::Temperature, volume::Dimension};

//Local debug constants
const DEBUG_FONT_SIZE: f32 = 20.;

pub struct DebugPlugin;

#[derive(Component)]
struct TemperatureText;
#[derive(Component)]
struct PressureText;
#[derive(Component)]
struct VolumeText;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, physical_debug_setup).add_systems(
            Update,
            (
                temperature_display_debug,
                pressure_display_debug,
                volume_display_debug,
            ),
        );
    }
}

fn physical_debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = asset_server.load("fonts/Roboto-Thin.ttf");
    //Temperature setup
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Temperature: ",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_asset.clone(),
                font_size: DEBUG_FONT_SIZE,
                ..default()
            }),
            TextSection::new(
                " K",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
        ]),
        TemperatureText,
    ));
    //Pressure setup
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Pressure: ",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_asset.clone(),
                font_size: DEBUG_FONT_SIZE,
                ..default()
            }),
            TextSection::new(
                " Pa",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(94.),
            right: Val::Percent(86.),
            ..default()
        }),
        PressureText,
    ));
    //Volume setup
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Volume: ",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_asset.clone(),
                font_size: DEBUG_FONT_SIZE,
                ..default()
            }),
            TextSection::new(
                " L",
                TextStyle {
                    font: font_asset.clone(),
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(91.),
            right: Val::Percent(90.),
            ..default()
        }),
        VolumeText,
    ));
}

fn temperature_display_debug(
    temp_query: Query<&Temperature>,
    mut text_query: Query<&mut Text, With<TemperatureText>>,
) {
    for mut text in &mut text_query {
        for temperature in temp_query.iter() {
            let value = temperature.temperature();
            text.sections[1].value = format!("{value:.2}")
        }
    }
}

fn pressure_display_debug(
    pressure_query: Query<&Pressure>,
    mut text_query: Query<&mut Text, With<PressureText>>,
) {
    for mut text in &mut text_query {
        for pressure in pressure_query.iter() {
            let value = pressure.pressure();
            text.sections[1].value = format!("{value:.2}")
        }
    }
}

fn volume_display_debug(
    volume_query: Query<&Dimension>,
    mut text_query: Query<&mut Text, With<VolumeText>>,
) {
    for mut text in &mut text_query {
        for volume in volume_query.iter() {
            let value = volume.volume();
            text.sections[1].value = format!("{value:.2}")
        }
    }
}
