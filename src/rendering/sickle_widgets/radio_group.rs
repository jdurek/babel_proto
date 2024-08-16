use bevy::{prelude::*, ui::FocusPolicy};

use sickle_ui::prelude::*;

// seems prelude::* import negates the need for these lines
// use sickle_macros::UiContext;
// use sickle_ui_scaffold::prelude::*;

// use crate::widgets::layout::{
//     container::UiContainerExt,
//     label::{LabelConfig, UiLabelExt},
// };



/*
    Tweaked RadioGroup from Sickle_UI to form it's own widget
    This one clones all basic radio_group functionality, and aims to do the following - 
    1) Radio Group pattern may not be a single line or column - so the group ID needs to be provided as an arg
    2) Radio Group members (Same ID) may not have the same styling - clicking needs to be universal, but base style differs.
        I think this can be done with the theme setup, using components...
    3) Radio mark is disabled - it should look more like a button with radio behavior.

*/

pub struct ButtonRadioGroupPlugin;

impl Plugin for ButtonRadioGroupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ComponentThemePlugin::<ButtonRadioGroup>::default(),
            ComponentThemePlugin::<ButtonRadioButton>::default(),
        ))
        .add_systems(
            Update,
            (
                toggle_button_radio_button,
                update_button_radio_group_buttons,
                update_button_radio_button,
            ),
        );
    }
}

fn toggle_button_radio_button(
    mut q_button_radio_buttons: Query<(&mut ButtonRadioButton, &FluxInteraction), Changed<FluxInteraction>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_group: Query<&mut ButtonRadioGroup>,
) {
    for (mut radio_button, interaction) in &mut q_button_radio_buttons {
        if *interaction == FluxInteraction::Pressed {
            let mut changed = false;

            if radio_button.checked
                && radio_button.unselectable
                && keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
            {
                radio_button.checked = false;
                changed = true;
            } else if !radio_button.checked {
                radio_button.checked = true;
                changed = true;
            }

            if !changed {
                continue;
            }

            let Ok(mut radio_group) = q_group.get_mut(radio_button.group) else {
                continue;
            };

            radio_group.selected = if radio_button.checked {
                radio_button.index.into()
            } else {
                None
            };
        }
    }
}

fn update_button_radio_group_buttons(
    mut q_button_radio_buttons: Query<(&ButtonRadioGroup, &Children), Changed<ButtonRadioGroup>>,
    mut q_button_radio_button: Query<&mut ButtonRadioButton>,
) {
    for (radio_group, children) in &mut q_button_radio_buttons {
        for child in children {
            if let Ok(mut button) = q_button_radio_button.get_mut(*child) {
                // This is to avoid double triggering the change
                let checked = radio_group.selected == button.index.into();
                if button.checked != checked {
                    button.checked = checked;
                }
            }
        }
    }
}

fn update_button_radio_button(
    q_button_radio_buttons: Query<(Entity, &ButtonRadioButton), Changed<ButtonRadioButton>>,
    mut commands: Commands,
) {
    for (entity, radio_button) in &q_button_radio_buttons {
        commands
            .style_unchecked(radio_button.radiomark)
            .visibility(match radio_button.checked {
                true => Visibility::Inherited,
                false => Visibility::Hidden,
            });

        if radio_button.checked {
            commands
                .entity(entity)
                .add_pseudo_state(PseudoState::Checked);
        } else {
            commands
                .entity(entity)
                .remove_pseudo_state(PseudoState::Checked);
        }
    }
}

#[derive(Component, Debug, Reflect, UiContext)]
#[reflect(Component)]
pub struct ButtonRadioGroup {
    pub selected: Option<usize>,
}

impl Default for ButtonRadioGroup {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl DefaultTheme for ButtonRadioGroup {
    fn default_theme() -> Option<Theme<ButtonRadioGroup>> {
        ButtonRadioGroup::theme().into()
    }
}

impl ButtonRadioGroup {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, value: impl Into<Option<usize>>) {
        let selected = value.into();
        if self.selected != selected {
            self.selected = selected;
        }
    }

    pub fn theme() -> Theme<ButtonRadioGroup> {
        let base_theme = PseudoTheme::build(None, ButtonRadioGroup::primary_style);
        Theme::new(vec![base_theme])
    }

    fn primary_style(style_builder: &mut StyleBuilder) {
        style_builder.flex_wrap(FlexWrap::Wrap);
    }

    fn container() -> impl Bundle {
        (Name::new("Radio Group"), NodeBundle::default())
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ButtonRadioButton {
    pub index: usize,
    pub checked: bool,
    unselectable: bool,
    group: Entity,
    radiomark_background: Entity,
    radiomark: Entity,
    label: Entity,
}

impl Default for ButtonRadioButton {
    fn default() -> Self {
        Self {
            index: 0,
            checked: false,
            unselectable: false,
            group: Entity::PLACEHOLDER,
            radiomark_background: Entity::PLACEHOLDER,
            radiomark: Entity::PLACEHOLDER,
            label: Entity::PLACEHOLDER,
        }
    }
}

impl UiContext for ButtonRadioButton {
    fn get(&self, target: &str) -> Result<Entity, String> {
        match target {
            ButtonRadioButton::RADIOMARK_BACKGROUND => Ok(self.radiomark_background),
            ButtonRadioButton::RADIOMARK => Ok(self.radiomark),
            ButtonRadioButton::LABEL => Ok(self.label),
            _ => Err(format!(
                "{} doesn't exists for ButtonRadioButton. Possible contexts: {:?}",
                target,
                self.contexts()
            )),
        }
    }

    fn contexts(&self) -> Vec<&'static str> {
        vec![
            ButtonRadioButton::RADIOMARK_BACKGROUND,
            ButtonRadioButton::RADIOMARK,
            ButtonRadioButton::LABEL,
        ]
    }
}

impl DefaultTheme for ButtonRadioButton {
    fn default_theme() -> Option<Theme<ButtonRadioButton>> {
        ButtonRadioButton::theme().into()
    }
}

impl ButtonRadioButton {
    pub const RADIOMARK_BACKGROUND: &'static str = "RadiomarkBackground";
    pub const RADIOMARK: &'static str = "Radiomark";
    pub const LABEL: &'static str = "Label";

    pub fn theme() -> Theme<ButtonRadioButton> {
        let base_theme = PseudoTheme::deferred(None, ButtonRadioButton::primary_style);
        let checked_theme =
            PseudoTheme::deferred(vec![PseudoState::Checked], ButtonRadioButton::checked_style);
        Theme::new(vec![base_theme, checked_theme])
    }


    // Reminder to other devs - edit BOTH style sections
    fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .height(Val::Px(theme_spacing.areas.small))
            .justify_content(JustifyContent::Start)
            .align_items(AlignItems::Center)
            .margin(UiRect::horizontal(Val::Px(theme_spacing.gaps.small)))
            .background_color(Color::NONE);

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK_BACKGROUND)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .align_content(AlignContent::Center)
            .size(Val::Px(
                theme_spacing.inputs.radio_button.radiomark_outer_size,
            ))
            .margin(UiRect::all(Val::Px(theme_spacing.gaps.small)))
            .border(UiRect::all(Val::Px(
                theme_spacing.inputs.radio_button.border_size,
            )))
            .border_radius(BorderRadius::all(Val::Px(
                theme_spacing.inputs.radio_button.radiomark_outer_size,
            )))
            .visibility(Visibility::Inherited)
            .animated()
            .border_color(AnimatedVals {
                idle: colors.on(On::SurfaceVariant),
                hover: colors.on(On::Surface).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK_BACKGROUND)
            .animated()
            .background_color(AnimatedVals {
                idle: Color::NONE,
                hover: colors.accent(Accent::Primary).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK)
            .size(Val::Px(theme_spacing.inputs.radio_button.radiomark_size))
            .background_color(colors.on(On::Primary))
            .border_radius(BorderRadius::all(Val::Px(
                theme_spacing.inputs.radio_button.radiomark_size,
            )));

        let font = theme_data
            .text
            .get(FontStyle::Body, FontScale::Medium, FontType::Regular);
        style_builder
            .switch_target(ButtonRadioButton::LABEL)
            .margin(UiRect::px(
                theme_spacing.gaps.small,
                theme_spacing.gaps.medium,
                0.,
                0.,
            ))
            .sized_font(font)
            .animated()
            .font_color(AnimatedVals {
                idle: colors.on(On::SurfaceVariant),
                hover: colors.on(On::Surface).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);
    }

    fn checked_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK_BACKGROUND)
            .background_color(colors.accent(Accent::Primary))
            .border(UiRect::all(Val::Px(0.)));

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK)
            .animated()
            .background_color(AnimatedVals {
                idle: colors.on(On::Primary),
                enter_from: colors.on(On::Surface).into(),
                ..default()
            })
            .copy_from(theme_data.enter_animation);

        style_builder
            .switch_target(ButtonRadioButton::RADIOMARK)
            .animated()
            .size(AnimatedVals {
                idle: Val::Px(theme_spacing.inputs.radio_button.radiomark_size),
                enter_from: Val::Px(0.).into(),
                ..default()
            })
            .copy_from(theme_data.enter_animation);

        style_builder
            .switch_target(ButtonRadioButton::LABEL)
            .font_color(colors.on(On::SurfaceVariant));
    }

    fn button(name: String) -> impl Bundle {
        (
            Name::new(name),
            ButtonBundle::default(),
            TrackedInteraction::default(),
        )
    }

    fn radio_mark_background() -> impl Bundle {
        (
            Name::new("Radiomark Background"),
            NodeBundle {
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            LockedStyleAttributes::lock(LockableStyleAttribute::FocusPolicy),
        )
    }

    fn radio_mark() -> impl Bundle {
        (
            Name::new("Radiomark"),
            NodeBundle {
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            LockedStyleAttributes::from_vec(vec![
                LockableStyleAttribute::FocusPolicy,
                LockableStyleAttribute::Visibility,
            ]),
        )
    }
}

pub trait UiButtonRadioGroupExt {
    fn button_radio_group(
        &mut self,
        options: Vec<impl Into<String>>,
        selected: impl Into<Option<usize>>,
        unselectable: bool,
        group: Entity,
    ) -> UiBuilder<Entity>;
}

impl UiButtonRadioGroupExt for UiBuilder<'_, Entity> {
    /// A simple radio group with options. Optionally, the radio group can be "unselected"
    ///
    /// ### PseudoState usage
    /// - `PseudoState::Checked` is added to the currently selected `ButtonRadioButton` entity
    fn button_radio_group(
        &mut self,
        options: Vec<impl Into<String>>,
        selected: impl Into<Option<usize>>,
        unselectable: bool,
        group: Entity,
    ) -> UiBuilder<Entity> {
        let mut radio_group = self.spawn((
            ButtonRadioGroup::container(),
            ButtonRadioGroup {
                selected: selected.into(),
            },
        ));

        let mut index = 0;
        // let group = radio_group.id();
        for option in options {
            let label = option.into();
            // let label = "DESPAIR".into();
            let name = format!("Button Radio Button [{}]", label);
            let mut radio_button = ButtonRadioButton {
                checked: false,
                unselectable,
                index,
                group,
                ..default()
            };

            radio_group
                .container(ButtonRadioButton::button(name), |button| {
                    
                    radio_button.radiomark_background = button
                        .container(ButtonRadioButton::radio_mark_background(), |radio_mark_bg| {
                            radio_button.radiomark =
                                radio_mark_bg.spawn(ButtonRadioButton::radio_mark()).id();
                        })
                        .id();

                    radio_button.label = button.label(LabelConfig { label, ..default() }).id();
                })
                .insert(radio_button);

            index += 1;
        }

        radio_group
    }
}
