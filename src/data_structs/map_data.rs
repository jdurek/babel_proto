/*  map_data.rs
  Defines the map structs and values
  Handles fetching map data and storing into the project's map_data struct
  Right now, maps are planned to be stored as a JSON through Serde
*/

use bevy::prelude::*;
use serde::*;

// Basic map definition
#[derive(Serialize, Deserialize, Copy, Clone)]

// Global access to current map - only 1 map is loaded at a time
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct CurrMap {

}
impl CurrMap{
    // Load Function
    // Save Function
    // Read Function (If needed - ownership on resources is funny)
    // Edit Function 
}


// Global access to Town Map - mostly to cache town and make it easier to edit on the fly
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