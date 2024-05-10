/*  dialogue_data.rs
    Defines how a dialogue is represented, and how to store/access it.
    Current idea is to use JSON struct - Or I could use both, where certain dialogue that gets accessed more often is in the DB (Due to JSON access speeds)

*/
#![allow(unused)]

use bevy::prelude::*;
use serde::*;
use sqlite::*;