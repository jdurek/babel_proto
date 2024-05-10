/*  minimap.rs
    This is a 2d rendering of the map data (And how map editor works)
    Since it's a 2d camera, it can be manipulated separately and do it's own things.


*/

#![allow(unused)]

use bevy::prelude::*;
use crate::data_structs::map_data::*; 

// Function to render a map JSON to a 2D camera
pub fn draw_2d_map_from_json(mut commands: Commands, map: MapBase){

}

// Function to render the current map to a 2D camera
pub fn draw_2d_map(mut commands: Commands, map: Res<CurrMap>){

}

// Function to toggle which rendering is used (basically a map style toggle)
// Corner, large-corner, transparent overlay, no minimap, etc...

