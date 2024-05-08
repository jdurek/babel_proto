/*  first_person.rs
    Rendering pipeline for first-person view
    Style will be dungeon crawler (Etrian Odyssey, Labrynth of Refrain)

    Due to how this is envisioned, a 3D camera is necessary

    Handles the following actions - 
        Loads into memory current map (and/or what's visible)
        Movement (Forwards, Backwards, Left, Right)
        Turning (Left, Right)
 */


use bevy::prelude::*;

/*
    General flow - 
        Fetch the JSON resource, AND entities with the CurrMap resource (Or Town Map resource). Also fetch the coordinate map resource for current map
        Begin loading in sprites, walls, etc...
*/

// Render the current area - uses what's currently loaded in
pub fn render_region(mut commands: Commands, map_data: JSON) {
    // Iterate over each grid, and create a transparent 'cube'
    // Apply a texture to the floor, and to the walls based on the JSON/struct data

}


// Handle shifting of our 'coordinates' by moving the 3D camera
// Supports moving 4(6) ways - Left, Right, Forwards, Backwards (Up, Down)
// Up/Down will be implemented later, for now I'll just assume a perfect 2d plane
pub fn grid_movement(query: Query<3DCameraBundle>){
    // Fetch the camera with the query, and whatever movement was plugged in
    // Move the camera accordingly with a 'slide' motion
    
}


// Handle matching camera to player direction (Rotation)
// Should only be able to turn in 90 degree increments, but being able to do a 180 or 360 may be handy.
pub fn grid_rotation(query: Query<3DCameraBundle>){
    // Fetch the camera with query, and update the values to change the rotation accordingly.
    // This should be a sliding motion rather than a jump to a new coordinate. 
    
}