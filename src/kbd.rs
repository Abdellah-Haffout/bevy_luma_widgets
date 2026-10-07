//! Keyboard keycap shortcut badge widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Sizing preset for keyboard keycaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum KbdSize {
    Small,
    #[default]
    Medium,
}

impl KbdSize {
    pub fn font_size(&self) -> f32 {
        match self {
            KbdSize::Small => 10.0,
            KbdSize::Medium => 12.0,
        }
    }

    pub fn min_height(&self) -> f32 {
        match self {
            KbdSize::Small => 20.0,
            KbdSize::Medium => 24.0,
        }
    }

    pub fn padding(&self) -> USides {
        match self {
            KbdSize::Small => USides::axes(6.0, 2.0),
            KbdSize::Medium => USides::axes(8.0, 3.0),
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            KbdSize::Small => 4.0,
            KbdSize::Medium => 5.0,
        }
    }
}

/// Marker component for a keyboard keycap badge.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaKbd {
    pub size: KbdSize,
}

const KBD_BG: Color = Color::srgb(0.14, 0.17, 0.23);
const KBD_BORDER: Color = Color::srgb(0.30, 0.36, 0.48);
const KBD_TEXT: Color = Color::srgb(0.88, 0.92, 0.97);

/// Declarative Keyboard Keycap widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Kbd {
    pub text: String,
    pub size: KbdSize,
    pub font: Handle<Font>,
}

impl Default for Kbd {
    fn default() -> Self {
        Self {
            text: String::new(),
            size: KbdSize::Medium,
            font: Handle::default(),
        }
    }
}

impl Kbd {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..default()
        }
    }

    pub fn small(text: impl Into<String>) -> Self {
        Self::new(text).size(KbdSize::Small)
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Kbd {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font_size = self.size.font_size();
        let min_height = self.size.min_height();
        let padding = self.size.padding();
        let radius = self.size.radius();

        let size = self.size;
        let text = self.text;
        let font = self.font;

        let s = bsn! {
            LumaKbd { size }
            UNode {
                min_height,
                background_color: KBD_BG,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: KBD_BORDER,
                width: 1.0,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                (UText {
                    text,
                    font_size,
                    font,
                    color: KBD_TEXT,
                })
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates an individual keyboard keycap scene.
pub fn luma_kbd(
    text: impl Into<String>,
    size: KbdSize,
    font: Handle<Font>,
) -> Kbd {
    Kbd::new(text).size(size).font(font)
}

/// Creates a keyboard shortcut separator (e.g. "+").
pub fn luma_kbd_separator(font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: { "+".to_string() },
                font_size: 11.0,
                font,
                color: Color::srgb(0.50, 0.55, 0.65),
            })
        ]
    }
}

/// Creates a horizontal container for grouping keycaps together (e.g. `Ctrl + Shift + P`).
pub fn luma_kbd_shortcut(keycaps: impl Scene) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 4.0,
        }
        Children [
            keycaps
        ]
    }
}

pub struct LumaKbdPlugin;

impl Plugin for LumaKbdPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaKbd>()
            .register_type::<KbdSize>();
    }
}
