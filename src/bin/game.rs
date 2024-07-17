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
  // Main app flow - 
  App::new()

    // Initializes the window of the game (From default or settings file)
    // Eventual TODO - configure so resolution of window is pulled from config file
    .add_plugins(DefaultPlugins
      .set(WindowPlugin{
          primary_window: Some(Window{ 
              title: "Ideation".to_string(),
              resolution: (1024 as f32, 720 as f32).into(),  // TODO - change this later for custom resolution (Or update it on the fly)
              ..Default::default()
          }),
          ..Default::default()
      }))

    // Trigger loading on global attributes, backend setup
    // .add_systems()

    // Initialize our global states and sub-systems (Plugins)

    ;
}