/*  data_struct module
  This module is intended to centralize the definition and manipulation of data structs
  Creating, Editing, Deleting should all be accessed through the setup here
*/

// Most of the logic is performed within the specific sub-crates
// This module mainly serves as a central access to data-parsing

#![allow(unused)]

use bevy::prelude::*;
use serde::*;

pub mod dialogue_data;
pub mod map_data;
pub mod player_data;

pub use map_data::*;
// pub use dialogue_data::*;
// pub use player_data::*;

