/*  data_parsing module
  This module is intended to centralize external data access and parsing functions
  Specifically, SQLite queries and JSON files
*/

// Most of the logic is performed within the specific sub-crates
// This module mainly serves as a central access to data-parsing

use bevy::prelude::*;
use serde::*;

pub mod dialogue_data;
pub mod map_data;
pub mod player_data;

pub use map_data::*;
// pub use dialogue_data::*;
// pub use player_data::*;

