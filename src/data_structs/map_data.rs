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