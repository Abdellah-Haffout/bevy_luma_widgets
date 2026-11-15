//! Dock area, window, tab bar, and sidebar structural components and events.

use std::sync::Arc;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Presentation style for tabs inside a [`DockArea`].
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize,
)]
pub enum DockAreaStyle {
    /// Horizontal top tab bar with tab pills, drag grip, `+` button, and per-tab close button.
    #[default]
    TabBar,
    /// Vertical sidebar strip showing one icon button per docked window.
    IconSidebar,
    /// Displays only the active window content with no chrome or header bar.
    Headless,
}

/// Container region holding one or more tabbed [`DockWindow`] children.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockArea {
    /// Logical identifier used for default routing and workspace persistence.
    pub id: String,
    /// Visual presentation mode for switching windows inside this area.
    pub style: DockAreaStyle,
    /// When `true`, closing the last tab leaves the empty area alive instead of collapsing it.
    pub persistent: bool,
}

impl DockArea {
    /// Creates a new tab-bar [`DockArea`] with the given identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            style: DockAreaStyle::TabBar,
            persistent: false,
        }
    }

    /// Sets the presentation [`DockAreaStyle`].
    pub fn with_style(mut self, style: DockAreaStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets whether this area stays alive when all its tabs are closed.
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }
}

/// Tracks the currently active [`DockWindow`] entity inside a [`DockArea`].
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct ActiveDockWindow(pub Option<Entity>);

impl ActiveDockWindow {
    /// Returns the active window entity, if any.
    pub fn get(&self) -> Option<Entity> {
        self.0
    }
}

/// Content window mounted inside a [`DockArea`].
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct DockWindow {
    /// Window descriptor id from [`WindowRegistry`](super::registry::WindowRegistry).
    pub window_id: String,
    /// Display title rendered in tabs and drag previews.
    pub name: String,
    /// Optional Lucide icon glyph rendered alongside or instead of the title.
    pub icon: Option<String>,
    /// Whether the user can close this window tab via `X`, middle-click, or right-click.
    pub closeable: bool,
}

impl Default for DockWindow {
    fn default() -> Self {
        Self {
            window_id: String::new(),
            name: String::new(),
            icon: None,
            closeable: true,
        }
    }
}

impl DockWindow {
    /// Creates a new closeable [`DockWindow`].
    pub fn new(window_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            window_id: window_id.into(),
            name: name.into(),
            icon: None,
            closeable: true,
        }
    }

    /// Attaches an icon glyph (e.g. a [`Icon`](bevy_luma::style::prelude::Icon) constant).
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Configures whether the tab can be closed by the user.
    pub fn with_closeable(mut self, closeable: bool) -> Self {
        self.closeable = closeable;
        self
    }
}

/// Marker on the top bar container of a [`DockAreaStyle::TabBar`] area.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTabBar;

/// Marker on the horizontal row holding individual [`DockTab`] items.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTabRow;

/// Interactive tab pill inside a [`DockTabRow`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct DockTab {
    pub window_entity: Entity,
    pub area_entity: Entity,
    pub tab_index: usize,
}

/// Close button (`X`) inside a [`DockTab`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct DockTabCloseButton {
    pub window_entity: Entity,
    pub area_entity: Entity,
}

/// Add-window button (`+`) in a [`DockTabBar`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct DockTabAddButton {
    pub area_entity: Entity,
}

/// Drag grip handle in a [`DockTabBar`] that drags the active window of that area.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct DockTabGrip {
    pub area_entity: Entity,
}

/// Container inside a [`DockArea`] holding its docked [`DockWindow`] content nodes.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTabContent;

/// Marker on the vertical icon strip of a [`DockAreaStyle::IconSidebar`] area.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockSidebarContainer;

/// Interactive icon button inside a [`DockSidebarContainer`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct DockSidebarIcon {
    pub window_entity: Entity,
    pub area_entity: Entity,
}

/// Default area identifier used as fallback when adding or routing windows.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct DefaultArea(pub String);

impl Default for DefaultArea {
    fn default() -> Self {
        Self("main".to_string())
    }
}

/// Optional explicit icon font override. When absent, the panel system falls back to `Theme::icon.font`.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct IconFontHandle(pub Handle<Font>);

/// Optional resolver mapping external area names/identifiers to canonical dock anchor IDs.
#[derive(Resource, Clone)]
pub struct ToAnchorId(pub Arc<dyn Fn(&str) -> Option<String> + Send + Sync>);

impl ToAnchorId {
    /// Creates a new anchor mapper closure.
    pub fn new(f: impl Fn(&str) -> Option<String> + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }

    /// Resolves an anchor identifier from an arbitrary area key.
    pub fn resolve(&self, area_key: &str) -> Option<String> {
        (self.0)(area_key)
    }
}

/// Emitted whenever a docked tab or sidebar window becomes active in an area.
#[derive(Event, Clone, Debug)]
pub struct TabActivated {
    pub area_entity: Entity,
    pub window_entity: Entity,
    pub window_id: String,
}

/// Emitted whenever a docked window tab is closed.
#[derive(Event, Clone, Debug)]
pub struct TabClosed {
    pub area_entity: Entity,
    pub window_entity: Entity,
    pub window_id: String,
}
