//! Sliding drawer and bottom sheet panel container.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::DrawerStyle;

/// Position from which the drawer slides out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum DrawerPlacement {
    Left,
    #[default]
    Right,
    Bottom,
    Top,
}

/// Marker and state component for a sliding Drawer or Sheet panel.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaDrawer {
    pub placement: DrawerPlacement,
    pub is_open: bool,
}

impl Default for LumaDrawer {
    fn default() -> Self {
        Self {
            placement: DrawerPlacement::Right,
            is_open: true,
        }
    }
}

/// Marker component for the drawer close button.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDrawerClose;

/// Marker component for the inner drawer content panel.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDrawerPanel;

/// Event triggered to close a drawer.
#[derive(Event, Clone, Copy, Debug)]
pub struct CloseDrawer(pub Entity);

/// Declarative Drawer container widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Drawer {
    pub placement: DrawerPlacement,
    pub is_open: bool,
    pub title: Option<String>,
    pub width: UVal,
    pub height: UVal,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<DrawerStyle>,
}

impl Default for Drawer {
    fn default() -> Self {
        Self {
            placement: DrawerPlacement::Right,
            is_open: true,
            title: None,
            width: UVal::Px(360.0),
            height: UVal::Percent(1.0),
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl Drawer {
    pub fn new(placement: DrawerPlacement) -> Self {
        let (width, height) = match placement {
            DrawerPlacement::Left | DrawerPlacement::Right => (UVal::Px(360.0), UVal::Percent(1.0)),
            DrawerPlacement::Bottom | DrawerPlacement::Top => (UVal::Percent(1.0), UVal::Px(320.0)),
        };
        Self {
            placement,
            width,
            height,
            ..default()
        }
    }

    pub fn left() -> Self {
        Self::new(DrawerPlacement::Left)
    }

    pub fn right() -> Self {
        Self::new(DrawerPlacement::Right)
    }

    pub fn bottom() -> Self {
        Self::new(DrawerPlacement::Bottom)
    }

    pub fn top() -> Self {
        Self::new(DrawerPlacement::Top)
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn style(mut self, style: DrawerStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Drawer {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let style = self.style.unwrap_or_default();
        let placement = self.placement;
        let is_open = self.is_open;
        let width = self.width;
        let height = self.height;

        let has_title = self.title.is_some();
        let title_text = self.title.unwrap_or_default();
        let font = self.font;
        let icon_font = self.icon_font;

        let (justify_content, align_items) = match placement {
            DrawerPlacement::Left => (UJustifyContent::Start, UAlignItems::Stretch),
            DrawerPlacement::Right => (UJustifyContent::End, UAlignItems::Stretch),
            DrawerPlacement::Bottom => (UJustifyContent::Stretch, UAlignItems::End),
            DrawerPlacement::Top => (UJustifyContent::Stretch, UAlignItems::Start),
        };

        let backdrop_color = style.backdrop_color;
        let bg_color = style.bg_color;
        let border_color = style.border_color;
        let border_width = style.border_width;
        let radius = style.radius;
        let padding = style.padding;

        let s = bsn! {
            LumaDrawer {
                placement,
                is_open,
            }
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: backdrop_color,
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(0.0),
                right: UVal::Px(0.0),
                top: UVal::Px(0.0),
                bottom: UVal::Px(0.0),
                ..USelf::default()
            }
            UZIndex::Global(90)
            UInteraction::default()
            ULayout {
                display: { if is_open { UDisplay::Flex } else { UDisplay::None } },
                flex_direction: UFlexDirection::Row,
                justify_content,
                align_items,
            }
            Children [
                UNode {
                    width,
                    height,
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(radius),
                    padding,
                }
                LumaDrawerPanel
                UInteraction::default()
                UBorder {
                    color: border_color,
                    width: border_width,
                    radius: UCornerRadius::all(radius),
                }
                UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(0.0, 8.0), 24.0)
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 16.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: { if has_title { UDisplay::Flex } else { UDisplay::None } },
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: title_text,
                            font_size: 18.0,
                            font,
                            color: Color::WHITE,
                        }
                        --
                        UNode {
                            padding: USides::all(6.0),
                            border_radius: UCornerRadius::all(6.0),
                            background_color: Color::NONE,
                        }
                        LumaDrawerClose
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::NONE,
                            hovered: Color::srgba(1.0, 1.0, 1.0, 0.08),
                            pressed: Color::srgba(1.0, 1.0, 1.0, 0.16),
                        }
                        Children [
                            UText {
                                text: { Icon::X.to_string() },
                                font_size: 16.0,
                                font: icon_font,
                                color: Color::srgb(0.60, 0.65, 0.75),
                            }
                        ]
                    ]
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a sliding drawer scene.
pub fn luma_drawer(placement: DrawerPlacement) -> Drawer {
    Drawer::new(placement)
}

fn on_drawer_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    drawer_query: Query<Entity, With<LumaDrawer>>,
    close_query: Query<Entity, With<LumaDrawerClose>>,
    parents_query: Query<&ChildOf>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let clicked = trigger.entity;

    // Check if clicked the [x] close button
    let is_close_btn = close_query.contains(clicked)
        || parents_query
            .iter_ancestors(clicked)
            .any(|e| close_query.contains(e));

    // Find the drawer entity
    let drawer_target = if drawer_query.contains(clicked) {
        Some(clicked)
    } else {
        parents_query
            .iter_ancestors(clicked)
            .find(|&e| drawer_query.contains(e))
    };

    let Some(drawer_entity) = drawer_target else {
        return;
    };

    if is_close_btn {
        commands.trigger(CloseDrawer(drawer_entity));
    } else if drawer_query.contains(clicked) {
        // Direct click on the backdrop (outside the inner panel)
        commands.trigger(CloseDrawer(drawer_entity));
    }
}

fn on_close_drawer_event(
    trigger: On<CloseDrawer>,
    mut drawer_query: Query<(&mut LumaDrawer, &mut ULayout)>,
) {
    let entity = trigger.event().0;
    if let Ok((mut drawer, mut layout)) = drawer_query.get_mut(entity) {
        drawer.is_open = false;
        layout.display = UDisplay::None;
    }
}

/// Plugin registering drawer systems and observers.
pub struct LumaDrawerPlugin;

impl Plugin for LumaDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaDrawer>()
            .register_type::<LumaDrawerClose>()
            .register_type::<LumaDrawerPanel>()
            .add_observer(on_drawer_click)
            .add_observer(on_close_drawer_event);
    }
}
