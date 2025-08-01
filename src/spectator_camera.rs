use crate::GameState;
use bevy::prelude::*;

pub struct SpectatorCameraPlugin;

impl Plugin for SpectatorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (move_camera, rotate_camera).run_if(in_state(GameState::Spectating)),
        );
    }
}

fn move_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let camera_speed = if input.pressed(KeyCode::ShiftLeft) {
        3.
    } else {
        1.5
    };
    let mut tf = transform_query.single_mut().unwrap();

    let forw = tf.clone().forward().normalize();
    let right = tf.clone().right().normalize();
    let up = tf.clone().up().normalize();

    if input.pressed(KeyCode::KeyW) {
        tf.translation += forw * camera_speed;
    }
    if input.pressed(KeyCode::KeyS) {
        tf.translation -= forw * camera_speed;
    }
    if input.pressed(KeyCode::KeyA) {
        tf.translation -= right * camera_speed;
    }
    if input.pressed(KeyCode::KeyD) {
        tf.translation += right * camera_speed;
    }
    if input.pressed(KeyCode::KeyQ) {
        tf.translation -= up * camera_speed;
    }
    if input.pressed(KeyCode::KeyE) {
        tf.translation += up * camera_speed;
    }
}

fn rotate_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        return;
    }

    let mouse_delta = mouse_motion.delta;
    let sensitivity = 8e-3;
    let mut tf = transform_query.single_mut().unwrap();
    tf.rotate_y(-sensitivity * mouse_delta.x);
    tf.rotate_local_x(-sensitivity * mouse_delta.y);
}
