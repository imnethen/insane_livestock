use bevy::prelude::*;

pub fn move_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let camera_speed = if input.pressed(KeyCode::ShiftLeft) {
        1.6
    } else {
        0.8
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

pub fn rotate_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        return;
    }

    let mouse_delta = mouse_motion.delta;
    let sensitivity = 1e-2;
    let mut tf = transform_query.single_mut().unwrap();
    tf.rotate_y(-sensitivity * mouse_delta.x);
    tf.rotate_local_x(-sensitivity * mouse_delta.y);
}
