/*
  game.rs - Loads main game, performs loading steps, etc... 
  The eventual goal is for calling this bin to functionally be like calling a .exe for the game. 

  During development, this section may skip some loading steps and go straight to scenarios, although I plan to split the 'straight to scenario' to a different file later. 
*/

#![allow(unused)]

use std::thread::spawn;

use bevy::render::{camera, view::RenderLayers};


mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    pub use babel_proto::data_structs::map_data::*; 
    pub use babel_proto::rendering::minimap::*;
    pub use babel_proto::rendering::first_person::*;
    pub use babel_proto::rendering::debug_camera::*;
    pub use babel_proto::states::*;
}

use prelude::*;

fn main(){

}