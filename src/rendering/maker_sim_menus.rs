/* Game Menus 
    Contains methods, initialization, and layout for the "Princess Maker" style menus/loop
 */


 // Typically, the player will always start within this menu/state when not on an expedition.

use std::thread::spawn;

use bevy::prelude::*;
use sickle_ui::prelude::*;

use super::sickle_widgets::radio_group::*;

#[derive(Component, Debug)]
pub struct MorningSwitch;


#[derive(Component, Debug)]
pub struct AfternoonSwitch;

 /*  ========================================================
        UI Layout / Design
    ========================================================
    State - MakerMenu
    Sub states - Sub-pages of Maker Menu (Adjust training/actions)

    
*/

// Draws the full Maker Menu (Takes up the entire screen / region)
pub fn draw_makermenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    // Need to figure out how to spawn an ID/entity value...
    let radio_group_a = commands.spawn(
        ButtonRadioGroup { selected: Some((0))}
    ).id();



    // General layout - Split the screen into a top row and two main sections
    commands.ui_builder(UiRoot)
        .column(|column|{
            // Title section (Text in a row/container)
            column.row(|row|{
                row.container(
                    (
                        TextBundle{
                            text: Text::from_section("Ye Olde Planne", TextStyle{..default()}),
                            ..Default::default()
                        }
                    ), |container|{}
                );
            });


            // TODO - Figure out why the radio buttons aren't showing up - doesn't seem to require extra_menu?
            // Might be import-related, or just something I'm overlooking.
            
            // Upper section - group of buttons with Radio behavior
            column.row(|row|{
                row.menu_bar(|bar|{
                    bar.extra_menu(|extra|{
                        extra.radio_group(vec!["PHYS", "IDEA", "GUILD"], 0, false)
                        .insert(MorningSwitch);
                    });
                });
                
            }).style()
                .height(Val::Percent((49.)))
            ;

            

            // Lower section - group of buttons with Radio behavior
            column.row(|row|{
                row.button_radio_group(vec!["PHYS", "IDEA", "GUILD"], 0, false, radio_group_a)
                .insert(AfternoonSwitch);
            }).style()
                .height(Val::Percent((49.)))
            ;

            column.row(|row|{
                row.button_radio_group(vec!["GUILE", "DESPAIR", "ANGER"], 0, false, radio_group_a)
                .insert(AfternoonSwitch);
            }).style()
                .height(Val::Percent((49.)))
            ;
        })
        .insert(UiContextRoot)
        .style()
        .width(Val::Percent((100.)))
        .height(Val::Percent((100.)))
        ;

}

// Draws the 'Categories' view - designed to be used twice
// Investigate if I can pass the UiBuilder's <Entity> reference and just plop the same snippet
// Although I do need to include "Top or bottom" so I know which button goes where
pub fn draw_maker_actions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

}

/*  ========================================================
        Core Logic and Actions for buttons
    ========================================================
    
*/

// Configuration Sub-menu
// To make it easy, let's just have the same menu show up twice - checkbox behavior
// List of actions, and some actions have a sub-menu to optionally refine it further?


// Confirmation button - Takes current state of checkboxes and passes onto next state (ExecuteDay)
