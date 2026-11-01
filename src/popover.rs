//! Floating popover overlay card widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::PopoverStyle;

/// Marker component for an open popover card.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaPopover {
    pub is_open: bool,
}

/// Declarative Popover overlay widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Popover {
    pub is_open: bool,
    pub width: UVal,
    pub height: UVal,
    pub style: Option<PopoverStyle>,
}

impl Default for Popover {
    fn default() -> Self {
        Self {
            is_open: true,
            width: UVal::Auto,
            height: UVal::Auto,
            style: None,
        }
    }
}

impl Popover {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn width(mut self, width: UVal) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: UVal) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: PopoverStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Popover {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let style = self.style.unwrap_or_default();
        let is_open = self.is_open;
        let width = self.width;
        let height = self.height;
        let bg_color = style.bg_color;
        let border_color = style.border_color;
        let border_width = style.border_width;
        let radius = style.radius;
        let padding = style.padding;

        let s = bsn! {
            LumaPopover {
                is_open,
            }
            UNode {
                width,
                height,
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: { if is_open { UDisplay::Flex } else { UDisplay::None } },
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a popover scene.
pub fn luma_popover() -> Popover {
    Popover::new()
}

/// Plugin registering popover types.
pub struct LumaPopoverPlugin;

impl Plugin for LumaPopoverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaPopover>();
    }
}
