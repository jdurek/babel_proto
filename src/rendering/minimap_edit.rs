/*  minimap_edit.rs
    Additional functionality related to the minimap, but only when accessed through the editor mode for the maps

*/

#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::data_structs::map_data::*; 
use crate::rendering::minimap::*;


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
}

#[derive(Component)]
pub struct SelectedOption;

// Logic for handling all mouse input during map builder's main drawing loop
pub fn mouse_input(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>, 
    map_cam: Query<(&Camera, &GlobalTransform)>,
    mut map: ResMut<CurrMap>,
    zoom: Res<ZoomLevel>,
    mut draw_line: Query<(&DragLine, &mut Transform, &mut Position, Entity)>,
    mut center: ResMut<Center>,
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
    mut center: ResMut<Center>,
){
    // Initialize camera and position info
    let (camera, camera_transform) = map_cam.single();
    let window = q_window.single();

    
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Translate the cursor position into our map position (Based on where the map is currently located)
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
                // We are close enough to 'snap' to a corner
                start_x = world_position.x;
                start_y = world_position.y;

                // Spawn the wall sprite with DragLine component
                commands.spawn((SpriteBundle{
                    sprite: Sprite { color: Color::ANTIQUE_WHITE, custom_size: (Some(Vec2::new(1.0,1.0))), ..Default::default() },
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


/*
    GUI Menu section - This is mainly for rendering and interactivity of the GUI menu, along with defining a few constants. 
*/

const NORMAL_BUTTON: Color = Color::GRAY;
const HOVERED_BUTTON: Color = Color::DARK_GRAY;
const HOVERED_PRESSED: Color = Color::DARK_GREEN;
const PRESSED_BUTTON: Color = Color::GREEN;


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

    // Spawns in the menu - Root contains N buttons, button nodes contain text and possibly images
    commands
        .spawn(NodeBundle{
            style: Style {
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::SEA_GREEN.into(),
            ..Default::default()
        })
        .with_children(|f|{
            f.spawn(NodeBundle
                {
                    ..Default::default()
                }
            ).with_children(|f|{f.spawn(NodeBundle{..Default::default()});});
        })
           
    ;



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

// Plugin for handling state-changes triggered by the menu GUI
// Integrates 3 functions - save_gui when we enter SavingMap (on clicking the save button), save_complete to exit the state, and save_cleanup in case something needs to be handled
pub fn mb_gui_plugin(app: &mut App){
    app
        .add_systems(OnEnter(MapBuildState::SavingMap),save_gui)
        .add_systems(OnEnter(MapBuildState::SavingMap),save_complete.after(save_gui))
        .add_systems(OnExit(MapBuildState::SavingMap), save_cleanup)
    ;
}

pub fn save_gui(
    mut commands: Commands,
    map_data: Res<CurrMap>,
){
    // TODO - add RFD to cargo, so we can use Windows built-in file-browser for name/path navigation
}

pub fn save_cleanup(
    mut commands: Commands,
){
    // Currently empty - can adjust it to handle something like a 'Last Saved' notification or handling misc cleanup
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