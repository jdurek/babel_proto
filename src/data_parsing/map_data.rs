/*  map_data.rs
  Handles fetching map data and storing into the project's map_data struct
  Right now, maps are planned to be stored as a JSON through Serde
*/

use bevy::prelude::*;
use serde::*;