/*  first_person.rs
    Rendering pipeline for first-person view
    Style will be dungeon crawler (Etrian Odyssey, Labrynth of Refrain)

    Due to how this is envisioned, a 3D camera is necessary

    Handles the following actions - 
        Loads into memory current map (and/or what's visible)
        Movement (Forwards, Backwards, Left, Right)
        Turning (Left, Right)
 */

 #![allow(unused)]

use std::{f32::consts::PI, fmt::Debug};

use bevy::prelude::*;
use crate::data_structs::*;
use bevy::input::mouse::MouseMotion;

const SCALE:f32 = 8.;

/*
    General flow - 
        Fetch the JSON resource, AND entities with the CurrMap resource (Or Town Map resource). Also fetch the coordinate map resource for current map
        Begin loading in sprites, walls, etc...
*/

// Render the current area - tries to optimize by 'hiding' things that you can't currently see if necessary
pub fn render_region(
    mut commands: Commands,
    map_data: Res<CurrMap>) {
    // Iterate over each grid, render it to the 'world' and figure it out from there


}

// Rendering function - renders the entire map (without any optimizations - just basic colors )
pub fn render_debug_map(
    mut commands: Commands,
    map_data: Res<CurrMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    // Do two iterations (One for the tiles, then one for the walls)
    // For now, each wall will be a rectangle, as will each floor 
    // I plan to convert floor into a tilemesh or something similar later to make it just one plane
    let dim_x = map_data.map_data.dim_x;
    let dim_y = map_data.map_data.dim_y;

    commands.spawn(PbrBundle{
        mesh: meshes.add(Cuboid::new(2.,2.,2.)),
        material: materials.add(Color::rgb_u8(200, 0, 200)),
        ..default()
    });

    // Floor tile loop
    for x in 0..dim_x {
        for y in 0..dim_y {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Rectangle::new(SCALE - (SCALE*0.1), SCALE - (SCALE*0.1))),
                material: materials.add(Color::rgb_u8(200, 200, 0)),
                transform: Transform::from_xyz(SCALE * x as f32, 0.0, SCALE * y as f32)
                                    .with_rotation(Quat::from_rotation_x(270. * PI / 180. )), 
                ..default()
            });
        
        }
    }

    // Wall loop - this is currently separate from floor tiles due to there being multiple ways to approach this
    // Currently iterates through all horizontal walls then vertical walls, optionally rendering a cuboid based on the index in wall vector
    // This version would have the same texture on both sides of the wall - which may not be what's desired - 
    // Alternate is to use the tile's wall array and render rectangles for the 'wall', since it's only visible from one direction for whatever reason.

    for x in 0..dim_x + 1 {
        for h in 0..dim_x {
            let cur_index = (h + x *(dim_x + dim_y+1)) as usize;
            if map_data.map_data.walls[cur_index].state == WallState::NoWall{
                // No wall present, onto the next step
                // continue;
            }
            // Wall was found, spawn and render
            commands.spawn(PbrBundle{
                mesh: meshes.add(Cuboid::new(0.2,SCALE,SCALE)),
                material: materials.add(Color::rgb_u8(70, 0, 200)),
                transform: Transform::from_xyz(SCALE * x as f32  - SCALE/2., 0.0, SCALE * h as f32)
                                    .with_rotation(Quat::from_rotation_x(270. * PI / 180. )), 
                ..default()
            });
        }
    }

    for y in 0..dim_y {
        for v in 0..dim_y + 1 {
            let cur_index = (dim_x + v + y*(dim_x + dim_y + 1)) as usize;
            if map_data.map_data.walls[cur_index].state == WallState::NoWall{
                // No wall present, onto the next step
                // continue;
            }
            // Wall was found, spawn and render - change color accordingly if we can
            commands.spawn(PbrBundle{
                mesh: meshes.add(Cuboid::new(SCALE,SCALE,0.2)),
                material: materials.add(Color::rgb_u8(70, 0, 200)),
                transform: Transform::from_xyz(SCALE * y as f32, 0.0, SCALE * v as f32 - SCALE/2.)
                                    .with_rotation(Quat::from_rotation_x(0. )), 
                ..default()
            });
        }
    }
    
    

}

// Handle shifting of our 'coordinates' by moving the 3D camera
// Supports moving 4(6) ways - Left, Right, Forwards, Backwards (Up, Down)
// Up/Down will be implemented later, for now I'll just assume a perfect 2d plane
pub fn grid_movement(
    // query: Query<3dCameraBundle>
){
    // Fetch the camera with the query, and whatever movement was plugged in
    // Move the camera accordingly with a 'slide' motion - just reuse a lot of the logic from the debug camera's movements, although the 'slide' itself may be tricky
    
}


// Handle matching camera to player direction (Rotation)
// Should only be able to turn in 90 degree increments, but being able to do a 180 or 360 may be handy.
pub fn grid_rotation(
    // query: Query<3dCameraBundle>
){
    // Fetch the camera with query, and update the values to change the rotation accordingly.
    // This should be a sliding motion rather than a jump to a new coordinate. 
    
}

