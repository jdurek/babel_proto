/*  States logic module
  This module is intended to centralize the main states and functions available within a given state.
  It will often need to pull from data_structs and rendering files to perform certain actions

*/

// Most of the logic is performed within the specific sub-crates
// This module mainly serves as a central access to the different game states and how the overall logic is attempted to be constructed. 

#![allow(unused)]

use bevy::prelude::*;
use serde::*;

pub mod map_state;

// States to represent which map type is loaded - mainly to enable/disable certain checks like world-map specific events
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapState {
    #[default]
    TownMap,
    WorldMap,
    InstanceMap,
    DebugMap,
}

// States to represent the gameplay state the player is in
// Notes - Shop is a 'derivative' of the menu, it'll be largely the same, just with some differences
// Dialogue and and Cutscene are also somewhat similar, cutscene just supports a bit more (eg, not just text, but videos/actions/voices ig) later on
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Exploration,
    Battle,
    Menu,
    Shop,
    Dialogue,
    Cutscene,
    Loading,
}