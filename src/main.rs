use bevy::prelude::*;

mod data_structs;
pub use data_structs::*;
mod rendering;
pub use rendering::*;

// Revamping the bevy project from scratch - going to try and focus more on doing subsystems one at a time, and combining them all later.

#[derive(Component)]
struct CamMarker3D;

// Basic setup - loads in first camera (Which drives the window)
fn setup_main_camera(mut commands: Commands){
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CamMarker3D,
    ));


}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_main_camera, setup_scene))
        .run();
}