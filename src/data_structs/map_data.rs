/*  map_data.rs
  Defines the map structs and values
  Handles fetching map data and storing into the project's map_data struct
  Right now, maps are planned to be stored as a JSON through Serde
*/
#![allow(unused)]

use std::collections::HashMap;
use bevy::prelude::*;
use serde::*;

// For serializer/deserializer - 
use std::fs::*;
use std::io::BufReader;

// TODO - figure out if I want to move this component elsewhere, since it's not exclusive to maps
// Note - This requires Eq and Hash for the HashMap to work
#[derive(Component, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ZoomLevel{
    pub zoom: i32,
}

// Basic wall enum (Can modify the definition of Walls later)
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum WallState {
    #[default]
    NoWall,
    Solid,
    Transparent,
    Door,
    Shortcut,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Wall {
    pub state: WallState,
    pub passable: bool,        // Tiles have a shortcut to this - use in conjunction with WallState on certain things like Locked doors?
    // TODO - figure out how I want to attach certain behaviors like shortcuts being one-way until used once, or locked doors and which key goes to it?
    //      Was thinking components can help, but I need to be able to make those components, and update the map data walls when we reach that point? Needs brainstorming.
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
    pub tiles: Vec<Tile>,
    pub walls: Vec<Wall>,
    // pub textures: HashMap<String, String>,
    // default_textures: TextureAtlas,
}
impl MapBase { 
    // Initialization function - creates vectors with blank tiles and no walls
    pub fn new(width: i32, height: i32) -> MapBase {
        MapBase { 
            dim_x: (width), 
            dim_y: (height), 
            tiles: (vec![Tile {walls:[false,false,false,false],}; (width* height) as usize]), 
            walls: (vec![Wall {state: WallState::NoWall, passable:false}; ((width+1)* height + (height+1) * width) as usize]),
        }
    }

    // Helper functions to make accessing and handling data easier
    pub fn get_tile(&self, x: i32, y:i32) -> Tile {
        self.tiles[self.get_tile_index(x, y) as usize]
    }

    // Given a X/Y coordinate, update the tile data at said coordinate
    pub fn update_tile(&mut self, t: Tile, index: usize){
        self.tiles[index as usize] = t;
    }
    // Given a wall index (Use helper function), update the wall value
    pub fn update_wall(&mut self, w: Wall, index: usize) {
        self.walls[index] = w;
        println!("Wall inserted at index {}", index);
    }

    // Given a single coordinate, obtain the tile index - if it's out of bounds, return -1
    pub fn get_tile_index(&self, x: i32, y: i32) -> i32 {
        if x >= self.dim_x || y >= self.dim_y || x < 0 || y < 0 {return -1;}
        (y * self.dim_x + x) as i32
    }

    // Given a single tile coordinate, obtain the 4 wall indexes (usize)
    pub fn get_walls_from_tile_index(&self, x:i32, y:i32) -> [usize; 4] {
        let bottom = y * (self.dim_y + self.dim_x + 1) + x;
        let left = y * (self.dim_y + self.dim_x + 1) + self.dim_x + x;
        let right = left + 1;
        let top = (y + 1) * (self.dim_y + self.dim_x + 1) + x;

        [bottom as usize, left as usize, right as usize, top as usize]
    }

    // Given a line (two coordinates) of length 1, determine which tiles are adjecent to it (1-2 tiles)
    pub fn get_tiles_from_line(&self, x1:i32, y1: i32, x2: i32, y2: i32) -> Result<[i32;2], String>{
        // Basic validation - 
        if (x1 - x2 + y1 - y2).abs() != 1 {
            // Line length is not equal to 1, cannot use this line
            return Err(String::from("Line length is not 1 - cannot obtain valid tiles"))
        }
        if self.coordinate_validator(x1, y1) == false || self.coordinate_validator(x2, y2) == false {
            // One of the coordinates is out of bounds, the line is invalid
            return Err(String::from("One or more parts of the line is not within expected coordinate region"))
        }

        // Validation passed, can now determine which tile(s) are adjecent to our line
        let x_diff = x1 - x2;
        let y_diff = y1 - y2;
        let mut index = [0, 0];

        match(x_diff, y_diff) {     // Always uses bottom left corner for determining index of wall
            (-1,0) => {     // Rightward line - use x1, y1 as index
                if y1 == 0 {    // Line is at bottom edge of the map - 1 tile does not exist
                    index[0] = -1;
                }
                else {
                    index[0] = self.get_tile_index(x1, y1-1);
                }
                if y1 == self.dim_y {   // Line is at top edge of map
                    index[1] = -1;
                }
                else {
                    index[1] = self.get_tile_index(x1, y1);
                }
            }
            (1, 0) => {     // Leftward line - use x2, y2 as index
                if y1 == 0 {    // Line is at bottom edge of the map - 1 tile does not exist
                    index[0] = -1;
                }
                else {
                    index[0] = self.get_tile_index(x2, y2-1);
                }
                if y1 == self.dim_y {   // Line is at top edge of map
                    index[1] = -1;
                }
                else {
                    index[1] = self.get_tile_index(x2, y2);
                }
            }
            (0,-1) => {     // Upward line - use x1, y1 as index
                if x1 == 0 {    // Line is at left edge of the map
                    index[0] = -1;
                }
                else {
                    index[0] = self.get_tile_index(x1-1, y1);
                }
                if x1 == self.dim_x {   // Line is at right edge of map
                    index[1] = -1;
                }
                else {
                    index[1] = self.get_tile_index(x1, y1);
                }
            }
            (0, 1) => {     // Downward line - use x2, y2 as index
                if x1 == 0 {    // Line is at left edge of the map
                    index[0] = -1;
                }
                else {
                    index[0] = self.get_tile_index(x2-1, y2);
                }
                if x1 == self.dim_x {   // Line is at right edge of map
                    index[1] = -1;
                }
                else {
                    index[1] = self.get_tile_index(x2, y2);
                }
            }
            _ => {          // Error case - line is not equal to 1
                // Should be impossible to reach this branch
                return Err(String::from("Match Statement hates your logic"))
            }
        }

        Ok(index)
    }

    // Given a line (two coordinates) of length 1, determine which wall index this line is
    pub fn get_wall_from_line(&self, x1:i32, y1: i32, x2: i32, y2: i32) -> Result<usize, String>{
        // Basic validation - 
        if (x1 - x2 + y1 - y2).abs() != 1 {
            // Line length is not equal to 1, cannot use this line
            return Err(String::from("Line length is not 1 - cannot obtain valid tiles"))
        }
        if self.coordinate_validator(x1, y1) == false || self.coordinate_validator(x2, y2) == false {
            // One of the coordinates is out of bounds, the line is invalid
            return Err(String::from("One or more parts of the line is not within expected coordinate region"))
        }

        // Validation passed, can now compute the wall index from the coordinates
        let (x_diff, y_diff) = (x1-x2, y1-y2);
        let mut index = 0;

        match(x_diff, y_diff) {     // Always uses bottom left corner for determining index of wall
            (-1,0) => {     // Rightward horizontal - use x1, y1 as index
                index = (2 * self.dim_x + 1) * y1 + x1;
            }
            (1, 0) => {     // Leftward horizontal - use x2, y2 as index (Or just subtract 1 from x1)
                index = (2 * self.dim_x + 1) * y1 + (x1 -1);
            }
            (0,-1) => {     // Upward vertical - use x1, y1 as index
                index = (2 * self.dim_x + 1) * y1 + x1 + self.dim_x;
            }
            (0, 1) => {     // Downward vertical - use x2, y2 as index (Or just subtract 1 from y1)
                index = (2 * self.dim_x + 1) * (y1-1) + x1 + self.dim_x;
            }
            _ => {          // Error case - line is not equal to 1
                // Should be impossible to reach this branch
                return Err(String::from("Match Statement hates your logic"))
            }
        }

        Ok(index as usize)
    }

    // coordinate validation - basically make sure the provided coordinate fits within the map's bounds
    // validation includes positions where walls may exist, (EG, outer edge) - may want to have a separate validator for tiles and walls due to how walls are implemented. 
    fn coordinate_validator(&self, x:i32, y:i32) -> bool {
        if x < 0 || y < 0 || x > self.dim_x || y > self.dim_y {
            return false
        }
        true
    }

    

}

// Global resource access to current map JSON data - only 1 map is loaded at a time
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct CurrMap {
    // Map Data (JSON), READ_ONLY
    pub map_data: MapBase,
    // Player Data (JSON) - basically an overlay of the map data, to account for how much the player's explored, certain triggers like changes in terrain, etc...
    // Map struct (Position to Entity), for quicker lookup if we need to update/edit a tile entity
    pub entity_lookup: HashMap<Position, Entity>,
}
impl CurrMap{
    // Initialization function
    pub fn new(base_map: MapBase) -> CurrMap{
        CurrMap {
            map_data: base_map,
            entity_lookup: HashMap::new(),
        }
    }
    // Load Function (Loads in JSON into map_data, creates the entities, and populates the position lookup)
    pub fn load_from_json(path: String) -> CurrMap {

        let file = File::open(path).unwrap();
        let rdr = BufReader::new(file);

        // TODO - plug this into error-handling so we can recover/record the whoopsie
        serde_json::from_reader(rdr).unwrap()
        
    }

    // Save Function - How do we define the save paths and filenames? 
    
    // Read Function (If needed - ownership on resources is funny)

    // Edit Function - Affects player data and entities. 
    // Likely won't see much use, as I could just edit the entity directly
}

// Due to how entities are accessed, the Map for position to entities can't be defined in impl CurrMap
// The following functions are intended to be used only for CurrMap setup. 

// Populates the Map struct (Position to Entity) linking for quicker lookup of which entity to check when checking the current tile
// TODO - figure out the best way to handle the references created in this, and ensue ownership doesn't bite me
pub fn create_position_lookup(
    mut commands: Commands, 
    mg: Res<CurrMap>,
    query: Query<Entity, With<LoadedMap>>,

){
    // Initialize a basic hashmap so we can just feed in the Position and link it to an entity

    // Iterate over all results in the query and fetch the Position Component, then write to the CurrMap resource's hashmap the lookup (Entity address in mem)
    // TODO - refine the query to be LoadedMap, and Position - 
    //  Loaded Map contains tiles, walls, sprites, etc...
    for tile in &query{
        
    }
    // for y in 0..mg.map_data.dim_y {
    //     for x in 0..mg.map_data.dim_x {
    //         // 
    //     }
    // }
}


// Global resource access to Town Map JSON data - mostly to cache town and make it easier to update anytime (For example - giving NPCs a schedule)
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct TownMap {
    // Underlying Map data
    map_data: MapBase,
    // Player Data / Progress (Town changes based on progress)
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


#[derive(Component)]
pub struct LoadedMap;

#[derive(Component)]
pub struct LoadedTownMap;
