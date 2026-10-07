//! Styled text input field wrapper.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker component identifying a high-level Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInput;

/// Declarative TextInput configuration struct.
#[derive(Clone, Debug, Reflect)]
pub struct TextInput {
    pub placeholder: String,
    pub placeholder_color: Color,
    pub text_color: Color,
    pub font_size: f32,
    pub mode: TextInputMode,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            placeholder: String::new(),
            placeholder_color: Color::srgb(0.45, 0.50, 0.62),
            text_color: Color::srgb(0.95, 0.96, 0.98),
            font_size: 14.0,
            mode: TextInputMode::Normal,
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

    pub fn password(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            mode: TextInputMode::Password,
            ..default()
        }
    }

    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = color;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
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

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let input = UTextInput {
            placeholder: self.placeholder,
            placeholder_color: self.placeholder_color,
            text_color: self.text_color,
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

/// Spawns a styled interactive text input field.
pub fn luma_input(commands: &mut Commands, placeholder: impl Into<String>) -> Entity {
    TextInput::new(placeholder).spawn(commands)
}

/// Spawns a password text input field with masked character display.
pub fn luma_password_input(commands: &mut Commands, placeholder: impl Into<String>) -> Entity {
    TextInput::password(placeholder).spawn(commands)
}
