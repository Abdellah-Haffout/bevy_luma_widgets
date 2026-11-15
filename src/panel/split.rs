//! Resizable split-panel layout containers and drag handles built natively on `bevy_luma`.

use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy_luma::prelude::*;

use crate::helpers::find_ancestor_or_self;

use super::reconcile::NodeBinding;
use super::style::{DockStyle, set_window_cursor};
use super::tree::{DockTree, SplitAxis};

/// Layout axis for a [`PanelGroup`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum SplitDirection {
    /// Arranges child panels horizontally from left to right.
    #[default]
    Row,
    /// Arranges child panels vertically from top to bottom.
    Column,
}

impl SplitDirection {
    /// Maps this split direction to `bevy_luma`'s [`UFlexDirection`].
    pub fn to_flex_direction(self) -> UFlexDirection {
        match self {
            SplitDirection::Row => UFlexDirection::Row,
            SplitDirection::Column => UFlexDirection::Column,
        }
    }

    /// Returns the window cursor icon matching this split axis.
    pub fn resize_cursor(self) -> SystemCursorIcon {
        match self {
            SplitDirection::Row => SystemCursorIcon::ColResize,
            SplitDirection::Column => SystemCursorIcon::RowResize,
        }
    }
}

impl From<SplitAxis> for SplitDirection {
    fn from(axis: SplitAxis) -> Self {
        match axis {
            SplitAxis::Horizontal => SplitDirection::Row,
            SplitAxis::Vertical => SplitDirection::Column,
        }
    }
}

impl From<SplitDirection> for SplitAxis {
    fn from(dir: SplitDirection) -> Self {
        match dir {
            SplitDirection::Row => SplitAxis::Horizontal,
            SplitDirection::Column => SplitAxis::Vertical,
        }
    }
}

/// Flex container arranging resizable [`Panel`] children separated by [`PanelHandle`] dividers.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct PanelGroup {
    pub direction: SplitDirection,
    /// Minimum normalized fraction `[0.02..0.45]` each panel inside a pair must retain.
    pub min_ratio: f32,
}

impl Default for PanelGroup {
    fn default() -> Self {
        Self {
            direction: SplitDirection::Row,
            min_ratio: 0.08,
        }
    }
}

impl PanelGroup {
    /// Creates a new [`PanelGroup`] with default `min_ratio`.
    pub fn new(direction: SplitDirection) -> Self {
        Self {
            direction,
            ..default()
        }
    }

    /// Sets the minimum fractional ratio `[0.02..0.45]` per panel.
    pub fn with_min_ratio(mut self, min_ratio: f32) -> Self {
        self.min_ratio = min_ratio.clamp(0.02, 0.45);
        self
    }
}

/// Resizable panel child inside a [`PanelGroup`].
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct Panel {
    /// Relative size weight among visible sibling panels.
    pub ratio: f32,
    /// When `false`, the panel collapses cleanly and adjacent split handles hide automatically.
    pub visible: bool,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            ratio: 0.5,
            visible: true,
        }
    }
}

impl Panel {
    /// Creates a visible [`Panel`] with the given relative size ratio.
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.max(0.01),
            visible: true,
        }
    }

    /// Sets initial visibility.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

/// Interactive resize divider placed between adjacent [`Panel`] entities in a [`PanelGroup`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
#[require(HandleClickTracker)]
pub struct PanelHandle;

/// Marker on the centered grip indicator inside a [`PanelHandle`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct PanelHandleGrip;

#[derive(Component, Clone, Copy, Debug, Default)]
struct HandleClickTracker {
    last_click_secs: f64,
}

/// Emitted whenever a user drags or resets a [`PanelHandle`] between two panels.
#[derive(Event, Clone, Debug)]
pub struct PanelSplitChanged {
    pub group_entity: Entity,
    pub first_panel: Entity,
    pub second_panel: Entity,
    /// Resulting normalized fraction of the first panel within the pair (`[0..1]`).
    pub fraction: f32,
}

/// Returns the Luma bundle for spawning a [`PanelGroup`].
pub fn panel_group(direction: SplitDirection) -> impl Bundle {
    panel_group_with_min(direction, 0.08)
}

/// Returns the Luma bundle for spawning a [`PanelGroup`] with a custom `min_ratio`.
pub fn panel_group_with_min(direction: SplitDirection, min_ratio: f32) -> impl Bundle {
    (
        PanelGroup::new(direction).with_min_ratio(min_ratio),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: direction.to_flex_direction(),
            align_items: UAlignItems::Stretch,
            ..default()
        },
    )
}

/// Returns the Luma bundle for spawning a [`Panel`] inside a [`PanelGroup`].
pub fn panel(ratio: f32) -> impl Bundle {
    let clamped = ratio.max(0.01);
    (
        Panel::new(clamped),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Stretch,
            ..default()
        },
        USelf {
            flex_grow: Some(clamped),
            flex_shrink: Some(1.0),
            flex_basis: Some(UVal::Px(0.0)),
            ..default()
        },
        UClip { enabled: true },
    )
}

/// Returns the Luma bundle for spawning a [`PanelHandle`].
pub fn panel_handle() -> impl Bundle {
    panel_handle_styled(&DockStyle::default())
}

/// Returns the styled Luma bundle for spawning a [`PanelHandle`].
pub fn panel_handle_styled(style: &DockStyle) -> impl Bundle + use<> {
    (
        PanelHandle,
        UNode {
            width: UVal::Px(style.handle_thickness),
            height: UVal::Percent(1.0),
            background_color: style.handle_idle,
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            ..default()
        },
        USelf {
            flex_grow: Some(0.0),
            flex_shrink: Some(0.0),
            ..default()
        },
        UInteraction::default(),
        UInteractionColors {
            normal: style.handle_idle,
            hovered: style.handle_hover,
            pressed: style.handle_pressed,
        },
    )
}

/// Spawns a [`PanelHandle`] with an inner grab pill indicator as a child of `parent`.
pub fn spawn_panel_handle(commands: &mut Commands, parent: Entity, style: &DockStyle) -> Entity {
    let handle = commands
        .spawn((ChildOf(parent), panel_handle_styled(style)))
        .id();
    commands.spawn((
        ChildOf(handle),
        PanelHandleGrip,
        UNode {
            width: UVal::Px(2.0),
            height: UVal::Px(24.0),
            background_color: style.handle_pill_color,
            border_radius: UCornerRadius::all(2.0),
            ..default()
        },
    ));
    handle
}

/// Normalizes panel flex weights, panel visibility, and handle orientation/visibility
/// across dirty [`PanelGroup`] containers.
pub fn normalize_panel_ratios(
    style: Option<Res<DockStyle>>,
    groups: Query<(Entity, &PanelGroup, &Children)>,
    changed_groups: Query<
        Entity,
        (
            With<PanelGroup>,
            Or<(Added<PanelGroup>, Changed<PanelGroup>, Changed<Children>)>,
        ),
    >,
    changed_children: Query<
        &ChildOf,
        (
            Or<(With<Panel>, With<PanelHandle>)>,
            Or<(
                Added<Panel>,
                Changed<Panel>,
                Added<PanelHandle>,
                Changed<PanelHandle>,
            )>,
        ),
    >,
    added_grips: Query<&ChildOf, Added<PanelHandleGrip>>,
    parents: Query<&ChildOf>,
    mut dirty_groups: Local<Vec<Entity>>,
    mut group_layouts: Query<&mut ULayout, (With<PanelGroup>, Without<Panel>, Without<PanelHandle>, Without<PanelHandleGrip>)>,
    mut panels: Query<(&Panel, &mut UNode, &mut ULayout, &mut USelf, &mut Visibility), (Without<PanelGroup>, Without<PanelHandle>, Without<PanelHandleGrip>)>,
    mut handles: Query<
        (&mut UNode, &mut ULayout, &mut Visibility, Option<&Children>),
        (With<PanelHandle>, Without<Panel>, Without<PanelGroup>, Without<PanelHandleGrip>),
    >,
    mut grips: Query<&mut UNode, (With<PanelHandleGrip>, Without<PanelHandle>, Without<Panel>, Without<PanelGroup>)>,
) {
    let style_changed = style.as_ref().is_some_and(|s| s.is_changed());
    dirty_groups.clear();

    if !style_changed {
        for group_ent in changed_groups.iter() {
            if !dirty_groups.contains(&group_ent) {
                dirty_groups.push(group_ent);
            }
        }
        for child_of in changed_children.iter() {
            let group_ent = child_of.parent();
            if !dirty_groups.contains(&group_ent) {
                dirty_groups.push(group_ent);
            }
        }
        for grip_child_of in added_grips.iter() {
            if let Ok(handle_parent) = parents.get(grip_child_of.parent()) {
                let group_ent = handle_parent.parent();
                if !dirty_groups.contains(&group_ent) {
                    dirty_groups.push(group_ent);
                }
            }
        }
        if dirty_groups.is_empty() {
            return;
        }
    }

    let default_style;
    let style = if let Some(s) = style.as_deref() {
        s
    } else {
        default_style = DockStyle::default();
        &default_style
    };

    let mut normalize_group = |group_entity: Entity, group: &PanelGroup, children: &Children| {
        if let Ok(mut group_layout) = group_layouts.get_mut(group_entity) {
            let expected_dir = group.direction.to_flex_direction();
            if group_layout.flex_direction != expected_dir {
                group_layout.flex_direction = expected_dir;
            }
        }

        // Compute normalized weight for visible panels.
        let total_visible_ratio: f32 = children
            .iter()
            .filter_map(|c| panels.get(c).ok())
            .filter(|(p, _, _, _, _)| p.visible)
            .map(|(p, _, _, _, _)| p.ratio.max(0.01))
            .sum();

        for &child in children {
            if let Ok((panel, mut node, mut layout, mut uself, mut vis)) = panels.get_mut(child) {
                if !panel.visible || total_visible_ratio <= 0.0 {
                    if layout.display != UDisplay::None {
                        layout.display = UDisplay::None;
                    }
                    if *vis != Visibility::Hidden {
                        *vis = Visibility::Hidden;
                    }
                    if uself.flex_grow != Some(0.0) {
                        uself.flex_grow = Some(0.0);
                    }
                    if uself.flex_basis != Some(UVal::Px(0.0)) {
                        uself.flex_basis = Some(UVal::Px(0.0));
                    }
                    continue;
                }

                if layout.display != UDisplay::Flex {
                    layout.display = UDisplay::Flex;
                }
                if *vis != Visibility::Inherited {
                    *vis = Visibility::Inherited;
                }

                let weight = (panel.ratio.max(0.01) / total_visible_ratio).max(0.001);
                let (target_w, target_h) = match group.direction {
                    SplitDirection::Row => (UVal::Px(0.0), UVal::Percent(1.0)),
                    SplitDirection::Column => (UVal::Percent(1.0), UVal::Px(0.0)),
                };
                if node.width != target_w {
                    node.width = target_w;
                }
                if node.height != target_h {
                    node.height = target_h;
                }
                if uself.flex_grow.is_none_or(|g| (g - weight).abs() > 1e-6) {
                    uself.flex_grow = Some(weight);
                }
                if uself.flex_shrink != Some(1.0) {
                    uself.flex_shrink = Some(1.0);
                }
                if uself.flex_basis != Some(UVal::Px(0.0)) {
                    uself.flex_basis = Some(UVal::Px(0.0));
                }
            }
        }

        // Update each handle: orient dimensions and hide unless visible panels exist on both sides.
        let ordered: &[Entity] = children;
        let (target_hw, target_hh) = match group.direction {
            SplitDirection::Row => (UVal::Px(style.handle_thickness), UVal::Percent(1.0)),
            SplitDirection::Column => (UVal::Percent(1.0), UVal::Px(style.handle_thickness)),
        };
        let (target_gw, target_gh) = match group.direction {
            SplitDirection::Row => (UVal::Px(2.0), UVal::Px(24.0)),
            SplitDirection::Column => (UVal::Px(24.0), UVal::Px(2.0)),
        };

        for (idx, &child) in ordered.iter().enumerate() {
            if !handles.contains(child) {
                continue;
            }
            let has_visible_before = ordered[..idx]
                .iter()
                .rev()
                .find_map(|&e| panels.get(e).ok().map(|(p, ..)| p.visible))
                .unwrap_or(false);
            let has_visible_after = ordered[idx + 1..]
                .iter()
                .find_map(|&e| panels.get(e).ok().map(|(p, ..)| p.visible))
                .unwrap_or(false);
            let show_handle = has_visible_before && has_visible_after;

            if let Ok((mut h_node, mut h_layout, mut h_vis, h_children)) = handles.get_mut(child) {
                let target_display = if show_handle {
                    UDisplay::Flex
                } else {
                    UDisplay::None
                };
                if h_layout.display != target_display {
                    h_layout.display = target_display;
                }
                let target_vis = if show_handle {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *h_vis != target_vis {
                    *h_vis = target_vis;
                }
                if h_node.width != target_hw {
                    h_node.width = target_hw;
                }
                if h_node.height != target_hh {
                    h_node.height = target_hh;
                }
                if let Some(hc) = h_children {
                    for &g_ent in hc {
                        if let Ok(mut g_node) = grips.get_mut(g_ent) {
                            if g_node.width != target_gw {
                                g_node.width = target_gw;
                            }
                            if g_node.height != target_gh {
                                g_node.height = target_gh;
                            }
                        }
                    }
                }
            }
        }
    };

    if style_changed {
        for (group_entity, group, children) in groups.iter() {
            normalize_group(group_entity, group, children);
        }
    } else {
        for &group_entity in &*dirty_groups {
            if let Ok((_, group, children)) = groups.get(group_entity) {
                normalize_group(group_entity, group, children);
            }
        }
    }
}

fn find_adjacent_visible_panels(
    handle_entity: Entity,
    children: &Children,
    panels: &Query<&mut Panel>,
) -> Option<(Entity, Entity)> {
    let ordered: Vec<Entity> = children.iter().collect();
    let idx = ordered.iter().position(|&e| e == handle_entity)?;

    let first = ordered[..idx]
        .iter()
        .rev()
        .copied()
        .find(|&e| panels.get(e).is_ok_and(|p| p.visible))?;
    let second = ordered[idx + 1..]
        .iter()
        .copied()
        .find(|&e| panels.get(e).is_ok_and(|p| p.visible))?;

    Some((first, second))
}

fn on_panel_handle_drag(
    mut trigger: On<PointerDrag>,
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    parents: Query<&ChildOf>,
    groups: Query<(
        &PanelGroup,
        &Children,
        Option<&ComputedSize>,
        Option<&NodeBinding>,
    )>,
    handles: Query<(), With<PanelHandle>>,
    mut panels: Query<&mut Panel>,
    windows: Query<Entity, With<Window>>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(handle_entity) = find_ancestor_or_self(trigger.entity, &handles, &parents) else {
        return;
    };
    let Ok(child_of) = parents.get(handle_entity) else {
        return;
    };
    let group_entity = child_of.parent();
    let Ok((group, children, computed_opt, node_binding)) = groups.get(group_entity) else {
        return;
    };
    let Some((first_ent, second_ent)) =
        find_adjacent_visible_panels(handle_entity, children, &panels)
    else {
        return;
    };

    trigger.propagate(false);
    set_window_cursor(&mut commands, &windows, group.direction.resize_cursor());

    let handle_thickness = style
        .as_deref()
        .map(|s| s.handle_thickness)
        .unwrap_or_else(|| DockStyle::default().handle_thickness);

    let visible_handle_count = children
        .iter()
        .filter(|&c| handles.contains(c))
        .count() as f32;

    let group_main = computed_opt
        .map(|c| match group.direction {
            SplitDirection::Row => c.width,
            SplitDirection::Column => c.height,
        })
        .unwrap_or(600.0);

    let available_main = (group_main - visible_handle_count * handle_thickness).max(1.0);
    let total_visible_ratio: f32 = children
        .iter()
        .filter_map(|c| panels.get(c).ok())
        .filter(|p| p.visible)
        .map(|p| p.ratio.max(0.01))
        .sum();

    let delta_px = match group.direction {
        SplitDirection::Row => trigger.delta.x,
        SplitDirection::Column => trigger.delta.y,
    };
    if delta_px.abs() <= f32::EPSILON {
        return;
    }

    let Ok([mut first_panel, mut second_panel]) = panels.get_many_mut([first_ent, second_ent])
    else {
        return;
    };

    let pair_sum = (first_panel.ratio + second_panel.ratio).max(0.02);
    let min_ratio = (pair_sum * group.min_ratio.clamp(0.02, 0.45)).min(pair_sum * 0.49);
    let delta_ratio = (delta_px / available_main) * total_visible_ratio.max(0.01);

    let new_first = (first_panel.ratio + delta_ratio).clamp(min_ratio, pair_sum - min_ratio);
    let new_second = pair_sum - new_first;

    first_panel.ratio = new_first;
    second_panel.ratio = new_second;

    let fraction = (new_first / pair_sum).clamp(0.05, 0.95);
    if let (Some(binding), Some(tree)) = (node_binding, dock_tree.as_deref_mut()) {
        tree.set_fraction(binding.0, fraction);
    }

    commands.trigger(PanelSplitChanged {
        group_entity,
        first_panel: first_ent,
        second_panel: second_ent,
        fraction,
    });
}

fn on_panel_handle_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    time: Option<Res<Time>>,
    parents: Query<&ChildOf>,
    groups: Query<(&Children, Option<&NodeBinding>)>,
    mut handles: Query<&mut HandleClickTracker, With<PanelHandle>>,
    mut panels: Query<&mut Panel>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(handle_entity) = find_ancestor_or_self(trigger.entity, &handles, &parents) else {
        return;
    };
    let now = time.as_deref().map(|t| t.elapsed_secs_f64()).unwrap_or(0.0);
    let is_double = if let Ok(mut tracker) = handles.get_mut(handle_entity) {
        let double = tracker.last_click_secs > 0.0 && (now - tracker.last_click_secs) < 0.35;
        tracker.last_click_secs = if double { 0.0 } else { now };
        double
    } else {
        false
    };
    if !is_double {
        return;
    }

    let Ok(child_of) = parents.get(handle_entity) else {
        return;
    };
    let group_entity = child_of.parent();
    let Ok((children, node_binding)) = groups.get(group_entity) else {
        return;
    };
    let Some((first_ent, second_ent)) =
        find_adjacent_visible_panels(handle_entity, children, &panels)
    else {
        return;
    };
    trigger.propagate(false);

    let Ok([mut first_panel, mut second_panel]) = panels.get_many_mut([first_ent, second_ent])
    else {
        return;
    };
    let pair_sum = (first_panel.ratio + second_panel.ratio).max(0.02);
    first_panel.ratio = pair_sum * 0.5;
    second_panel.ratio = pair_sum * 0.5;

    if let (Some(binding), Some(tree)) = (node_binding, dock_tree.as_deref_mut()) {
        tree.set_fraction(binding.0, 0.5);
    }

    commands.trigger(PanelSplitChanged {
        group_entity,
        first_panel: first_ent,
        second_panel: second_ent,
        fraction: 0.5,
    });
}

fn on_panel_handle_over(
    trigger: On<PointerOver>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    groups: Query<&PanelGroup>,
    handles: Query<(), With<PanelHandle>>,
    windows: Query<Entity, With<Window>>,
) {
    let Some(handle_entity) = find_ancestor_or_self(trigger.entity, &handles, &parents) else {
        return;
    };
    if let Ok(child_of) = parents.get(handle_entity) {
        if let Ok(group) = groups.get(child_of.parent()) {
            set_window_cursor(&mut commands, &windows, group.direction.resize_cursor());
        }
    }
}

fn on_panel_handle_out(
    trigger: On<PointerOut>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    handles: Query<(), With<PanelHandle>>,
    windows: Query<Entity, With<Window>>,
) {
    if find_ancestor_or_self(trigger.entity, &handles, &parents).is_some() {
        set_window_cursor(&mut commands, &windows, SystemCursorIcon::Default);
    }
}

fn on_panel_handle_drag_end(
    trigger: On<PointerDragEnd>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    handles: Query<(), With<PanelHandle>>,
    windows: Query<Entity, With<Window>>,
) {
    if find_ancestor_or_self(trigger.entity, &handles, &parents).is_some() {
        set_window_cursor(&mut commands, &windows, SystemCursorIcon::Default);
    }
}

/// Plugin registering split panel groups, handles, ratio normalization, and resize interactions.
pub struct SplitPanelPlugin;

impl Plugin for SplitPanelPlugin {
    fn build(&self, app: &mut App) {
        super::configure_panel_pipeline(app);
        if !app.world().contains_resource::<DockStyle>() {
            app.init_resource::<DockStyle>();
        }
        app.register_type::<SplitDirection>()
            .register_type::<PanelGroup>()
            .register_type::<Panel>()
            .register_type::<PanelHandle>()
            .add_systems(
                Update,
                normalize_panel_ratios.in_set(super::PanelSystemSet::SplitNormalize),
            )
            .add_observer(on_panel_handle_drag)
            .add_observer(on_panel_handle_click)
            .add_observer(on_panel_handle_over)
            .add_observer(on_panel_handle_out)
            .add_observer(on_panel_handle_drag_end);
    }
}
