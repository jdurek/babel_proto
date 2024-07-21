/*
  Minimap Builder - a helper tool used to help visualize a BaseMap struct and edit it as neededs
*/

#![allow(unused)]

use bevy::render::{camera, view::RenderLayers};
use sickle_ui::{prelude::*, SickleUiPlugin};

mod prelude {
    pub use bevy::prelude::*;
    pub use serde::*;
    pub use babel_proto::data_structs::map_data::*; 
    pub use babel_proto::rendering::minimap::*;
    pub use babel_proto::rendering::minimap_edit::*;
    pub use babel_proto::states::*;
}

use prelude::*;


#[derive(Component)]
struct MapCamera;

fn camera_setup(mut commands: Commands){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    
    // Camera starts pointed at 0,0 coordinate (Middle of screen)
    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn((camera, MapCamera, RenderLayers::from_layers(&[0, 2])));
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
                title: "Map Builder".to_string(),
                resolution: (1024 as f32, 720 as f32).into(),  // TODO - change this later for custom resolution (Or update it on the fly)
                ..Default::default()
            }),
            ..Default::default()
        }))
    .add_plugins(SickleUiPlugin)

    .add_systems(Startup, camera_setup)
    .add_systems(Startup, init_resources)
    .add_systems(Startup, draw_mb_menu)

    .init_state::<MapState>()
    .init_state::<MapBuildState>()

    .add_plugins(mb_gui_plugin)

    // Despawn previous render and build new one
    // .add_systems(OnEnter(MapBuildState::RenderMap))
    .add_systems(Update, (draw_2d_map, render_complete).run_if(in_state(MapBuildState::RenderMap)))
    .add_systems(Update, mouse_behavior.run_if(in_state(MapBuildState::Drawing)))
    .add_systems(Update, menu_button_system)
    .add_systems(Update, menu_action)

    
    // .add_systems(Update, text_summary)
    
    .run();
}

