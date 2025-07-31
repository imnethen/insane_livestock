use crate::{player, util, AssetHandles, GameState};
use avian3d::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot, explode).run_if(in_state(GameState::Spectating)),
        );
    }
}

#[derive(Component)]
#[require(RigidBody = RigidBody::Dynamic, Sensor, CollisionEventsEnabled, Transform, Collider)]
struct Bullet;

fn shoot(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_pos_query: Query<&Transform, With<Camera>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let trans = camera_pos_query.single().unwrap();
    let bullet_speed = 1000.;
    commands.spawn((
        Bullet,
        Collider::sphere(1.),
        Transform::from_translation(trans.translation + trans.forward() * 0.5),
        LinearVelocity(trans.forward() * bullet_speed),
        Mesh3d(asset_handles.the_sphere.clone().unwrap()),
        MeshMaterial3d(asset_handles.bullet_material.clone().unwrap()),
    ));
}

fn explode(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut player_query: Query<(&Transform, &mut LinearVelocity), With<player::Player>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut collision_events: EventReader<CollisionStarted>,
) {
    let mut bullets: HashMap<Entity, &Transform> = HashMap::new();
    for (bullet_entity, bullet_transform) in bullet_query {
        bullets.insert(bullet_entity, bullet_transform);
    }

    for event in collision_events.read() {
        let (bullet_entity, bullet_trans) = if bullets.contains_key(&event.0) {
            (event.0, bullets[&event.0])
        } else {
            (event.1, bullets[&event.1])
        };

        commands.entity(bullet_entity).despawn();
        commands.spawn(util::explosion(&asset_handles, bullet_trans.translation));

        for (player_trans, mut linvel) in &mut player_query {
            let dir = (player_trans.translation - bullet_trans.translation).normalize();
            let strength = 1e5
                / player_trans
                    .translation
                    .distance_squared(bullet_trans.translation);

            linvel.0 += dir * strength;
        }
    }
}
