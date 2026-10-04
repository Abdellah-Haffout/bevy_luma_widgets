//! Checkbox widget with checkmark indicator and label support.

use bevy::prelude::*;
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

/// Creates an isolated checkbox box scene.
pub fn luma_checkbox_box(is_checked: bool, icon_font: Handle<Font>) -> impl Scene {
    let bg = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
    let border = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };
    let icon_color = if is_checked { Color::WHITE } else { Color::NONE };

    bsn! {
        LumaCheckbox {
            is_checked,
            disabled: false,
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
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                LumaCheckmark
                UText {
                    text: { Icon::CHECK.to_string() },
                    font_size: 13.0,
                    font: icon_font,
                    color: icon_color,
                }
            )
        ]
    }
}

/// Creates a checkbox with an adjacent text label.
pub fn luma_checkbox(
    label: impl Into<String>,
    is_checked: bool,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    let label = label.into();

    bsn! {
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (
                luma_checkbox_box(is_checked, icon_font)
            ),
            (
                UText {
                    text: label,
                    font_size: 14.0,
                    font,
                    color: Color::srgb(0.92, 0.94, 0.98),
                }
            )
        ]
    }
}

fn on_checkbox_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(&mut LumaCheckbox, &mut UNode, &mut UBorder, Option<&Children>)>,
    mut checkmark_query: Query<&mut UText, With<LumaCheckmark>>,
) {
    let entity = trigger.entity.entity();
    if let Ok((mut checkbox, mut node, mut border, children)) = query.get_mut(entity) {
        if checkbox.disabled {
            return;
        }

        checkbox.is_checked = !checkbox.is_checked;
        let is_checked = checkbox.is_checked;

        node.background_color = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
        border.color = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };

        let icon_color = if is_checked { Color::WHITE } else { Color::NONE };

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut text) = checkmark_query.get_mut(child) {
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

pub struct LumaCheckboxPlugin;

impl Plugin for LumaCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaCheckbox>()
            .register_type::<LumaCheckmark>()
            .add_observer(on_checkbox_click);
    }
}
