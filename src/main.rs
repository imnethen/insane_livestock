use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

mod twitch;

#[derive(Resource, Default)]
struct CursorMovement(Vec2);

fn accumulate_cursor_movement(
    mut cursor_movement: ResMut<CursorMovement>,
    mut move_events: EventReader<CursorMoved>,
) {
    cursor_movement.0 = Vec2::ZERO;
    for event in move_events.read() {
        cursor_movement.0 += event.delta.unwrap_or(Vec2::ZERO);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                cursor_options: bevy::window::CursorOptions {
                    visible: true,
                    hit_test: true,
                    grab_mode: bevy::window::CursorGrabMode::None,
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_obj::ObjPlugin)
        .add_plugins(BillboardPlugin)
        .add_plugins(twitch::TwitchPlugin)
        .insert_resource(AssetHandles::default())
        .insert_resource(CursorMovement::default())
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (read_user_events.run_if(in_state(GameState::Menu)),),
        )
        .add_systems(FixedPreUpdate, accumulate_cursor_movement)
        .add_systems(
            FixedUpdate,
            (move_spectator_camera, rotate_spectator_camera),
        )
        .run();
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    Menu,
    Alive,
    Spectator,
}

#[derive(Resource, Default)]
struct AssetHandles {
    sheep_mesh: Option<Handle<Mesh>>,
    sheep_material: Option<Handle<StandardMaterial>>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7., 8., 10.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));

    asset_handles.sheep_mesh = Some(asset_server.load::<Mesh>("goat/goat.obj"));
    asset_handles.sheep_material = Some(materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("goat/goat.png")),
        unlit: true,
        ..Default::default()
    }));
}

fn spawn_sheep(commands: &mut Commands, asset_handles: &Res<AssetHandles>, name: String) {
    commands.spawn((
        Mesh3d(asset_handles.sheep_mesh.clone().unwrap()),
        MeshMaterial3d(asset_handles.sheep_material.clone().unwrap()),
        Transform::default().with_scale(Vec3::splat(0.1)),
        BillboardText::new(name),
    ));
}

fn read_user_events(
    mut events: EventReader<twitch::UserJoined>,
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
) {
    for event in events.read() {
        spawn_sheep(&mut commands, &asset_handles, event.0.clone());
    }
}

fn move_spectator_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let camera_speed = if input.pressed(KeyCode::ShiftLeft) {
        0.5
    } else {
        0.3
    };
    let mut tf = transform_query.single_mut().unwrap();

    let forw = tf.clone().forward();
    let right = tf.clone().right();
    let up = tf.clone().up();

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

fn rotate_spectator_camera(
    mut transform_query: Query<&mut Transform, With<Camera>>,
    cursor_movement: Res<CursorMovement>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        return;
    }

    let sensitivity = 4e-3;
    let mut tf = transform_query.single_mut().unwrap();
    tf.rotate_y(-sensitivity * cursor_movement.0.x);
    tf.rotate_local_x(-sensitivity * cursor_movement.0.y);
}
