mod gun;
mod menu;
mod player;
mod spectator_camera;
mod twitch;
mod util;

use avian3d::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

const SHEEP_SIZE: Vec3 = vec3(1.5, 3.5, 3.);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        resolution: bevy::window::WindowResolution::new(2048., 1152.),
                        cursor_options: bevy::window::CursorOptions {
                            visible: true,
                            hit_test: true,
                            grab_mode: bevy::window::CursorGrabMode::None,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(bevy::log::LogPlugin {
                    filter: bevy::log::DEFAULT_FILTER.replace("wgpu=error", "wgpu=off"),
                    ..default()
                }),
        )
        .add_plugins((
            bevy_obj::ObjPlugin,
            BillboardPlugin,
            PhysicsPlugins::default(),
            twitch::TwitchPlugin,
            menu::MenuPlugin,
            player::PlayerPlugin,
            spectator_camera::SpectatorCameraPlugin,
            gun::GunPlugin,
        ))
        .insert_resource(AssetHandles::default())
        .insert_resource(SkyboxLoaded::default())
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (setup_skybox.run_if(should_run_skybox),))
        .run();
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    Start,
    Connected,
    Spectating,
    End,
}

#[derive(Resource, Default)]
struct AssetHandles {
    sheep_mesh: Option<Handle<Mesh>>,
    sheep_material: Option<Handle<StandardMaterial>>,
    sheep_sized_cuboid: Option<Handle<Mesh>>,
    the_sphere: Option<Handle<Mesh>>,
    explosion_sound: Option<Handle<AudioSource>>,
    explosion_cube: Option<Handle<Mesh>>,
    explosion_material: Option<Handle<StandardMaterial>>,
    skybox: Option<Handle<Image>>,
    bullet_material: Option<Handle<StandardMaterial>>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    let skybox_handle = asset_server.load::<Image>("skybox.png");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 300.),
        SpatialListener::new(-1.),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.,
            rotation: Quat::default(),
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(600., 1., 600.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            base_color_texture: Some(asset_server.load("grass.png")),
            unlit: true,
            ..Default::default()
        })),
        RigidBody::Static,
        Collider::cuboid(600., 1., 600.),
    ));

    let fence_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("fence.png")),
        unlit: true,
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        ..Default::default()
    });

    // spawn fence
    for i in 0..30 {
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(vec3(1., 0., 0.), vec2(2.5, 10.)))),
            MeshMaterial3d(fence_material.clone()),
            Transform::from_xyz(-300., 2.5, i as f32 * 20. - 290.),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(vec3(-1., 0., 0.), vec2(2.5, 10.)))),
            MeshMaterial3d(fence_material.clone()),
            Transform::from_xyz(300., 2.5, i as f32 * 20. - 290.),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(vec3(0., 0., 1.), vec2(10., 2.5)))),
            MeshMaterial3d(fence_material.clone()),
            Transform::from_xyz(i as f32 * 20. - 290., 2.5, -300.),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::new(vec3(0., 0., -1.), vec2(10., 2.5)))),
            MeshMaterial3d(fence_material.clone()),
            Transform::from_xyz(i as f32 * 20. - 290., 2.5, 300.),
        ));
    }

    asset_handles.sheep_mesh = Some(asset_server.load::<Mesh>("goat/goat.obj"));
    asset_handles.sheep_sized_cuboid = Some(meshes.add(Cuboid::from_size(SHEEP_SIZE)));
    asset_handles.the_sphere = Some(meshes.add(Sphere::new(1.)));
    asset_handles.sheep_material = Some(materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("goat/goat.png")),
        unlit: true,
        ..Default::default()
    }));
    asset_handles.explosion_sound = Some(asset_server.load("explosion.ogg"));
    asset_handles.explosion_cube = Some(meshes.add(Cuboid::from_size(Vec3::splat(10.))));
    asset_handles.explosion_material = Some(materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("explosion.png")),
        cull_mode: None,
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        ..Default::default()
    }));
    asset_handles.skybox = Some(skybox_handle);
    asset_handles.bullet_material = Some(materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("33fire.png")),
        unlit: true,
        ..Default::default()
    }));
}

#[derive(Resource, Default)]
struct SkyboxLoaded(bool);

fn should_run_skybox(
    asset_server: Res<AssetServer>,
    asset_handles: Res<AssetHandles>,
    skybox_loaded: Res<SkyboxLoaded>,
) -> bool {
    !skybox_loaded.0
        && asset_server
            .load_state(&asset_handles.skybox.clone().unwrap())
            .is_loaded()
}

fn setup_skybox(
    asset_handles: Res<AssetHandles>,
    mut images: ResMut<Assets<Image>>,
    mut skybox_loaded: ResMut<SkyboxLoaded>,
) {
    let img = images
        .get_mut(&asset_handles.skybox.clone().unwrap())
        .unwrap();
    img.reinterpret_stacked_2d_as_array(6);
    img.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
        dimension: Some(bevy::render::render_resource::TextureViewDimension::Cube),
        ..Default::default()
    });

    skybox_loaded.0 = true;
}
