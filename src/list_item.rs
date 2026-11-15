//! Standard interactive list item widget with leading icon, title, subtitle, and trailing action.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{find_ancestor_or_self, interaction_colors, is_primary_click};
use crate::theme::{ListItemStyle, WidgetColors};

/// Marker and state component for an interactive list item.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaListItem {
    pub is_selected: bool,
    pub disabled: bool,
    pub style: ListItemStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaListItem {
    fn default() -> Self {
        Self {
            is_selected: false,
            disabled: false,
            style: ListItemStyle::default(),
            has_explicit_style: false,
        }
    }
}

/// Marker component for the primary title text inside a list item.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaListItemTitle;

/// Marker component for the subtitle / trailing muted text inside a list item.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaListItemSubtitle;

/// Event triggered when a list item is clicked.
#[derive(Event, Clone, Copy, Debug)]
pub struct ListItemClicked(pub Entity);

/// Declarative ListItem widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct ListItem {
    pub title: String,
    pub subtitle: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_text: Option<String>,
    pub trailing_icon: Option<String>,
    pub is_selected: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<ListItemStyle>,
}

impl Default for ListItem {
    fn default() -> Self {
        Self {
            title: String::new(),
            subtitle: None,
            leading_icon: None,
            trailing_text: None,
            trailing_icon: None,
            is_selected: false,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl ListItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
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

    pub fn style(mut self, style: ListItemStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for ListItem {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();

        let has_leading = self.leading_icon.is_some();
        let leading_text = self.leading_icon.unwrap_or_default();

        let has_subtitle = self.subtitle.is_some();
        let subtitle_text = self.subtitle.unwrap_or_default();

        let has_trailing_text = self.trailing_text.is_some();
        let trailing_text = self.trailing_text.unwrap_or_default();

        let has_trailing_icon = self.trailing_icon.is_some();
        let trailing_icon_text = self.trailing_icon.unwrap_or_default();

        let is_selected = self.is_selected;
        let disabled = self.disabled;
        let title = self.title;
        let font = self.font;
        let icon_font = self.icon_font;

        let bg_normal = if disabled {
            colors.disabled_bg()
        } else if is_selected {
            style.bg_pressed
        } else {
            style.bg_normal
        };

        let min_height = style.min_height;
        let radius = style.radius;
        let padding = style.padding;
        let border_color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };
        let border_width = style.border_width;
        let item_colors = interaction_colors(bg_normal, style.bg_hover, style.bg_pressed, disabled);
        let font_size = style.font_size;
        let subtitle_font_size = style.subtitle_font_size;
        let title_color = if disabled {
            colors.disabled_text()
        } else {
            style.title_color
        };
        let subtitle_color = if disabled {
            colors.disabled_text()
        } else {
            style.subtitle_color
        };
        let luma_item = LumaListItem {
            is_selected,
            disabled,
            style,
            has_explicit_style,
        };

        let s = bsn! {
            luma_item
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
            UInteraction::default()
            item_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::default()
            }
            UFocusVisual::border(colors.focus_ring(), 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 12.0,
            }
            Children [
                // Left group: Leading icon + Title & Subtitle column
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: { if has_leading { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        LumaListItemSubtitle
                        UText {
                            text: leading_text,
                            font_size: 18.0,
                            font: { icon_font.clone() },
                            color: subtitle_color,
                        }
                    ]
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 2.0,
                    }
                    Children [
                        LumaListItemTitle
                        UText {
                            text: title,
                            font_size,
                            font: { font.clone() },
                            color: title_color,
                        }
                        --
                        UNode::default()
                        ULayout {
                            display: { if has_subtitle { UDisplay::Flex } else { UDisplay::None } },
                        }
                        Children [
                            LumaListItemSubtitle
                            UText {
                                text: subtitle_text,
                                font_size: subtitle_font_size,
                                font: { font.clone() },
                                color: subtitle_color,
                            }
                        ]
                    ]
                ]
                --
                // Right group: Trailing text / icon
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: { if has_trailing_text { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        LumaListItemSubtitle
                        UText {
                            text: trailing_text,
                            font_size: subtitle_font_size,
                            font,
                            color: subtitle_color,
                        }
                    ]
                    --
                    UNode::default()
                    ULayout {
                        display: { if has_trailing_icon { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        LumaListItemSubtitle
                        UText {
                            text: trailing_icon_text,
                            font_size: 16.0,
                            font: icon_font,
                            color: subtitle_color,
                        }
                    ]
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a standard list item scene.
pub fn luma_list_item(title: impl Into<String>) -> ListItem {
    ListItem::new(title)
}

fn activate_list_item(
    target: Entity,
    query: &Query<&LumaListItem>,
    parents_query: &Query<&ChildOf>,
    commands: &mut Commands,
) {
    let Some(entity) = find_ancestor_or_self(target, query, parents_query) else {
        return;
    };
    if let Ok(item) = query.get(entity) {
        if !item.disabled {
            commands.trigger(ListItemClicked(entity));
        }
    }
}

fn list_item_click_observer(
    trigger: On<PointerClick>,
    query: Query<&LumaListItem>,
    parents_query: Query<&ChildOf>,
    mut commands: Commands,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    activate_list_item(trigger.entity, &query, &parents_query, &mut commands);
}

fn list_item_activate_observer(
    trigger: On<UFocusActivate>,
    query: Query<&LumaListItem>,
    parents_query: Query<&ChildOf>,
    mut commands: Commands,
) {
    activate_list_item(trigger.entity, &query, &parents_query, &mut commands);
}

/// Plugin registering list item systems and observers.
pub struct LumaListItemPlugin;

impl Plugin for LumaListItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaListItem>()
            .register_type::<LumaListItemTitle>()
            .register_type::<LumaListItemSubtitle>()
            .add_observer(list_item_click_observer)
            .add_observer(list_item_activate_observer);
    }
}
