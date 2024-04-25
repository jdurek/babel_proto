/*  map_data.rs
  Defines the map structs and values
  Handles fetching map data and storing into the project's map_data struct
  Right now, maps are planned to be stored as a JSON through Serde
*/

use std::collections::HashMap;
use bevy::prelude::*;
use serde::*;


// Basic wall enum (Can modify the definition of Walls later)
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum Wall {
    #[default]
    NoWall,
    Solid,
    Transparent,
    Door,
    Shortcut,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Tile {
   pub walls:  [bool; 4],
}

// Basic map definition
#[derive(Serialize, Deserialize, Clone)]
pub struct MapBase {
    pub dim_x: i32,
    pub dim_y: i32,
    tiles: Vec<Tile>,
    walls: Vec<Wall>,
    textures: HashMap<String, String>,
    // default_textures: TextureAtlas,
}
impl MapBase { 
    // Helper functions to make accessing and handling data easier

    // Given a X/Y coordinate, update the tile data at said coordinate
    fn update_tile(&mut self, t: Tile, x: i32, y: i32){
        let index = self.get_tile_index(x, y);
        self.tiles[index as usize] = t;
    }
    // Given a wall index (Use helper function), update the wall value
    fn update_wall(&mut self, w: Wall, index: i32) {
        self.walls[index as usize] = w;
    }

    // Given a coordinate, obtain the tile index
    fn get_tile_index(&self, x: i32, y: i32) -> i32 {
        y * self.dim_y + x
    }

    // Given a coordinate, obtain the 4 wall indexes
    fn get_wall_index(&self, x:i32, y:i32) -> [i32; 4] {
        let bottom = y * (self.dim_y + self.dim_x + 1) + x;
        let left = y * (self.dim_y + self.dim_x + 1) + self.dim_x + x;
        let right = left + 1;
        let top = (y + 1) * (self.dim_y + self.dim_x + 1) + x;

        [bottom, left, right, top]
    }
}

// Global access to current map - only 1 map is loaded at a time
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct CurrMap {
    // Map Data
}
impl CurrMap{
    // Load Function
    // Save Function
    // Read Function (If needed - ownership on resources is funny)
    // Edit Function 
}


// Global access to Town Map - mostly to cache town and make it easier to update anytime (For example - giving NPCs a schedule)
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct TownMap {
    // Underlying Map data
    // NPC/Shop Data
    // Time Data
}
impl TownMap{
    // Load Function
    // Save Function
    // Read Function (If needed - ownership on resources is funny)
    // Edit Function 
    // Tick Function (Town will keep doing it's own thing)
    // Set Time (Could have townies scripted to do certain behaviors at times, so this helps to set it when we return to town)
}