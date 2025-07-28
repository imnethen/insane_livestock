use crate::twitch;
use crate::{AssetHandles, SHEEP_SIZE};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct Players(HashSet<String>);

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    asset_handles: &Res<AssetHandles>,
    name: String,
    pos: Vec3,
    rot_angle: f32,
) {
    commands.spawn((
        // Mesh3d(asset_handles.sheep_sized_cuboid.clone().unwrap()),
        // MeshMaterial3d(asset_handles.sheep_material.clone().unwrap()),
        Player,
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
            (vec3(0., 1., -1.5), Quat::default(), Collider::sphere(1.)),
        ]),
        ComputedMass::new(20.),
        ComputedCenterOfMass::new(0., -1., 0.),
        Visibility::Inherited,
        children![
            (
                Mesh3d(asset_handles.sheep_mesh.clone().unwrap()),
                MeshMaterial3d(asset_handles.sheep_material.clone().unwrap()),
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
            //     MeshMaterial3d(asset_handles.sheep_material.clone().unwrap()),
            //     Transform::from_xyz(0., 1., -1.5)
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

    if right_score.abs() < 0.0001 {
        0.
    } else if right_score > 0. {
        -0.02
    } else {
        0.02
    }
}

pub fn control_players(
    mut player_query: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    let mut positions: Vec<Vec3> = vec![];
    for (_, trans) in &player_query {
        positions.push(trans.translation);
    }
    let player_acc = 1.;
    let player_max_speed = 50.;

    for (mut linvel, mut trans) in &mut player_query {
        // rotate
        let angle = decide_angle(&trans, &positions);
        trans.rotate_y(angle);
        let rotated_xz = Mat2::from_angle(angle).mul_vec2(linvel.0.xz());
        linvel.0 = vec3(rotated_xz.x, linvel.0.y, rotated_xz.y);

        // accelerate forwards
        linvel.0 += player_acc * trans.forward();
        let clamped_xz = linvel.0.xz().clamp_length_max(player_max_speed);
        linvel.0 = vec3(clamped_xz.x, linvel.0.y, clamped_xz.y);

        let adj = trans.right().dot(linvel.0) * trans.right() * 0.1;
        linvel.0 -= adj;
    }
}

pub fn kill_players(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, trans) in player_query {
        if trans.up().dot(Vec3::Y) < 0.1 {
            commands.entity(entity).despawn();
            commands.spawn((
                AudioPlayer::new(asset_handles.explosion_sound.clone().unwrap()),
                PlaybackSettings::DESPAWN
                    .with_volume(bevy::audio::Volume::Linear(0.5))
                    .with_spatial(true)
                    .with_spatial_scale(bevy::audio::SpatialScale::new(0.03)),
                Transform::default().with_translation(trans.translation),
                GlobalTransform::default(),
                Mesh3d(asset_handles.explosion_cube.clone().unwrap()),
                MeshMaterial3d(asset_handles.explosion_material.clone().unwrap()),
            ));
        }
    }
}

pub fn read_user_events(
    mut events: EventReader<twitch::UserJoined>,
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut players: ResMut<Players>,
    // TODO: remove
    input: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        if players.0.contains(&event.0) {
            continue;
        }
        players.0.insert(event.0.clone());
        let pos = vec3(
            rand::random_range(-300.0..300.0),
            3.,
            rand::random_range(-300.0..300.0),
        );
        spawn_player(
            &mut commands,
            &asset_handles,
            event.0.clone(),
            pos,
            rand::random_range(0.0..std::f32::consts::TAU),
        );
    }

    if input.pressed(KeyCode::Space) {
        let name = "mrrow".to_owned();
        players.0.insert(name.clone());
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
        );
    }
}
