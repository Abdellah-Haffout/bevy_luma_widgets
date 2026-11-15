//! Checkbox widget with checkmark indicator and label support.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{
    find_target_with_children, interaction_colors, is_primary_click, shift_brightness,
    sync_focus_visual_border,
};
use crate::theme::{CheckboxStyle, WidgetColors};

/// Marker and state component for a checkbox.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaCheckbox {
    pub is_checked: bool,
    pub disabled: bool,
    pub style: CheckboxStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaCheckbox {
    fn default() -> Self {
        Self {
            is_checked: false,
            disabled: false,
            style: CheckboxStyle::default(),
            has_explicit_style: false,
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

/// Declarative Checkbox widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Checkbox {
    pub label: Option<String>,
    pub is_checked: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<CheckboxStyle>,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            label: None,
            is_checked: false,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
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

    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = Some(style);
        self
    }
}

pub(crate) fn checkbox_visual_state(
    style: &CheckboxStyle,
    is_checked: bool,
    disabled: bool,
) -> (Color, Color, Color, UInteractionColors) {
    let palette = WidgetColors::default();
    if disabled {
        let bg = palette.disabled_bg();
        let border = palette.disabled_border();
        let icon = if is_checked { palette.disabled_text() } else { Color::NONE };
        return (bg, border, icon, interaction_colors(bg, bg, bg, true));
    }

    let bg = if is_checked { style.checked_bg } else { style.unchecked_bg };
    let border = if is_checked { style.checked_border } else { style.unchecked_border };
    let icon = if is_checked { style.check_color } else { Color::NONE };
    let (hovered, pressed) = if is_checked {
        if style.checked_bg == palette.primary {
            (palette.primary_hover, palette.primary_pressed)
        } else {
            (shift_brightness(style.checked_bg, 0.08), shift_brightness(style.checked_bg, -0.08))
        }
    } else if style.unchecked_bg == palette.surface {
        (palette.surface_hover, palette.secondary_pressed)
    } else {
        (shift_brightness(style.unchecked_bg, 0.06), shift_brightness(style.unchecked_bg, -0.06))
    };
    (bg, border, icon, interaction_colors(bg, hovered, pressed, false))
}

impl Scene for Checkbox {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let is_checked = self.is_checked;
        let disabled = self.disabled;
        let icon_font = self.icon_font;
        let font = self.font;

        let palette = WidgetColors::default();
        let (bg, border, icon_color, i_colors) = checkbox_visual_state(&style, is_checked, disabled);
        let label_color = if disabled { palette.disabled_text() } else { style.label_color };
        let focus_color = palette.focus_ring();
        let box_size = style.size;
        let box_radius = style.radius;
        let border_width = style.border_width;
        let label_font_size = style.font_size;
        let icon_font_size = (box_size * 0.65).max(11.0);

        let box_node = bsn! {
            LumaCheckbox {
                is_checked,
                disabled,
                style,
                has_explicit_style,
            }
            UNode {
                width: UVal::Px(box_size),
                height: UVal::Px(box_size),
                background_color: bg,
                border_radius: UCornerRadius::all(box_radius),
            }
            UBorder {
                color: border,
                width: border_width,
                radius: UCornerRadius::all(box_radius),
            }
            UInteraction::default()
            i_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::new()
            }
            UFocusVisual::border(focus_color, 2.0)
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                LumaCheckmark
                UText {
                    text: { Icon::CHECK.to_string() },
                    font_size: icon_font_size,
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
                        font_size: label_font_size,
                        font,
                        color: label_color,
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
    if let Ok((mut checkbox, mut node, mut border, children, visual_opt, colors_opt)) = query.get_mut(entity) {
        if checkbox.disabled {
            return;
        }

        checkbox.is_checked = !checkbox.is_checked;
        let is_checked = checkbox.is_checked;

        let (new_bg, new_border, icon_color, new_colors) =
            checkbox_visual_state(&checkbox.style, is_checked, false);
        node.background_color = new_bg;

        if let Some(mut colors) = colors_opt {
            *colors = new_colors;
        }
        sync_focus_visual_border(visual_opt, &mut border, new_border);

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
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(entity) = find_target_with_children(trigger.entity, &query, &parents_query, &children_query) {
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
    if let Some(entity) = find_target_with_children(trigger.entity, &query, &parents_query, &children_query) {
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
