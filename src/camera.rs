use crate::*;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

//Constants
const WASD_SCALE_FACTOR: f32 = 15.;
const MOUSE_SCOLL_ZOOM_SCALE_FACTOR: f32 = 5.;

//Components
#[derive(Component)]
pub struct MainCamera;

//Plugin
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        MainCamera,
    ));
}

fn pan_controls(
    mut transform_q: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transform_q.iter_mut() {
        //Keyboard.
        //WASD camera
        //If the "W" or arrow-up key is pressed, move camera upwards.
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            //If true, transform the position of the camera "upwards".
            transform.translation.y += WASD_SCALE_FACTOR; //May have to add this later * time.delta_seconds();
        }
        //If the "S" or arrown-down key is pressed, move camera downwards.
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            //If true, transform the position of the camera "downwards".
            transform.translation.y -= WASD_SCALE_FACTOR; //May have to add this later * time.delta_seconds();
        }
        //If the "A" or arrow-left key is pressed, move camera towards the left.
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            //If true, transform the position of the camera towards the "left".
            transform.translation.x -= WASD_SCALE_FACTOR; //May have to add this later * time.delta_seconds();
        }
        //If the "D" or arrow-right key is pressed, move camera towards the right.
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            //If true, transform the position of the camera towards the right.
            transform.translation.x += WASD_SCALE_FACTOR; //May have to add this later * time.delta_seconds();
        }
    }
}

fn mouse_pan_controls(
    mut transform_q: Query<&mut Transform, With<MainCamera>>,
    mut mouse_motion_event_rdr: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    for mut transform in transform_q.iter_mut() {
        if mouse_button.pressed(MouseButton::Middle) {
            //TODO: Improve panning of camera.
            for mouse_motion in mouse_motion_event_rdr.read() {
                transform.translation.x += mouse_motion.delta.x; //May have to add later * time.delta_seconds();
                transform.translation.y -= mouse_motion.delta.y; //May have to add later * time.delta_seconds();
            }
        }
    }
}

fn zoom_controls(
    mut camera_q: Query<&mut OrthographicProjection, With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_scoll_event: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    //Zoom.
    for mut projection in camera_q.iter_mut() {
        //Scroll wheel.
        for mouse_scoll in mouse_scoll_event.read() {
            if mouse_scoll.y.is_sign_positive() {
                let mut log_scale = projection.scale.ln();
                log_scale -= MOUSE_SCOLL_ZOOM_SCALE_FACTOR * time.delta_seconds();
                projection.scale = log_scale.exp();
            }
            if mouse_scoll.y.is_sign_negative() {
                let mut log_scale = projection.scale.ln();
                log_scale += MOUSE_SCOLL_ZOOM_SCALE_FACTOR * time.delta_seconds();
                projection.scale = log_scale.exp();
            }
            //If "Ctrl" button is held, increase zoom speed.
            if mouse_scoll.y.is_sign_positive() && keyboard_input.pressed(KeyCode::ControlLeft) {
                let mut log_scale = projection.scale.ln();
                log_scale -= MOUSE_SCOLL_ZOOM_SCALE_FACTOR.powf(2.) * time.delta_seconds();
                projection.scale = log_scale.exp();
            }
            if mouse_scoll.y.is_sign_negative() && keyboard_input.pressed(KeyCode::ControlLeft) {
                let mut log_scale = projection.scale.ln();
                log_scale += MOUSE_SCOLL_ZOOM_SCALE_FACTOR.powf(2.) * time.delta_seconds();
                projection.scale = log_scale.exp();
            }
        }
        //Keyboard.
        //If "+" or "Num+" button is pressed, zoom in.
        if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
            let mut log_scale = projection.scale.ln();
            log_scale -= MOUSE_SCOLL_ZOOM_SCALE_FACTOR * time.delta_seconds();
            projection.scale = log_scale.exp();
        }
        //If "-" or "Num-" button is pressed, zoom out.
        if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract)
        {
            let mut log_scale = projection.scale.ln();
            log_scale += MOUSE_SCOLL_ZOOM_SCALE_FACTOR * time.delta_seconds();
            projection.scale = log_scale.exp();
        }
    }
}
