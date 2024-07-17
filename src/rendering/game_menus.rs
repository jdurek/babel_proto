/* Game Menus 
    Contains methods, initialization, and layout of the game's various menus
 */

#![allow(unused_variables)]

use bevy::prelude::*;


// Each menu, due to differences, will need the following - 
// Rendering logic | Mouse controller (and list it can move in) | what each button does


/*  ========================================================
                MAIN MENU / TITLE SCREEN
    ========================================================
    State - MainMenu
    Sub states - TopMenu, New Game, Load Game, Options/Settings, Exit Game

    Overall layout is title banner, and 5 options to pick from.
    Defaults to Load Game selected, unless no save data is available.
*/

// RenderMenu function - Handles basic rendering
pub fn draw_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    // CSS styling, and the button actions
}


// Mouse/Controller behavior for changing menu selection - (Changes whichever is highlighted)


// Functions for handling the buttons (State/sub-state changes)
pub fn main_menu_action(
    // interaction_query: Query<>,
    // Substate resource for quick-swapping
){
    // Use minimap edit's for interaction in query, if interaction = pressed/clicked (depending on input), do a match to button.
}