/*  rendering module
  This module is intended to centralize all methods that generate sprite entities across the game

*/

// Most of the logic is performed within the specific sub-crates
// This module mainly serves as a central access to data-parsing

#![allow(unused)]

use bevy::prelude::*;
use serde::*;

pub mod dialogue;
pub mod first_person;
pub mod minimap;
pub mod game_menus;
pub mod minimap_edit;

pub use minimap::*;

