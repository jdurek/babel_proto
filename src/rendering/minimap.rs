/*  minimap.rs
    This is a 2d rendering of the map data (And how map editor works)
    Since it's a 2d camera, it can be manipulated separately and do it's own things.


*/

#![allow(unused)]

use bevy::prelude::*;
use crate::data_structs::map_data::{self, *}; 


#[derive(Component)]
pub struct DragLine;

#[derive(Component)]
pub struct MapCellSprite;

// Function to render a map JSON to a 2D camera
pub fn draw_2d_map_from_json(mut commands: Commands, map: MapBase){

}

// Function to render the current map to a 2D camera
pub fn draw_2d_map(mut commands: Commands, map: Res<CurrMap>, zoom: Res<ZoomLevel>){
    let grid_scale = zoom.zoom as f32;
    // This section draws out the grid (Tiles)
    for y in 0..map.map_data.dim_y {
        for x in 0..map.map_data.dim_x {
            // TODO - associate the sprite with our specific tile if we want to
            // map.map_data.get_tile(x, y);
            
            commands.spawn((SpriteBundle{
                sprite: Sprite { color: Color::TURQUOISE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                visibility: Visibility::Visible,
                transform: Transform {
                    translation: Vec2::new(x as f32 * grid_scale, y as f32 * grid_scale).extend(0.0),
                    scale: Vec3::new(grid_scale - 1., grid_scale - 1., 0.),
                    ..default()
                },
                ..Default::default()
            }, 
            MapCellSprite, 
            // RenderLayers::layer(2),
            ));
        }
    }

    // This section draws out the wall lines (Walls) - does 2 passes (One for horizontal walls, one for vertical)
    for x in 0..map.map_data.dim_x + 1 {
        for h in 0..map.map_data.dim_x {
            // Index in the wall vector is x + h*(x + dim_y + 1)
            let indx = (h + x*(map.map_data.dim_x + map.map_data.dim_y + 1)) as usize;
        }
    }
    
    for y in 0..map.map_data.dim_y {
        for v in 0..map.map_data.dim_y + 1 {
            // Index in the wall vector is dim_x + v + y*(x+y+1)
            let indx = (map.map_data.dim_x + v + y*(map.map_data.dim_x + map.map_data.dim_y + 1)) as usize;
        }
    }

}

// Function to toggle which rendering is used (basically a map style toggle)
// Corner, large-corner, transparent overlay, no minimap, etc...

