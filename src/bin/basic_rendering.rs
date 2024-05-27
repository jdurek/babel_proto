/*
  basic_rendering.rs - A bin file designed to do some minimal rendering testing (Basically show that it's loading in and we're able to move around the env)
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


#[derive(Component)]
struct ViewCamera;

fn camera_setup(mut commands: Commands){
    // TODO - adjust the 3D camera initialization, as this'll be the first time using this camera variety
    // let mut camera = Camera3dBundle::default();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10., -12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        FlyCamera{..default()},
    ));
    

    
    
    // commands.spawn((camera, ViewCamera));
}

// Simply populates the CurrMap resource with a default map of 16x16 - can also load a default one from memory here if desired. 
fn init_resources(mut commands: Commands){
    let base_map = MapBase::new(16,16);
    let curr_map = CurrMap::new(base_map);
    let zoom = ZoomLevel { zoom: 16 };
    let center = Center {x: -64., y: -64.};
    commands.insert_resource(curr_map);
    commands.insert_resource(zoom);
    commands.insert_resource(center);
}



fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window{ 
                title: "Basic_Render PoC".to_string(),
                resolution: (1024 as f32, 720 as f32).into(),  // TODO - change this later for custom resolution (Or update it on the fly)
                ..Default::default()
            }),
            ..Default::default()
        }))
    .add_systems(Startup, camera_setup)
    .add_systems(Startup, init_resources)

    .insert_state(MapState::DebugMap)

        
    .add_plugins(DebugCamPlugin)

    // Despawn previous render and build new one
    // TODO - move this logic into the states folder to clean up this section
    .add_systems(OnEnter(MapState::DebugMap), (render_full_map))
    .add_systems(OnEnter(MapState::TownMap), (render_full_map))
    .run();
}

