//! Complete Luma-native split panels, docking tree, tab drag-and-drop, icon sidebars,
//! `+` add-window popups, and multi-workspace strips.

pub mod add_window_popup;
pub mod area;
pub mod drag;
pub mod layout;
pub mod reconcile;
pub mod registry;
pub mod sidebar;
pub mod split;
pub mod style;
pub mod tabs;
pub mod tree;
pub mod workspace;
pub mod workspace_tabs;

#[cfg(test)]
mod tests;

use bevy::prelude::*;

/// Ordered execution stages for the docking & panel pipeline within [`Update`].
///
/// Explicit [`ApplyDeferred`] barriers between these sets ensure tree reconciliation,
/// split ratio normalization, area chrome creation, and tab/sidebar/workspace visual sync
/// all complete within the same frame before layout solving and rendering.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelSystemSet {
    /// Reconciles [`DockTree`] mutations and dirty [`WindowRegistry`] entries into ECS entities.
    Reconcile,
    /// Normalizes [`PanelGroup`] flex weights and [`PanelHandle`] visibility/orientation.
    SplitNormalize,
    /// Ensures bare [`DockArea`] entities possess tab-bar / sidebar chrome and content containers.
    AreaChrome,
    /// Synchronizes active window visibility/display, tab pills, sidebar icons, and workspace strip.
    SyncVisuals,
}

#[derive(Resource)]
struct PanelPipelineConfigured;

/// Configures ordered [`PanelSystemSet`] barriers once per [`App`], even when sub-plugins are
/// registered standalone outside [`DockPlugin`].
pub(crate) fn configure_panel_pipeline(app: &mut App) {
    if app.world().contains_resource::<PanelPipelineConfigured>() {
        return;
    }
    app.insert_resource(PanelPipelineConfigured)
        .configure_sets(
            Update,
            (
                PanelSystemSet::Reconcile,
                PanelSystemSet::SplitNormalize,
                PanelSystemSet::AreaChrome,
                PanelSystemSet::SyncVisuals,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ApplyDeferred
                    .after(PanelSystemSet::Reconcile)
                    .before(PanelSystemSet::SplitNormalize),
                ApplyDeferred
                    .after(PanelSystemSet::SplitNormalize)
                    .before(PanelSystemSet::AreaChrome),
                ApplyDeferred
                    .after(PanelSystemSet::AreaChrome)
                    .before(PanelSystemSet::SyncVisuals),
            ),
        );
}

pub use add_window_popup::{AddWindowPopup, AddWindowPopupPlugin};
pub use area::{
    ActiveDockWindow, DefaultArea, DockArea, DockAreaStyle, DockSidebarContainer, DockSidebarIcon,
    DockTab, DockTabAddButton, DockTabBar, DockTabCloseButton, DockTabContent, DockTabGrip,
    DockTabRow, DockWindow, IconFontHandle, TabActivated, TabClosed, ToAnchorId,
};
pub use drag::{ActiveDockDrag, DockDragPlugin, DockDragState, DropEdge, DropTarget};
pub use layout::{AreaState, LayoutState, capture_layout_state};
pub use reconcile::{
    DockTreeHost, LeafBinding, NodeBinding, ReconcilePlugin, TabBinding, reconcile_dock_tree_now,
};
pub use registry::{DockWindowBuildFn, DockWindowDescriptor, WindowRegistry};
pub use sidebar::DockSidebarPlugin;
pub use split::{
    Panel, PanelGroup, PanelHandle, PanelSplitChanged, SplitDirection, SplitPanelPlugin,
    normalize_panel_ratios, panel, panel_group, panel_group_with_min, panel_handle,
    panel_handle_styled, spawn_panel_handle,
};
pub use style::DockStyle;
pub use tabs::{DockTabAccent, DockTabLabel, DockTabPlugin, spawn_dock_area};
pub use tree::{
    DockLeaf, DockNode, DockSplit, DockTabEntry, DockTree, Edge, NodeId, SplitAxis, TabId,
};
pub use workspace::{
    DockWorkspaceChanged, WorkspaceDescriptor, WorkspaceEntry, WorkspacePersist, WorkspaceRegistry,
    WorkspacesPersist,
};
pub use workspace_tabs::{
    WorkspaceRenameInput, WorkspaceTab, WorkspaceTabAddButton, WorkspaceTabCloseButton,
    WorkspaceTabStrip, WorkspaceTabsPlugin, workspace_tab_strip,
};

/// Umbrella plugin registering the complete panel, docking, drag-and-drop, and workspace system.
pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DockStyle>()
            .init_resource::<DefaultArea>()
            .init_resource::<LayoutState>()
            .register_type::<DockStyle>()
            .register_type::<DefaultArea>()
            .register_type::<IconFontHandle>()
            .register_type::<DockAreaStyle>()
            .register_type::<DockArea>()
            .register_type::<ActiveDockWindow>()
            .register_type::<DockWindow>()
            .register_type::<DockTabBar>()
            .register_type::<DockTabRow>()
            .register_type::<DockTab>()
            .register_type::<DockTabCloseButton>()
            .register_type::<DockTabAddButton>()
            .register_type::<DockTabGrip>()
            .register_type::<DockTabContent>()
            .register_type::<DockSidebarContainer>()
            .register_type::<DockSidebarIcon>()
            .register_type::<NodeId>()
            .register_type::<TabId>()
            .register_type::<SplitAxis>()
            .register_type::<Edge>()
            .register_type::<DockTabEntry>()
            .register_type::<DockLeaf>()
            .register_type::<AreaState>()
            .register_type::<LayoutState>()
            .add_plugins((
                SplitPanelPlugin,
                DockTabPlugin,
                DockSidebarPlugin,
                ReconcilePlugin,
                AddWindowPopupPlugin,
                DockDragPlugin,
                WorkspaceTabsPlugin,
            ));
    }
}
