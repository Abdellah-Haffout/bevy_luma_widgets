//! Styled text input field widget with validation states, icons, and presets.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::InputStyle;

/// Visual validation or focus state for text input fields.
#[derive(Clone, Debug, PartialEq, Eq, Default, Reflect)]
pub enum InputState {
    #[default]
    Normal,
    Focused,
    Error(String),
    Success,
    Disabled,
}

/// Marker component identifying a high-level Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInput;

/// Marker component for the password visibility eye toggle button.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaPasswordToggle;

/// Declarative TextInput configuration and scene widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct TextInput {
    pub value: String,
    pub placeholder: String,
    pub state: InputState,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub font_size: f32,
    pub mode: TextInputMode,
    pub style: Option<InputStyle>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            state: InputState::Normal,
            leading_icon: None,
            trailing_icon: None,
            font: Handle::default(),
            icon_font: Handle::default(),
            font_size: 14.0,
            mode: TextInputMode::Normal,
            style: None,
        }
    }
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
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

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn mode(mut self, mode: TextInputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Preset constructor for a Password input field.
    pub fn password(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            mode: TextInputMode::Password,
            trailing_icon: Some(Icon::EYE.to_string()),
            ..default()
        }
    }

    /// Preset constructor for a Search Bar input field.
    pub fn search(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            leading_icon: Some(Icon::SEARCH.to_string()),
            style: Some(InputStyle::search()),
            ..default()
        }
    }

    /// Preset constructor for an OTP / PIN single-character or segmented digit field.
    pub fn otp() -> Self {
        Self {
            style: Some(InputStyle::otp()),
            ..default()
        }
    }

    /// Preset constructor for a multi-line Text Area field.
    pub fn multiline(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            style: Some(InputStyle::multiline()),
            ..default()
        }
    }

    /// Legacy manual spawn helper.
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let input = UTextInput {
            value: self.value,
            placeholder: self.placeholder,
            font_size: self.font_size,
            mode: self.mode,
            ..default()
        };
        let entity = spawn_text_input(commands, input);
        if let Ok(mut cmd) = commands.get_entity(entity) {
            cmd.insert(LumaInput);
        }
        entity
    }
}

impl Scene for TextInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let style = self.style.unwrap_or_default();

        let border_color = match &self.state {
            InputState::Normal => style.border_normal,
            InputState::Focused => style.border_focus,
            InputState::Error(_) => style.border_error,
            InputState::Success => style.border_success,
            InputState::Disabled => style.border_normal,
        };

        let has_leading = self.leading_icon.is_some();
        let leading_text = self.leading_icon.unwrap_or_default();

        let has_trailing = self.trailing_icon.is_some();
        let trailing_text = self.trailing_icon.unwrap_or_default();

        let display_value = if self.value.is_empty() {
            self.placeholder.clone()
        } else if self.mode == TextInputMode::Password {
            "•".repeat(self.value.chars().count())
        } else {
            self.value.clone()
        };

        let text_color = if self.value.is_empty() {
            style.placeholder_color
        } else {
            style.text_color
        };

        let value = self.value;
        let placeholder = self.placeholder;
        let placeholder_color = style.placeholder_color;
        let font_size = style.font_size;
        let mode = self.mode;
        let min_height = style.min_height;
        let radius = style.radius;
        let padding = style.padding;
        let bg_normal = style.bg_normal;
        let border_width = style.border_width;
        let border_focus = style.border_focus;
        let input_text_color = style.text_color;
        let font = self.font;
        let icon_font = self.icon_font;
        let is_password_mode = mode == TextInputMode::Password;

        let s = bsn! {
            LumaInput
            UTextInput {
                value,
                placeholder,
                placeholder_color,
                text_color: input_text_color,
                font_size,
                mode,
            }
            UNode {
                width: UVal::Percent(1.0),
                min_height,
                background_color: bg_normal,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UFocusable::new()
            UFocusVisual::border(border_focus, 2.0)
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: { if has_leading { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    UText {
                        text: leading_text,
                        font_size: 16.0,
                        font: { icon_font.clone() },
                        color: placeholder_color,
                    }
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                }
                Children [
                    UText {
                        text: display_value,
                        font_size,
                        font,
                        color: text_color,
                    }
                ]
                --
                UNode {
                    padding: USides::all(4.0),
                    border_radius: UCornerRadius::all(4.0),
                    background_color: Color::NONE,
                }
                LumaPasswordToggle
                UInteraction::default()
                UInteractionColors {
                    normal: Color::NONE,
                    hovered: { if is_password_mode { Color::srgba(1.0, 1.0, 1.0, 0.08) } else { Color::NONE } },
                    pressed: { if is_password_mode { Color::srgba(1.0, 1.0, 1.0, 0.16) } else { Color::NONE } },
                }
                ULayout {
                    display: { if has_trailing { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    UText {
                        text: trailing_text,
                        font_size: 16.0,
                        font: icon_font,
                        color: placeholder_color,
                    }
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Spawns a styled interactive text input field scene.
pub fn luma_input(placeholder: impl Into<String>) -> TextInput {
    TextInput::new(placeholder)
}

/// Spawns a password text input field with masked character display scene.
pub fn luma_password_input(placeholder: impl Into<String>) -> TextInput {
    TextInput::password(placeholder)
}

fn on_password_toggle_click(
    trigger: On<PointerClick>,
    toggle_query: Query<&LumaPasswordToggle>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut input_query: Query<&mut UTextInput>,
    mut icon_text_query: Query<&mut UText>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let is_toggle = toggle_query.contains(trigger.entity)
        || parents_query
            .iter_ancestors(trigger.entity)
            .any(|e| toggle_query.contains(e));

    if !is_toggle {
        return;
    }

    // Find the UTextInput ancestor
    let input_target = if input_query.contains(trigger.entity) {
        Some(trigger.entity)
    } else {
        parents_query
            .iter_ancestors(trigger.entity)
            .find(|&e| input_query.contains(e))
    };

    let Some(input_entity) = input_target else {
        return;
    };
    let Ok(mut text_input) = input_query.get_mut(input_entity) else {
        return;
    };

    let is_password = text_input.mode == TextInputMode::Password;
    text_input.mode = if is_password {
        TextInputMode::Normal
    } else {
        TextInputMode::Password
    };

    // Update the eye icon
    let toggle_node = if toggle_query.contains(trigger.entity) {
        trigger.entity
    } else {
        parents_query
            .iter_ancestors(trigger.entity)
            .find(|&e| toggle_query.contains(e))
            .unwrap_or(trigger.entity)
    };

    if let Ok(children) = children_query.get(toggle_node) {
        for child in children.iter() {
            if let Ok(mut text) = icon_text_query.get_mut(child) {
                text.text = if is_password {
                    Icon::EYE_OFF.to_string()
                } else {
                    Icon::EYE.to_string()
                };
            }
        }
    }
}

/// Plugin registering text input widget systems and observers.
pub struct LumaInputPlugin;

impl Plugin for LumaInputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaInput>()
            .register_type::<LumaPasswordToggle>()
            .add_observer(on_password_toggle_click);
    }
}
