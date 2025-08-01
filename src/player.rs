use crate::{menu, twitch, util, AssetHandles, GameState};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use std::collections::HashSet;

pub const SHEEP_SIZE: Vec3 = vec3(2.5, 3.5, 3.5);

#[derive(Resource, Default)]
pub struct Players(pub HashSet<String>);

#[derive(Component)]
pub struct Player(String);

#[derive(Component)]
struct Speed(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Players::default())
            .add_systems(
                Update,
                read_user_events.run_if(in_state(GameState::Connected)),
            )
            .add_systems(
                FixedUpdate,
                // (/*control_players,*/ kill_players, end).run_if(in_state(GameState::Spectating)),
                (control_players, kill_players, end).run_if(in_state(GameState::Spectating)),
            );
    }
}

fn spawn_player(
    commands: &mut Commands,
    asset_handles: &Res<AssetHandles>,
    name: String,
    pos: Vec3,
    rot_angle: f32,
    speed: f32,
) {
    commands.spawn((
        // Mesh3d(asset_handles.sheep_sized_cuboid.clone().unwrap()),
        // MeshMaterial3d(asset_handles.player_material.clone().unwrap()),
        Player(name.clone()),
        Speed(speed),
        Transform::default()
            .with_translation(pos)
            .with_rotation(Quat::from_rotation_y(rot_angle)),
        RigidBody::Dynamic,
        Collider::compound(vec![
            (
                Vec3::ZERO,
                Quat::default(),
                Collider::cuboid(SHEEP_SIZE.x, SHEEP_SIZE.y, SHEEP_SIZE.z),
            ),
            (vec3(0., 0.7, -2.5), Quat::default(), Collider::sphere(1.25)),
        ]),
        ComputedMass::new(100.),
        ComputedCenterOfMass::new(0., -1.7, 0.),
        Visibility::Inherited,
        children![
            (
                Mesh3d(asset_handles.player_mesh.clone().unwrap()),
                MeshMaterial3d(asset_handles.player_material.clone().unwrap()),
                Transform::default()
                    .with_scale(Vec3::splat(0.1))
                    .with_translation(vec3(0.2, -1.7, 0.))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI))
            ),
            (
                BillboardText::new(name),
                Transform::default()
                    .with_scale(Vec3::splat(0.05))
                    .with_translation(vec3(0., 4., 0.))
            ),
            // (
            //     Mesh3d(asset_handles.the_sphere.clone().unwrap()),
            //     MeshMaterial3d(asset_handles.player_material.clone().unwrap()),
            //     Transform::from_xyz(0., 0.7, -2.5)
            // )
        ],
    ));
}

fn is_to_the_right(transform: &Transform, pos: &Vec3) -> bool {
    let pos = pos - transform.translation;
    pos.dot(transform.right().into()) > 0.
}

fn decide_angle(transform: &Transform, all_pos: &Vec<Vec3>) -> f32 {
    let mut right_score = 0.;

    for pos in all_pos {
        if pos.distance_squared(transform.translation) < 0.01 {
            continue;
        }

        let strength = 1. / pos.distance(transform.translation).powi(2);
        let sign = ((is_to_the_right(transform, pos) as i32) * 2 - 1) as f32;
        right_score += strength * sign;
    }

    // TODO
    if right_score.abs() < 0. {
        0.
    } else if right_score > 0. {
        -0.02
    } else {
        0.02
    }
}

fn control_players(
    mut player_query: Query<(&mut LinearVelocity, &mut Transform, &Speed), With<Player>>,
) {
    let mut positions: Vec<Vec3> = vec![];
    for (_, trans, _) in &player_query {
        positions.push(trans.translation);
    }
    let player_acc = 1.;

    for (mut linvel, mut trans, max_speed) in &mut player_query {
        // rotate
        let angle = decide_angle(&trans, &positions);
        trans.rotate_y(angle);
        let rotated_xz = Mat2::from_angle(angle).mul_vec2(linvel.0.xz());
        linvel.0 = vec3(rotated_xz.x, linvel.0.y, rotated_xz.y);

        // accelerate forwards
        // linvel.0 += player_acc * trans.forward();
        // let clamped_xz = linvel.0.xz().clamp_length_max(max_speed.0);
        // linvel.0 = vec3(clamped_xz.x, linvel.0.y, clamped_xz.y);
        if linvel.0.xz().length() < max_speed.0 {
            linvel.0 += player_acc * trans.forward();
        }

        // stop them from drifting if theyre on the ground
        if trans.translation.y < 3. {
            let adj = trans.right().dot(linvel.0.normalize()) * trans.right() * 3.;
            linvel.0 -= adj;
        }
    }
}

fn kill_players(
    mut commands: Commands,
    mut players: ResMut<Players>,
    asset_handles: Res<AssetHandles>,
    player_query: Query<(Entity, &Transform, &Player)>,
) {
    for (entity, trans, name) in player_query {
        // die if close enough to upside down or outside the bounds
        if trans.up().dot(Vec3::Y) < 0.15
            || trans
                .translation
                .clamp(Vec3::splat(-300.), Vec3::splat(300.))
                != trans.translation
        {
            players.0.remove(&name.0);
            commands.entity(entity).despawn();
            commands.spawn(util::explosion(&asset_handles, trans.translation));
        }
    }
}

fn end(player_query: Query<&Player>, mut next_game_state: ResMut<NextState<crate::GameState>>) {
    if player_query.iter().len() == 1 {
        next_game_state.set(crate::GameState::End);
    }
}

fn read_user_events(
    mut events: EventReader<twitch::UserJoined>,
    mut commands: Commands,
    settings: Res<menu::Settings>,
    asset_handles: Res<AssetHandles>,
    mut players: ResMut<Players>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        let msg = event.0.clone();
        for i in 0..settings.goats_per_player {
            let name = if settings.goats_per_player == 1 {
                msg.sender.clone()
            } else {
                msg.sender.clone() + " " + &i.to_string()
            };

            if players.0.contains(&name)
                || (settings.filter_joins && msg.text != "!play".to_owned())
            {
                continue;
            }
            players.0.insert(name.clone());
            let pos = vec3(
                rand::random_range(-300.0..300.0),
                3.,
                rand::random_range(-300.0..300.0),
            );
            spawn_player(
                &mut commands,
                &asset_handles,
                name.clone(),
                pos,
                rand::random_range(0.0..std::f32::consts::TAU),
                rand::random_range(40.0..60.0),
            );
        }
    }

    if input.pressed(KeyCode::Space) {
        let mut name = "mrrow ".to_owned();
        for i in 2..999999 {
            if !players.0.contains(&(name.clone() + &i.to_string())) {
                name = name.clone() + &i.to_string();
                players.0.insert(name.clone());
                break;
            }
        }
        let pos = vec3(
            rand::random_range(-300.0..300.0),
            3.,
            rand::random_range(-300.0..300.0),
        );
        spawn_player(
            &mut commands,
            &asset_handles,
            name,
            pos,
            rand::random_range(0.0..std::f32::consts::TAU),
            rand::random_range(40.0..60.0),
        );
    }
}
