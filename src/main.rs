use bevy::prelude::*;

mod data_structs;
pub use data_structs::*;

// Revamping the bevy project from scratch - going to try and focus more on doing subsystems one at a time, and combining them all later.


// Basic setup - loads in first camera (Which drives the window)
fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    
    commands.spawn(camera);


}


fn main() {
    
}