/*
  idea_maker.rs - Binary file for just testing the "Princess Maker" GUI aspects and making sure stages change correctly

  Goal is for the game state to always be printed to the UI, so that we can see which state we're in, that it changes correctly, etc...
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
    pub use babel_proto::rendering::maker_sim_menus::*;
}

use prelude::*;
use sickle_ui::{prelude::UiRoot, SickleUiPlugin, 
  theme::{PseudoTheme, Theme}, widgets::inputs::radio_group::RadioButton};

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

// Theme override method (Temp location while testing)
// Seems promising so far - just need to figure out how much the override involves.
fn theme_setup(
  // q_root: Query<Entity, With<UiRoot>>, // TODO - figuyrue out the query section here
  mut commands: Commands) 
{
  // let root = q_root.get_single().unwrap();

  // Theme for selected radio button (Includes focus/highlight)
  let radio_sel = PseudoTheme::<RadioButton>::deferred(None, |style_builder, data| {
    style_builder.background_color(Color::BLACK);
  });

  // Theme for unselected radio button
  let radio_unsel = PseudoTheme::<RadioButton>::deferred(None, |style_builder, data| {
    style_builder.background_color(Color::BLACK);
  });


  // commands.entity(root).insert((Theme::<RadioButton>::new(vec![radio_sel, radio_unsel])));

}

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
    .add_plugins(SickleUiPlugin)
    
    .add_systems(Startup, (draw_makermenu, camera_setup))

    // Trigger loading on global attributes, backend setup
    // .add_systems()

    // Initialize our global states and sub-systems (Plugins)

    .run();
}