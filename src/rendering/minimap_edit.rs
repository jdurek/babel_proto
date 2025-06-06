/*  minimap_edit.rs
    Additional functionality related to the minimap, but only when accessed through the editor mode for the maps

*/

// #![allow(unused)]

use bevy::color::palettes::tailwind;
use bevy::ecs::world;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use sickle_ui::prelude::*;
use crate::data_structs::map_data::*; 
use crate::rendering::minimap::*;
use crate::states::MapState;

// Following are for file I/O (save/load)
use rfd::FileDialog;
use std::fs::*;
use std::io::*;
use std::path::Path;


// Simple states for the map_builder loop - only needed by the map_builder tool at the moment, which is why it's only kept in here
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MapBuildState {
    #[default]
    RenderMap,
    LoadingMap,
    SavingMap,
    Drawing,
}

// Cursor states to change the behavior of the mouse clicks
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MapCursorMode {
    #[default]
    Wall,   // Wall-drawing behavior
    Drag,   // Two ways to think of this - dragging stuff like mobs/entities, or sliding the map to change the 0,0 point
    Paint,  // Apply generic behavior to tile (Such as water, terrain type, etc...)
    Trait,  // Modify trait of existing object (EG, if wall exists, modify the type of wall)
}

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component, Debug)]
pub enum MBMenuButtonAction {
    New,
    Save,
    Load,
    Undo,
    Redo,
}

// Logic for handling all mouse input during map builder's main drawing loop
pub fn mouse_input(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
    zoom: Res<ZoomLevel>,
    mut draw_line: Query<(&DragLine, &mut Transform, &mut Position, Entity)>,
    mut center: Res<Center>,
){
    // First, check to see if the cursor position is on any menu buttons (Save, Load, Mode change)
    // If it is, handle it accordingly (Potentially hover-over, left-click, etc...)

    // Otherwise, check what mode we're in and handle the mouse behavior by throwing to helper functions that are meant to handle said modes
    // match cursorMode {
    //     MapCursorMode::Wall => {

    //     }
    //     MapCursorMode::Drag => {

    //     }
    //     _ => {

    //     }
    // }

}

// Original mouse_behavior
pub fn mouse_behavior(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
    zoom: Res<ZoomLevel>,
    mut draw_line: Query<(&DragLine, &mut Transform, &mut Position, Entity)>,
    mut center: Res<Center>,
    mut map_state: ResMut<NextState<MapBuildState>>,
){
    // Initialize camera and position info
    let (camera, camera_transform) = map_cam.single();
    let window = q_window.single();

    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Translate the cursor position into our map position (Based on where the map is currently located)
        // Due to a quirk, the corners are offset onto scale/2 grid, rather than scale grid

        // Rounded positions is used to check if the tile is valid (EG, within center and whatever other bounding was used)
        let scale = zoom.zoom as f32;
        let rounded_positions = (world_position.x.round() + scale/2., world_position.y.round() + scale/2.);
        let loc_x = rounded_positions.0.rem_euclid(scale);
        let loc_y = rounded_positions.1.rem_euclid(scale);

        let mut start_x: f32 = 0.;
        let mut start_y: f32 = 0.;
        
        // Mouse is clicked - check if we're near a valid corner and spawn in the sprite if we are
        // TODO - restrict it to only valid wall positions (Use Center, 0,0, and dim X/Y to bound this)
        if mouse.just_pressed(MouseButton::Left) {
            if(loc_x / scale < 0.2 || loc_x / scale > 0.8) &&
              (loc_y / scale < 0.2 || loc_y / scale > 0.8)
            {
                // We are close enough to 'snap' to a corner - so go ahead and snap it

                // start_x = world_position.x;
                // start_y = world_position.y;
                // println!("{},{}",start_x, start_y);

                start_x = ((world_position.x + scale / 2.) / scale).round() * scale - scale / 2.;
                start_y = ((world_position.y + scale / 2.) / scale).round() * scale - scale / 2.;
                // println!("{},{}",start_x, start_y);

                // Spawn the wall sprite with DragLine component
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color:  tailwind::NEUTRAL_500.into(), custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec2::new(start_x.round(), start_y.round()).extend(5.0),
                        scale: Vec3::new(0., 1.5, 1.),
                        rotation: Quat::from_rotation_z(0.),
                        ..default()
                    },
                    ..Default::default()
                }, 
                // RenderLayers::layer(2), // TODO - fix renderLayers
                DragLine,
                Position{x:start_x as i32, y:start_y as i32, z:0},
                ));
            }
        }

        if mouse.pressed(MouseButton::Left) {
            // Transform the DragLine sprite (within limits)
            for(_drag, mut transf, mut pos, _ent) in draw_line.iter_mut(){
                let norm_pts = (world_position.x - pos.x as f32, world_position.y - pos.y as f32);
                let theta = norm_pts.1.atan2(norm_pts.0);
                // Compute dist value - we can assume it will always be a right angle triangle
                let dist = ((world_position.x - pos.x as f32).abs().powi(2) + (world_position.y - pos.y as f32).abs().powi(2)).sqrt();

                // If dist value is about the same length as scale, check to see if we can snap the line and draw a new wall
                if dist > scale * 0.9 && dist < scale * 1.2 {
                    if  (loc_x / scale < 0.1 || loc_x / scale > 0.9) &&
                        (loc_y / scale < 0.1 || loc_y / scale > 0.9) 
                    {
                        // We can snap the line - perform that operation and update the walls where appropiate
                        let (old_x, old_y) = (pos.x as f32, pos.y as f32);

                        pos.x = (((world_position.x + scale / 2.) / scale).round() * scale - scale / 2.) as i32;
                        pos.y = (((world_position.y + scale / 2.) / scale).round() * scale - scale / 2.) as i32;

                        // println!("Pre-conv: Start: ({},{}) | End: ({},{})", old_x, old_y, pos.x, pos.y);
                        // Add the wall to our map (In walls and tiles.walls)
                        let start_pair = wall_coordinate_conv(center.as_ref(), zoom.zoom as f32, old_x, old_y);
                        let end_pair = wall_coordinate_conv(center.as_ref(), zoom.zoom as f32, pos.x as f32, pos.y as f32);

                        println!("Start: ({},{}) | End: ({},{})", start_pair.0, start_pair.1, end_pair.0, end_pair.1);

                        let new_wall = Wall{ state: WallState::Solid, passable: false };
                        let Ok(idx) = map.map_data.get_wall_from_line(start_pair.0, start_pair.1, end_pair.0, end_pair.1) 
                            else { return };
                        map.map_data.update_wall(new_wall, idx);

                        // Trigger re-rendering by shifting states (Or just re-entering itself, but that might not be supported)
                        map_state.set(MapBuildState::RenderMap);
                        
                    }
                }


                // Update the wall sprite - caps length to match current scaling of the map
                if dist < scale {
                    transf.scale.x = dist;
                    transf.translation.x = (pos.x as f32 + world_position.x)/ 2.;
                    transf.translation.y = (pos.y as f32 + world_position.y)/ 2.;
                }
                else {  
                    transf.scale.x = scale;
                    transf.translation.x = pos.x as f32 + (theta.cos() * scale)/ 2.;
                    transf.translation.y = pos.y as f32 + (theta.sin() * scale)/ 2.;
                }
                transf.rotation = Quat::from_rotation_z(theta);
            }
        }

    }


    // This section is outside of the IF block to account for the mouse being released while not in valid bounds
    if mouse.just_released(MouseButton::Left){
        for line in draw_line.iter(){
            commands.entity(line.3).despawn();
        }
    }
}

// Helper function - converts mouse cursor/screen position into minimap wall coordinates
pub fn wall_coordinate_conv(
    center: &Center,
    zoom: f32,
    cur_x: f32,
    cur_y: f32,
) -> (i32, i32){
    // To avoid rounding issues in the wall itself, we shift over by zoom/2 to the right (brings it in line to 0,0) 
    let x = ((cur_x - center.x + zoom/2.) / zoom).round();
    let y = ((cur_y - center.y + zoom/2.) / zoom).round();
    (x as i32, y as i32)
}

// Helper function - converts mouse cursor/screen position into minimap tile coordinates
pub fn coordinate_conv(
    center: &Center,
    zoom: f32,
    cur_x: f32,
    cur_y: f32,
) -> (i32, i32){
    let x = ((cur_x - center.x) / zoom).round();
    let y = ((cur_y - center.y) / zoom).round();
    (x as i32, y as i32)
}

/*
    GUI Menu section - This is mainly for rendering and interactivity of the GUI menu, along with defining a few constants. 
*/

const NORMAL_BUTTON: Srgba = tailwind::ZINC_500;
const HOVERED_BUTTON: Srgba = tailwind::ZINC_900;
const HOVERED_PRESSED: Srgba = tailwind::ZINC_50;
const PRESSED_BUTTON: Srgba = tailwind::ZINC_50;


// This function draws the Menu UI/UX
pub fn draw_mb_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
){
    // TODO - CSS shenagians to set it all up. Can reuse older one from other project as a starting point? 

    // Map Builder Button Styling: 
    let btn_style = Style{
        width: Val::Px(150.),
        height: Val::Px(50.),
        margin: UiRect::all(Val::Px(20.0)),
        align_items: AlignItems::Center,
        ..default()
    };
    let btn_icon_style = Style{
        width: Val::Px(30.),
        position_type: PositionType::Absolute,
        left: Val::Px(10.),
        ..default()
    };
    let btn_text_style = TextStyle{
        font_size: 20.0,
        color: Color::BLACK,
        ..default()
    };

    // Spawning in the menu with Sickle_UI to familiarize myself with it
    commands.ui_builder(UiRoot).column(|column|{
        // Title section (On it's own row or something?)
        column.container(
            (
                NodeBundle{
                    style: Style{
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("..\\src\\assets\\images\\MegaOrby.png"))
            ), |container|{

            });
        
        column.menu_item(MenuItemConfig {
            name: "SAVE MAP".into(),
            ..default()
        }).insert(MBMenuButtonAction::Save);
        column.menu_item(MenuItemConfig {
            name: "LOAD MAP".into(),
            ..default()
        })
        .insert(MBMenuButtonAction::Load)
        .style()
        ;
        
        

        column.menu(
            MenuConfig {
                name: "Menu".into(),
                alt_code: KeyCode::KeyM.into(),
                ..default()
            },
            |menu| {
                menu.menu_item(MenuItemConfig {
                    name: "SAVE MAP".into(),
                    ..default()
                });
                menu.menu_item(MenuItemConfig {
                    name: "LOAD MAP".into(),
                    ..default()
                });
                menu.menu_item(MenuItemConfig{
                    name: "NEW MAP".into(),
                    ..default()
                });
            }
        );


    })
    .style()
    // TODO - fix the assets folder (or pathing - this hard-path isn't great)
    // .image(ImageSource::Path(("..\\src\\assets\\images\\MegaOrby.png".into())))
    .background_color(tailwind::EMERALD_300.into())
    ;

    // Spawns in the menu - Root contains N buttons, button nodes contain text and possibly images
    // commands
    //     .spawn(NodeBundle{
    //         style: Style {
    //             flex_direction: FlexDirection::Column,      // Forces the children buttons into a column config
    //             align_items: AlignItems::Center,
    //             position_type: PositionType::Absolute,
    //             left: Val::Px(0.),
    //             top: Val::Px(0.),
    //             bottom: Val::Px(0.),
    //             ..Default::default()
    //         },
    //         background_color:  tailwind::EMERALD_300.into(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent|{
    //         // MB Menu Title

    //         // Save Button
    //         parent
    //         .spawn((ButtonBundle
    //             {
    //                 style: btn_style.clone(),
    //                 background_color: tailwind::EMERALD_300.into(),
    //                 ..Default::default()
    //             },
    //             MBMenuButtonAction::Save,
    //         ))
    //         .with_children(|parent| {
    //             parent.spawn(TextBundle::from_section("Save Map", btn_text_style.clone(),
    //             ));
    //         });

    //         // Load Button
    //         parent
    //         .spawn((ButtonBundle
    //             {
    //                 style: btn_style.clone(),
    //                 background_color: tailwind::EMERALD_300.into(),
    //                 ..Default::default()
    //             },
    //             MBMenuButtonAction::Load,
    //         ))
    //         .with_children(|parent| {
    //             parent.spawn(TextBundle::from_section("Load Map", btn_text_style.clone(),
    //             ));
    //         });
    //     })
           
    // ;



}

// This function handles button interactivity (Updating the colors based on the 4 consts and what's being done)
// TODO - relearn the selected option bit
pub fn menu_button_system(
    mut interact_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interact_query {
        *color = match(*interaction, selected) {
            // Match to the different interaction cases - need to define the colors used in advance
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn menu_action(
    interaction_query: Query<(&Interaction, &MBMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MapBuildState>>,
){
    for (interaction, menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed{
            // Match the button action to the logic - most of these will switch states
            match menu_action{
                MBMenuButtonAction::Save => {
                    println!("SAVING BUTTON PRESSED");
                    menu_state.set(MapBuildState::SavingMap);
                }
                MBMenuButtonAction::Load => {
                    println!("LOAD BUTTON PRESSED");
                    // menu_state.set(MapBuildState::LoadingMap);
                }
                MBMenuButtonAction::New => {
                    // Load a blank map (requires us to define the size in a prompt)
                    // menu_state.set(MapBuildState::NewMap);
                }
                MBMenuButtonAction::Undo => {

                }
                MBMenuButtonAction::Redo => {

                }
                _ => {
                    println!("Unidentified action: {:?}", menu_action);
                }
            }
        }
    }
}

// Plugin for handling state-changes triggered by the menu GUI
// Integrates 3 functions - save_gui when we enter SavingMap (on clicking the save button), save_complete to exit the state, and save_cleanup in case something needs to be handled
pub fn mb_gui_plugin(app: &mut App){
    app
        .add_systems(OnEnter(MapBuildState::SavingMap),save_map)
        .add_systems(OnEnter(MapBuildState::SavingMap),save_complete.after(save_map))
        .add_systems(OnExit(MapBuildState::SavingMap), save_cleanup)

        .add_systems(OnEnter(MapBuildState::LoadingMap), load_map)
        .add_systems(OnEnter(MapBuildState::LoadingMap), load_complete.after(load_map))
    ;
}

// Takes the current map in-memory and writes it to a JSON file on-disk. 
// Uses device's native GUI for saving (FileDialog) using RFD to interface with that
pub fn save_map(
    mut commands: Commands,
    map_data: Res<CurrMap>,
){
    println!("Attempting to save current map...");

    // Opens native GUI interface for saving a file with .json as accepted format
    let file = FileDialog::new()
        .add_filter("data", &["json"])
        .set_directory(std::env::current_dir().unwrap())
        .save_file();

    // Once the above is complete (Blocks until it's resolved), writes the MapData to file
    // let map_str = serde_json::to_string(map_data.as_ref());
    if let Some(route) = file {
        let file = File::create(route).unwrap();

        let mut writer = BufWriter::new(file);
        let w = serde_json::to_writer(&mut writer, map_data.as_ref());
        writer.flush();

        println!("Save complete!");
    }
    else{
        // User canceled action?
        println!("User canceled operation");
    }

}

pub fn save_cleanup(
    mut commands: Commands,
){
    // Currently empty - can adjust it to handle something like a 'Last Saved' notification or handling misc cleanup
}


// Takes a JSON the user provides from GUI (FileDialog) and loads it into CurrMap resource (Overwrites whatever's already in there)
pub fn load_map(
    mut commands: Commands,
    map_data: ResMut<CurrMap>,
){
    let file = FileDialog::new()
        .add_filter("data", &["json"])
        .set_directory(std::env::current_dir().unwrap())
        .pick_file();

    // User has chosen a file - attempt to open and read the JSON into our struct (with serde_json)
    if let Some(route) = file {
        let file = File::open(route).unwrap();
        let rdr = BufReader::new(file);

        let temp_map: CurrMap = serde_json::from_reader(rdr).unwrap();

        // Optional TODO: Do a quick prompt to the user to verify they want to load the map (and possibly show a preview?)
        
        // Insert the newly loaded CurrMap data into the resource ()
        commands.insert_resource(temp_map);

        println!("New map loaded!");
        // 
    }
}

// Debug/testing tool - displays misc information related to current minimap in editor screen
pub fn text_summary(mut commands: Commands,){
    let debug_text = "{}";
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..Default::default()
    };
    
    // Delete previous bundle if one was already created (Prevents this from spawning infinitely)

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(debug_text, text_style.clone())
                .with_justify(JustifyText::Center),
            ..default()
        },
    ));

}


/*
    State-changing functions - These functions simply change states to help trigger certain behaviors (On Enter and On Exit)
    
*/

// From Rendering state to Drawing state
pub fn render_complete(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
){
    next_state.set(MapBuildState::Drawing);
}

// From any state to Rendering
pub fn trigger_render(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
){
    next_state.set(MapBuildState::RenderMap);
}

// From SavingMap state to drawing state
pub fn save_complete(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
){
    next_state.set(MapBuildState::Drawing);
}

pub fn load_complete(
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapBuildState>>,
){
    // Trigger the re-rendering
    next_state.set(MapBuildState::RenderMap);
}

// Function to draw wall - needs Commands, coordinates - may also add the 'wall edit' to this, to keep the logic clumped together, but ownership will be fun. 
pub fn draw_wall(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
){

}


pub fn delete_wall(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
){

}