//! Dropdown selection menu widget.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::SelectStyle;

/// Single option entry in a Select menu.
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// Marker and state component for a Select dropdown.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSelect {
    pub is_open: bool,
    pub selected_index: Option<usize>,
    pub disabled: bool,
}

impl Default for LumaSelect {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_index: None,
            disabled: false,
        }
    }
}

/// Event triggered when a select option is picked.
#[derive(Event, Clone, Debug)]
pub struct SelectChanged {
    pub entity: Entity,
    pub selected_index: usize,
    pub value: String,
}

/// Declarative Select dropdown widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Select {
    pub placeholder: String,
    pub options: Vec<SelectOption>,
    pub selected_index: Option<usize>,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<SelectStyle>,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            placeholder: "Select an option...".to_string(),
            options: Vec::new(),
            selected_index: None,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl Select {
    pub fn new(placeholder: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            placeholder: placeholder.into(),
            options,
            ..default()
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
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

    pub fn style(mut self, style: SelectStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Select {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let style = self.style.unwrap_or_default();

        let display_text = if let Some(idx) = self.selected_index {
            self.options
                .get(idx)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| self.placeholder.clone())
        } else {
            self.placeholder.clone()
        };

        let is_placeholder = self.selected_index.is_none();
        let text_color = if is_placeholder {
            Color::srgb(0.50, 0.55, 0.65)
        } else {
            style.text_color
        };

        let min_height = style.min_height;
        let bg_color = style.bg_color;
        let bg_hover = style.bg_hover;
        let radius = style.radius;
        let padding = style.padding;
        let border_color = style.border_color;
        let border_width = style.border_width;
        let border_focus = style.border_focus;
        let font_size = style.font_size;
        let icon_color = style.text_color;

        let selected_index = self.selected_index;
        let disabled = self.disabled;
        let font = self.font;
        let icon_font = self.icon_font;

        let s = bsn! {
            LumaSelect {
                is_open: false,
                selected_index,
                disabled,
            }
            UNode {
                width: UVal::Percent(1.0),
                min_height,
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UInteraction::default()
            UInteractionColors {
                normal: bg_color,
                hovered: bg_hover,
                pressed: bg_hover,
            }
            UFocusable::new()
            UFocusVisual::border(border_focus, 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                UNode::default()
                Children [
                    UText {
                        text: display_text,
                        font_size,
                        font,
                        color: text_color,
                    }
                ]
                --
                UNode::default()
                Children [
                    UText {
                        text: { Icon::CHEVRON_DOWN.to_string() },
                        font_size: 14.0,
                        font: icon_font,
                        color: icon_color,
                    }
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a standard select dropdown scene.
pub fn luma_select(placeholder: impl Into<String>, options: Vec<SelectOption>) -> Select {
    Select::new(placeholder, options)
}

fn select_click_observer(
    trigger: On<PointerClick>,
    mut query: Query<&mut LumaSelect>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let entity = trigger.entity;
    if let Ok(mut select) = query.get_mut(entity) {
        if !select.disabled {
            select.is_open = !select.is_open;
        }
    }
}

/// Plugin registering select dropdown systems and observers.
pub struct LumaSelectPlugin;

impl Plugin for LumaSelectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSelect>()
            .add_observer(select_click_observer);
    }
}
