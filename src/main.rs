use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            range: 1000.0,
            radius: 50.0,
            intensity: 9000000.0,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            range: 1000.0,
            radius: 50.0,
            intensity: 9000000.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, -10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            range: 1000.0,
            radius: 50.0,
            intensity: 9000000.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            range: 1000.0,
            radius: 50.0,
            intensity: 9000000.0,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, -10.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 60.0 * std::f32::consts::PI / 180.0,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
        }),
        transform: Transform::from_xyz(1.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let texture_handle = asset_server.load("table.png");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(30.0, 0.0, 20.0)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        }),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("robotX.glb#Scene0"),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            scale: Vec3::new(10.0, 10.0, 10.0),
        },
        ..default()
    });
}
