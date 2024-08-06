pub mod radio_group;


use bevy::prelude::*;
use sickle_ui::prelude::*;

use radio_group::ButtonRadioGroupPlugin;

pub struct CustomWidgetPlugin;

// Load in all our plugins created here
impl Plugin for CustomWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, WidgetLibraryUpdate.after(FloatingPanelUpdate))
            .add_plugins((
                ButtonRadioGroupPlugin
            ));
    }
}