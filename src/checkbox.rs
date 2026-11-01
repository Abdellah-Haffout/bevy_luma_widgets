//! Checkbox widget with checkmark indicator and label support.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Marker and state component for a checkbox.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaCheckbox {
    pub is_checked: bool,
    pub disabled: bool,
}

impl Default for LumaCheckbox {
    fn default() -> Self {
        Self {
            is_checked: false,
            disabled: false,
        }
    }
}

/// Marker component for the internal checkmark text/icon.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCheckmark;

/// Event triggered when a checkbox state changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct CheckboxChanged {
    pub entity: Entity,
    pub is_checked: bool,
}

const CHECKED_BG: Color = Color::srgb(0.20, 0.45, 0.90);
const UNCHECKED_BG: Color = Color::srgba(0.12, 0.15, 0.20, 0.8);
const CHECKED_BORDER: Color = Color::srgb(0.28, 0.55, 1.00);
const UNCHECKED_BORDER: Color = Color::srgb(0.30, 0.35, 0.45);

/// Declarative Checkbox widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Checkbox {
    pub label: Option<String>,
    pub is_checked: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            label: None,
            is_checked: false,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Checkbox {
    pub fn new(label: impl Into<String>, is_checked: bool) -> Self {
        Self {
            label: Some(label.into()),
            is_checked,
            ..default()
        }
    }

    pub fn box_only(is_checked: bool) -> Self {
        Self {
            is_checked,
            ..default()
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn checked(mut self, is_checked: bool) -> Self {
        self.is_checked = is_checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for Checkbox {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let bg = if self.is_checked { CHECKED_BG } else { UNCHECKED_BG };
        let border = if self.is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };
        let icon_color = if self.is_checked { Color::WHITE } else { Color::NONE };

        let is_checked = self.is_checked;
        let disabled = self.disabled;
        let icon_font = self.icon_font;
        let font = self.font;
        let bg_hover = if self.is_checked { Color::srgb(0.28, 0.55, 1.00) } else { Color::srgb(0.24, 0.28, 0.36) };
        let bg_pressed = if self.is_checked { Color::srgb(0.14, 0.35, 0.75) } else { Color::srgb(0.12, 0.15, 0.20) };

        let box_node = bsn! {
            LumaCheckbox {
                is_checked,
                disabled,
            }
            UNode {
                width: UVal::Px(20.0),
                height: UVal::Px(20.0),
                background_color: bg,
                border_radius: UCornerRadius::all(5.0),
            }
            UBorder {
                color: border,
                width: 1.5,
                radius: UCornerRadius::all(5.0),
            }
            UInteraction::default()
            UInteractionColors {
                normal: bg,
                hovered: bg_hover,
                pressed: bg_pressed,
            }
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                LumaCheckmark
                UText {
                    text: { Icon::CHECK.to_string() },
                    font_size: 13.0,
                    font: icon_font,
                    color: icon_color,
                }
            ]
        };

        if let Some(label_text) = self.label {
            let container = bsn! {
                UNode::default()
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    @{box_node}
                    --
                    UText {
                        text: label_text,
                        font_size: 14.0,
                        font,
                        color: Color::srgb(0.92, 0.94, 0.98),
                    }
                ]
            };
            container.resolve(context, scene)
        } else {
            box_node.resolve(context, scene)
        }
    }
}

/// Creates an isolated checkbox box scene.
pub fn luma_checkbox_box(is_checked: bool, icon_font: Handle<Font>) -> Checkbox {
    Checkbox::box_only(is_checked).icon_font(icon_font)
}

/// Creates a checkbox with an adjacent text label.
pub fn luma_checkbox(
    label: impl Into<String>,
    is_checked: bool,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Checkbox {
    Checkbox::new(label, is_checked).font(font).icon_font(icon_font)
}

fn toggle_checkbox(
    entity: Entity,
    commands: &mut Commands,
    query: &mut Query<(
        &mut LumaCheckbox,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    text_query: &mut Query<&mut UText, With<LumaCheckmark>>,
) {
    if let Ok((mut checkbox, mut node, mut border, children, mut visual_opt, mut colors_opt)) = query.get_mut(entity) {
        if checkbox.disabled {
            return;
        }

        checkbox.is_checked = !checkbox.is_checked;
        let is_checked = checkbox.is_checked;

        node.background_color = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
        let new_border = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };

        if let Some(ref mut colors) = colors_opt {
            if is_checked {
                colors.normal = CHECKED_BG;
                colors.hovered = Color::srgb(0.28, 0.55, 1.00);
                colors.pressed = Color::srgb(0.14, 0.35, 0.75);
            } else {
                colors.normal = UNCHECKED_BG;
                colors.hovered = Color::srgb(0.24, 0.28, 0.36);
                colors.pressed = Color::srgb(0.12, 0.15, 0.20);
            }
        }

        if let Some(ref mut visual) = visual_opt {
            if visual.is_captured() {
                visual.set_original_border_color(new_border);
            } else {
                border.color = new_border;
            }
        } else {
            border.color = new_border;
        }

        let icon_color = if is_checked { Color::WHITE } else { Color::NONE };

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.color = icon_color;
                }
            }
        }

        commands.trigger(CheckboxChanged {
            entity,
            is_checked,
        });
    }
}

fn on_checkbox_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    mut query: Query<(
        &mut LumaCheckbox,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut text_query: Query<&mut UText, With<LumaCheckmark>>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let target = if query.contains(trigger.entity) {
        Some(trigger.entity)
    } else if let Some(ancestor) = parents_query.iter_ancestors(trigger.entity).find(|&e| query.contains(e)) {
        Some(ancestor)
    } else if let Ok(children) = children_query.get(trigger.entity) {
        children.iter().find(|&e| query.contains(e))
    } else {
        None
    };
    if let Some(entity) = target {
        toggle_checkbox(entity, &mut commands, &mut query, &mut text_query);
    }
}

fn on_checkbox_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    mut query: Query<(
        &mut LumaCheckbox,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut text_query: Query<&mut UText, With<LumaCheckmark>>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    let target = if query.contains(trigger.entity) {
        Some(trigger.entity)
    } else if let Some(ancestor) = parents_query.iter_ancestors(trigger.entity).find(|&e| query.contains(e)) {
        Some(ancestor)
    } else if let Ok(children) = children_query.get(trigger.entity) {
        children.iter().find(|&e| query.contains(e))
    } else {
        None
    };
    if let Some(entity) = target {
        toggle_checkbox(entity, &mut commands, &mut query, &mut text_query);
    }
}

pub struct LumaCheckboxPlugin;

impl Plugin for LumaCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaCheckbox>()
            .register_type::<LumaCheckmark>()
            .add_observer(on_checkbox_click)
            .add_observer(on_checkbox_activate);
    }
}
