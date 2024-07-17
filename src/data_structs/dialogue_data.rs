/*  dialogue_data.rs
    Defines how a dialogue is represented, and how to store/access it.
    Current idea is to use JSON struct - Or I could use both, where certain dialogue that gets accessed more often is in the DB (Due to JSON access speeds)

*/
#![allow(unused)]

// Thinking of using YarnSpinner - https://docs.yarnspinner.dev/using-yarnspinner-with-rust/quick-start

use bevy::prelude::*;
use serde::*;
use bevy_yarnspinner::prelude::*;

use bevy_yarnspinner_example_dialogue_view::prelude::*;

// Creation of a plugin for initializing our dialogue system
// Loads in YarnSpinner (Grabs all .yarn files), and a dialogue view to render it in
fn dialogue_plugin(app: &mut App){
    app.add_plugins((
        YarnSpinnerPlugin::new()
        ,ExampleYarnSpinnerDialogueViewPlugin::new()
    ))
    
    ;

}