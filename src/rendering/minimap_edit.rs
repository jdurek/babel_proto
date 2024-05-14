/*  minimap_edit.rs
    Additional functionality related to the minimap, but only when accessed through the editor mode for the maps

*/

#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::data_structs::map_data::*; 
use crate::rendering::minimap::*;


// Simple states for the map_builder loop - only needed by the map_builder tool at the moment
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MapBuildState {
    #[default]
    RenderMap,
    LoadingMap,
    SavingMap,
    Drawing,
}

// Initializes behavior based on mouse cursor location and which button was pressed
pub fn mouse_behavior(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
    zoom: Res<ZoomLevel>,
    mut draw_line: Query<(&DragLine, &mut Transform, &mut Position, Entity)>,
){
    // Initialize camera and position info
    let (camera, camera_transform) = map_cam.single();
    let window = q_window.single();

    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Translate the cursor position into our map position (Based on where the map is currently located)
        let scale = zoom.zoom as f32;
        let rounded_positions = (world_position.x.round() + scale/2., world_position.y.round() + scale/2.);
        let loc_x = rounded_positions.0.rem_euclid(scale);
        let loc_y = rounded_positions.1.rem_euclid(scale);

        let mut start_x: f32 = 0.;
        let mut start_y: f32 = 0.;
        
        // Mouse is clicked - check if we're near a valid corner and spawn in the sprite if we are
        if mouse.just_pressed(MouseButton::Left) {
            if(loc_x / scale < 0.2 || loc_x / scale > 0.8) &&
              (loc_y / scale < 0.2 || loc_y / scale > 0.8)
            {
                // We are close enough to 'snap' to a corner
                start_x = world_position.x;
                start_y = world_position.y;

                // Spawn the wall sprite with DragLine component
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(start_x.round(), start_y.round()).extend(5.0),
                        scale: Vec3::new(0., 1.5, 1.),
                        rotation: Quat::from_rotation_z(0.),
                        ..default()
                    },
                    ..Default::default()
                }, 
                // RenderLayers::layer(2), // TODO - fix renderLayers
                DragLine,
                Position{x:start_x as i32, y:start_y as i32, z:0},
                ));
            }
        }

        if mouse.pressed(MouseButton::Left) {
            // Transform the DragLine sprite (within limits)
            for(_drag, mut transf, mut pos, _ent) in draw_line.iter_mut(){
                let norm_pts = (world_position.x - pos.x as f32, world_position.y - pos.y as f32);
                let theta = norm_pts.1.atan2(norm_pts.0);
                // Compute dist value - we can assume it will always be a right angle triangle
                let dist = ((world_position.x - pos.x as f32).abs().powi(2) + (world_position.y - pos.y as f32).abs().powi(2)).sqrt();

                transf.scale.x = dist;
                transf.translation.x = (pos.x as f32 + world_position.x)/ 2.;
                transf.translation.y = (pos.y as f32 + world_position.y)/ 2.;
                transf.rotation = Quat::from_rotation_z(theta);
            }
        }

    }
}

pub fn render_complete(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
){
    next_state.set(MapBuildState::Drawing);
}


pub fn draw_wall(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
){

}


pub fn delete_wall(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
){

}