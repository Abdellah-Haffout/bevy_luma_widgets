//! Tab-bar chrome, interactive tab pills, close/add actions, and active window visibility sync.

use bevy::ecs::system::SystemParam;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::layout::prelude::UScrollContent;
use bevy_luma::prelude::*;

use crate::helpers::find_ancestor_or_self;

use super::area::{
    ActiveDockWindow, DockArea, DockAreaStyle, DockSidebarContainer, DockTab, DockTabAddButton,
    DockTabBar, DockTabCloseButton, DockTabContent, DockTabGrip, DockTabRow, DockWindow,
    IconFontHandle, TabActivated, TabClosed,
};
use super::reconcile::LeafBinding;
use super::style::{DockStyle, resolve_fonts, set_interaction_colors_if_changed};
use super::tree::DockTree;

/// Marker on the top active indicator bar inside a [`DockTab`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTabAccent;

/// Marker on the title text label inside a [`DockTab`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTabLabel;

/// Signature cached on a [`DockArea`] to avoid redundant tab/sidebar rebuilds when only selection changes.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AreaTabsSignature {
    pub(crate) style: DockAreaStyle,
    pub(crate) items: Vec<(Entity, String, Option<String>, bool)>,
}

/// Spawns a complete [`DockArea`] container with its [`DockTabContent`] and matching chrome
/// (`DockTabBar` or `DockSidebarContainer`), returning `(area_entity, content_entity)`.
pub fn spawn_dock_area(
    commands: &mut Commands,
    parent: Option<Entity>,
    area: DockArea,
    style: &DockStyle,
    text_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> (Entity, Entity) {
    let flex_dir = match area.style {
        DockAreaStyle::IconSidebar => UFlexDirection::Row,
        DockAreaStyle::TabBar | DockAreaStyle::Headless => UFlexDirection::Column,
    };

    let mut area_cmd = commands.spawn((
        area.clone(),
        ActiveDockWindow(None),
        AreaTabsSignature::default(),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: style.area_bg,
            border_radius: UCornerRadius::all(style.area_radius),
            ..default()
        },
        UBorder {
            color: style.area_border,
            width: style.area_border_width,
            radius: UCornerRadius::all(style.area_radius),
            offset: 0.0,
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: flex_dir,
            align_items: UAlignItems::Stretch,
            ..default()
        },
        UClip { enabled: true },
    ));
    if let Some(p) = parent {
        area_cmd.insert(ChildOf(p));
    }
    let area_entity = area_cmd.id();

    match area.style {
        DockAreaStyle::TabBar => {
            spawn_tab_bar_chrome(commands, area_entity, style, icon_font);
        }
        DockAreaStyle::IconSidebar => {
            spawn_sidebar_chrome(commands, area_entity, style);
        }
        DockAreaStyle::Headless => {}
    }

    let _ = text_font;
    let content_entity = spawn_tab_content_container(commands, area_entity, style);
    (area_entity, content_entity)
}

pub(crate) fn spawn_tab_bar_chrome(
    commands: &mut Commands,
    area_entity: Entity,
    style: &DockStyle,
    icon_font: Handle<Font>,
) -> Entity {
    let tab_bar = commands
        .spawn((
            ChildOf(area_entity),
            DockTabBar,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(style.tab_bar_height),
                padding: USides::axes(4.0, 0.0),
                background_color: style.tab_bar_bg,
                ..default()
            },
            UBorder {
                color: style.tab_bar_border,
                width: 1.0,
                radius: UCornerRadius::default(),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
            USelf {
                flex_grow: Some(0.0),
                flex_shrink: Some(0.0),
                ..default()
            },
        ))
        .id();

    // Scrollable horizontal viewport around the tab row so many tabs remain cleanly accessible.
    let scroll_vp = commands
        .spawn((
            ChildOf(tab_bar),
            UScrollContainer {
                vertical: false,
                horizontal: true,
                ..UScrollContainer::horizontal()
            },
            UNode {
                width: UVal::Px(0.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Stretch,
                ..default()
            },
            USelf {
                flex_grow: Some(1.0),
                flex_shrink: Some(1.0),
                flex_basis: Some(UVal::Px(0.0)),
                ..default()
            },
            UClip { enabled: true },
        ))
        .id();

    commands.spawn((
        ChildOf(scroll_vp),
        DockTabRow,
        UScrollContent::default(),
        UNode {
            width: UVal::Auto,
            height: UVal::Percent(1.0),
            padding: USides::top(3.0),
            background_color: Color::NONE,
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Stretch,
            gap: style.tab_gap,
            ..default()
        },
    ));

    // Right action cluster (`+` add window, `⋮⋮` drag grip).
    let actions = commands
        .spawn((
            ChildOf(tab_bar),
            UNode {
                width: UVal::Auto,
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 2.0,
                ..default()
            },
            USelf {
                flex_grow: Some(0.0),
                flex_shrink: Some(0.0),
                ..default()
            },
        ))
        .id();

    let add_btn = commands
        .spawn((
            ChildOf(actions),
            DockTabAddButton { area_entity },
            UNode {
                width: UVal::Px(24.0),
                height: UVal::Px(24.0),
                border_radius: UCornerRadius::all(4.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::NONE,
                hovered: style.action_hover_bg,
                pressed: style.action_hover_bg,
            },
        ))
        .id();
    commands.spawn((
        ChildOf(add_btn),
        UNode::default(),
        UText {
            text: Icon::PLUS.to_string(),
            font_size: 14.0,
            font: icon_font.clone(),
            color: style.icon_color,
            ..default()
        },
    ));

    let grip_btn = commands
        .spawn((
            ChildOf(actions),
            DockTabGrip { area_entity },
            UNode {
                width: UVal::Px(20.0),
                height: UVal::Px(24.0),
                border_radius: UCornerRadius::all(4.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: Color::NONE,
                hovered: style.action_hover_bg,
                pressed: style.action_hover_bg,
            },
        ))
        .id();
    commands.spawn((
        ChildOf(grip_btn),
        UNode::default(),
        UText {
            text: Icon::GRIP_VERTICAL.to_string(),
            font_size: 13.0,
            font: icon_font,
            color: style.icon_color,
            ..default()
        },
    ));

    tab_bar
}

pub(crate) fn spawn_sidebar_chrome(
    commands: &mut Commands,
    area_entity: Entity,
    style: &DockStyle,
) -> Entity {
    commands
        .spawn((
            ChildOf(area_entity),
            DockSidebarContainer,
            UNode {
                width: UVal::Px(style.sidebar_width),
                height: UVal::Percent(1.0),
                padding: USides::axes(4.0, 6.0),
                background_color: style.sidebar_bg,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
            USelf {
                flex_grow: Some(0.0),
                flex_shrink: Some(0.0),
                ..default()
            },
        ))
        .id()
}

pub(crate) fn spawn_tab_content_container(
    commands: &mut Commands,
    area_entity: Entity,
    style: &DockStyle,
) -> Entity {
    commands
        .spawn((
            ChildOf(area_entity),
            DockTabContent,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: style.area_bg,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Stretch,
                ..default()
            },
            USelf {
                flex_grow: Some(1.0),
                flex_shrink: Some(1.0),
                flex_basis: Some(UVal::Px(0.0)),
                ..default()
            },
            UClip { enabled: true },
        ))
        .id()
}

/// Collects `(window_entity, window_ref)` ordered children under an area's [`DockTabContent`].
pub(crate) fn area_windows(
    area_entity: Entity,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<DockTabContent>>,
    window_q: &Query<&DockWindow>,
) -> Vec<(Entity, DockWindow)> {
    let Some(content_ent) = find_area_content_entity(area_entity, children_q, content_q) else {
        return Vec::new();
    };
    let Ok(content_children) = children_q.get(content_ent) else {
        return Vec::new();
    };
    content_children
        .iter()
        .filter_map(|e| window_q.get(e).ok().map(|w| (e, w.clone())))
        .collect()
}

/// Counts live [`DockWindow`] children under an area's [`DockTabContent`] without allocating or cloning.
pub(crate) fn count_area_windows(
    area_entity: Entity,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<DockTabContent>>,
    window_q: &Query<&DockWindow>,
) -> usize {
    let Some(content_ent) = find_area_content_entity(area_entity, children_q, content_q) else {
        return 0;
    };
    let Ok(content_children) = children_q.get(content_ent) else {
        return 0;
    };
    content_children
        .iter()
        .filter(|&e| window_q.contains(e))
        .count()
}

/// Locates the [`DockTabContent`] child inside `area_entity`.
pub(crate) fn find_area_content_entity(
    area_entity: Entity,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<DockTabContent>>,
) -> Option<Entity> {
    let children = children_q.get(area_entity).ok()?;
    children.iter().find(|&c| content_q.contains(c))
}

/// Locates the [`DockTabRow`] inside `area_entity`.
pub(crate) fn find_area_tab_row(
    area_entity: Entity,
    children_q: &Query<&Children>,
    tab_bar_q: &Query<(), With<DockTabBar>>,
    tab_row_q: &Query<(), With<DockTabRow>>,
) -> Option<Entity> {
    let children = children_q.get(area_entity).ok()?;
    for &child in children {
        if tab_bar_q.contains(child) {
            // Recurse up to scroll viewport -> DockTabRow
            if let Ok(bar_children) = children_q.get(child) {
                for &bc in bar_children {
                    if tab_row_q.contains(bc) {
                        return Some(bc);
                    }
                    if let Ok(vp_children) = children_q.get(bc) {
                        if let Some(row) = vp_children.iter().find(|&v| tab_row_q.contains(v)) {
                            return Some(row);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Activates `window_entity` inside `area_entity` and syncs [`DockTree`] if bound to a leaf.
pub(crate) fn activate_window_in_area(
    commands: &mut Commands,
    area_entity: Entity,
    window_entity: Entity,
    active_q: &mut Query<&mut ActiveDockWindow>,
    leaf_bindings: &Query<&LeafBinding>,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<DockTabContent>>,
    window_q: &Query<&DockWindow>,
    dock_tree: Option<&mut DockTree>,
) {
    let Ok(win) = window_q.get(window_entity) else {
        return;
    };
    let window_id = win.window_id.clone();

    if let Ok(mut active) = active_q.get_mut(area_entity) {
        active.0 = Some(window_entity);
    }

    if let (Some(tree), Ok(binding)) = (dock_tree, leaf_bindings.get(area_entity)) {
        if let Some(content_ent) = find_area_content_entity(area_entity, children_q, content_q) {
            if let Ok(content_children) = children_q.get(content_ent) {
                if let Some(tab_index) = content_children
                    .iter()
                    .filter(|&c| window_q.contains(c))
                    .position(|c| c == window_entity)
                {
                    tree.set_active(binding.0, tab_index);
                }
            }
        }
    }

    commands.trigger(TabActivated {
        area_entity,
        window_entity,
        window_id,
    });
}

/// Closes `window_entity` in `area_entity`, updating [`DockTree`] when bound or managing standalone areas.
pub(crate) fn close_window_in_area(
    commands: &mut Commands,
    area_entity: Entity,
    window_entity: Entity,
    active_q: &mut Query<&mut ActiveDockWindow>,
    leaf_bindings: &Query<&LeafBinding>,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<DockTabContent>>,
    window_q: &Query<&DockWindow>,
    dock_tree: Option<&mut DockTree>,
) {
    let Ok(win) = window_q.get(window_entity) else {
        return;
    };
    if !win.closeable {
        return;
    }
    let window_id = win.window_id.clone();

    let windows_in_area = area_windows(area_entity, children_q, content_q, window_q);
    let tab_index = windows_in_area
        .iter()
        .position(|(e, _)| *e == window_entity);

    commands.trigger(TabClosed {
        area_entity,
        window_entity,
        window_id,
    });

    if let (Some(tree), Ok(binding), Some(idx)) = (dock_tree, leaf_bindings.get(area_entity), tab_index)
    {
        tree.remove_tab(binding.0, idx);
        return;
    }

    // Standalone (unmanaged) DockArea fallback: despawn window entity and pick neighbor.
    if let Ok(mut cmd) = commands.get_entity(window_entity) {
        cmd.try_despawn();
    }
    if let Ok(mut active) = active_q.get_mut(area_entity) {
        if active.0 == Some(window_entity) {
            let next = windows_in_area
                .iter()
                .filter(|(e, _)| *e != window_entity)
                .map(|(e, _)| *e)
                .next();
            active.0 = next;
        }
    }
}

/// Ensures manually spawned [`DockArea`]s receive [`ActiveDockWindow`], chrome, and [`DockTabContent`].
fn ensure_area_structure(
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    areas: Query<
        (
            Entity,
            &DockArea,
            Option<&ActiveDockWindow>,
            Option<&AreaTabsSignature>,
            Option<&Children>,
        ),
        Or<(Added<DockArea>, Changed<DockArea>)>,
    >,
    tab_bar_q: Query<(), With<DockTabBar>>,
    sidebar_q: Query<(), With<DockSidebarContainer>>,
    content_q: Query<(), With<DockTabContent>>,
    mut area_layouts: Query<&mut ULayout, With<DockArea>>,
) {
    if areas.is_empty() {
        return;
    }

    let default_style;
    let style = match style.as_deref() {
        Some(s) => s,
        None => {
            default_style = DockStyle::default();
            &default_style
        }
    };
    let (_, icon_font) = resolve_fonts(theme.as_deref(), icon_font_res.as_deref());

    for (area_ent, area, active_opt, sig_opt, children_opt) in areas.iter() {
        if active_opt.is_none() {
            commands.entity(area_ent).try_insert(ActiveDockWindow(None));
        }
        if sig_opt.is_none() {
            commands
                .entity(area_ent)
                .try_insert(AreaTabsSignature::default());
        }

        if let Ok(mut layout) = area_layouts.get_mut(area_ent) {
            let expected_dir = match area.style {
                DockAreaStyle::IconSidebar => UFlexDirection::Row,
                DockAreaStyle::TabBar | DockAreaStyle::Headless => UFlexDirection::Column,
            };
            if layout.flex_direction != expected_dir {
                layout.flex_direction = expected_dir;
            }
        }

        let existing_tab_bar = children_opt
            .and_then(|c| c.iter().find(|&e| tab_bar_q.contains(e)));
        let existing_sidebar = children_opt
            .and_then(|c| c.iter().find(|&e| sidebar_q.contains(e)));
        let has_content = children_opt
            .is_some_and(|c| c.iter().any(|e| content_q.contains(e)));

        match area.style {
            DockAreaStyle::TabBar => {
                if let Some(sb) = existing_sidebar {
                    if let Ok(mut cmd) = commands.get_entity(sb) {
                        cmd.try_despawn();
                    }
                }
                if existing_tab_bar.is_none() {
                    spawn_tab_bar_chrome(&mut commands, area_ent, style, icon_font.clone());
                }
            }
            DockAreaStyle::IconSidebar => {
                if let Some(tb) = existing_tab_bar {
                    if let Ok(mut cmd) = commands.get_entity(tb) {
                        cmd.try_despawn();
                    }
                }
                if existing_sidebar.is_none() {
                    spawn_sidebar_chrome(&mut commands, area_ent, style);
                }
            }
            DockAreaStyle::Headless => {
                if let Some(tb) = existing_tab_bar {
                    if let Ok(mut cmd) = commands.get_entity(tb) {
                        cmd.try_despawn();
                    }
                }
                if let Some(sb) = existing_sidebar {
                    if let Ok(mut cmd) = commands.get_entity(sb) {
                        cmd.try_despawn();
                    }
                }
            }
        }

        if !has_content {
            spawn_tab_content_container(&mut commands, area_ent, style);
        }
    }
}

#[derive(SystemParam)]
struct TabDirtyQueries<'w, 's> {
    just_area_q: Query<'w, 's, (), With<DockArea>>,
    changed_contents: Query<
        'w,
        's,
        &'static ChildOf,
        (
            With<DockTabContent>,
            Or<(Added<DockTabContent>, Changed<Children>)>,
        ),
    >,
    changed_windows: Query<
        'w,
        's,
        &'static ChildOf,
        (With<DockWindow>, Or<(Added<DockWindow>, Changed<DockWindow>)>),
    >,
    added_tab_chrome: Query<'w, 's, &'static ChildOf, Or<(Added<DockTabBar>, Added<DockTabRow>)>>,
    parents_q: Query<'w, 's, &'static ChildOf>,
}

#[derive(SystemParam)]
struct TabTreeQueries<'w, 's> {
    children_q: Query<'w, 's, &'static Children>,
    content_q: Query<'w, 's, (), With<DockTabContent>>,
    tab_bar_q: Query<'w, 's, (), With<DockTabBar>>,
    tab_row_q: Query<'w, 's, (), With<DockTabRow>>,
    window_meta_q: Query<'w, 's, &'static DockWindow>,
}

/// Synchronizes active tab window visibility, tab bar items, and active tab colors.
fn sync_tabs_and_visibility(
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    mut areas: Query<(Entity, Ref<DockArea>, &mut ActiveDockWindow, &mut AreaTabsSignature)>,
    dirty: TabDirtyQueries,
    mut dirty_areas: Local<Vec<Entity>>,
    tree: TabTreeQueries,
    mut window_layout_q: Query<
        (&mut ULayout, &mut Visibility, &mut Transform),
        (With<DockWindow>, Without<DockTab>, Without<DockTabAccent>),
    >,
    mut tabs_q: Query<
        (Entity, &DockTab, &mut UNode, &mut UInteractionColors),
        (Without<DockWindow>, Without<DockTabAccent>),
    >,
    mut accents_q: Query<
        (&mut UNode, &mut Visibility),
        (With<DockTabAccent>, Without<DockTab>, Without<DockWindow>),
    >,
    mut tab_labels_q: Query<
        &mut UText,
        (
            With<DockTabLabel>,
            Without<DockTab>,
            Without<DockTabAccent>,
            Without<DockWindow>,
        ),
    >,
) {
    let visual_res_changed = style.as_ref().is_some_and(|s| s.is_changed())
        || theme.as_ref().is_some_and(|t| t.is_changed())
        || icon_font_res.as_ref().is_some_and(|f| f.is_changed());

    dirty_areas.clear();
    if !visual_res_changed {
        for co in dirty.changed_contents.iter() {
            if !dirty_areas.contains(&co.parent()) {
                dirty_areas.push(co.parent());
            }
        }
        for co in dirty.changed_windows.iter() {
            if let Ok(content_co) = dirty.parents_q.get(co.parent()) {
                let area_ent = content_co.parent();
                if !dirty_areas.contains(&area_ent) {
                    dirty_areas.push(area_ent);
                }
            }
        }
        for co in dirty.added_tab_chrome.iter() {
            if let Some(area_ent) =
                find_ancestor_or_self(co.parent(), &dirty.just_area_q, &dirty.parents_q)
            {
                if !dirty_areas.contains(&area_ent) {
                    dirty_areas.push(area_ent);
                }
            }
        }
    }

    let mut default_style_storage: Option<DockStyle> = None;
    let mut cached_fonts: Option<(Handle<Font>, Handle<Font>)> = None;

    for (area_ent, area, mut active, mut sig) in areas.iter_mut() {
        let is_dirty = visual_res_changed
            || area.is_added()
            || area.is_changed()
            || active.is_added()
            || active.is_changed()
            || dirty_areas.contains(&area_ent);
        if !is_dirty {
            continue;
        }

        let style = match style.as_deref() {
            Some(s) => s,
            None => default_style_storage.get_or_insert_with(DockStyle::default),
        };
        let (text_font, icon_font) = cached_fonts
            .get_or_insert_with(|| resolve_fonts(theme.as_deref(), icon_font_res.as_deref()));

        let windows_in_area = area_windows(
            area_ent,
            &tree.children_q,
            &tree.content_q,
            &tree.window_meta_q,
        );

        // Ensure ActiveDockWindow points to an existing window when tabs exist.
        let valid_active = active
            .0
            .filter(|e| windows_in_area.iter().any(|(we, _)| we == e));
        let resolved_active = valid_active.or_else(|| windows_in_area.first().map(|(e, _)| *e));
        if active.0 != resolved_active {
            active.0 = resolved_active;
        }

        // Toggle UDisplay::Flex / UDisplay::None on docked windows.
        for (win_ent, _) in &windows_in_area {
            let is_active = Some(*win_ent) == resolved_active;
            if let Ok((mut layout, mut vis, mut transform)) = window_layout_q.get_mut(*win_ent) {
                let target_display = if is_active {
                    UDisplay::Flex
                } else {
                    UDisplay::None
                };
                if layout.display != target_display {
                    layout.display = target_display;
                }
                let target_vis = if is_active {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *vis != target_vis {
                    *vis = target_vis;
                }
                if is_active && transform.scale == Vec3::ZERO {
                    transform.scale = Vec3::ONE;
                }
            }
        }

        if area.style != DockAreaStyle::TabBar {
            continue;
        }

        let Some(tab_row_ent) = find_area_tab_row(
            area_ent,
            &tree.children_q,
            &tree.tab_bar_q,
            &tree.tab_row_q,
        ) else {
            continue;
        };

        let sig_matches = !visual_res_changed
            && sig.style == area.style
            && sig.items.len() == windows_in_area.len()
            && sig.items.iter().zip(windows_in_area.iter()).all(
                |((s_ent, s_name, s_icon, s_closeable), (w_ent, w))| {
                    *s_ent == *w_ent
                        && *s_name == w.name
                        && *s_icon == w.icon
                        && *s_closeable == w.closeable
                },
            );

        if !sig_matches {
            *sig = AreaTabsSignature {
                style: area.style,
                items: windows_in_area
                    .iter()
                    .map(|(e, w)| (*e, w.name.clone(), w.icon.clone(), w.closeable))
                    .collect(),
            };
            if let Ok(existing) = tree.children_q.get(tab_row_ent) {
                for &c in existing {
                    if let Ok(mut cmd) = commands.get_entity(c) {
                        cmd.try_despawn();
                    }
                }
            }
            for (idx, (win_ent, win)) in windows_in_area.iter().enumerate() {
                let is_active = Some(*win_ent) == resolved_active;
                spawn_tab_pill(
                    &mut commands,
                    tab_row_ent,
                    area_ent,
                    *win_ent,
                    idx,
                    win,
                    is_active,
                    style,
                    text_font.clone(),
                    icon_font.clone(),
                );
            }
        } else if let Ok(row_children) = tree.children_q.get(tab_row_ent) {
            // Fast path: only update active/inactive styling on existing pills.
            for &tab_ent in row_children {
                let Ok((_, tab, mut node, mut icolors)) = tabs_q.get_mut(tab_ent) else {
                    continue;
                };
                let is_active = Some(tab.window_entity) == resolved_active;
                let bg = if is_active {
                    style.tab_bg_active
                } else {
                    style.tab_bg_inactive
                };
                if node.background_color != bg {
                    node.background_color = bg;
                }
                let next_icolors = UInteractionColors {
                    normal: bg,
                    hovered: if is_active { bg } else { style.tab_bg_hover },
                    pressed: style.tab_bg_active,
                };
                set_interaction_colors_if_changed(&mut icolors, next_icolors);
                if let Ok(tc) = tree.children_q.get(tab_ent) {
                    for &c in tc {
                        if let Ok((mut anode, mut avis)) = accents_q.get_mut(c) {
                            let target_vis = if is_active {
                                Visibility::Inherited
                            } else {
                                Visibility::Hidden
                            };
                            if *avis != target_vis {
                                *avis = target_vis;
                            }
                            if anode.background_color != style.accent_color {
                                anode.background_color = style.accent_color;
                            }
                        }
                        if let Ok(mut label) = tab_labels_q.get_mut(c) {
                            let target_color = if is_active {
                                style.tab_text_active
                            } else {
                                style.tab_text_inactive
                            };
                            if label.color != target_color {
                                label.color = target_color;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tab_pill(
    commands: &mut Commands,
    tab_row: Entity,
    area_entity: Entity,
    window_entity: Entity,
    tab_index: usize,
    win: &DockWindow,
    is_active: bool,
    style: &DockStyle,
    text_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Entity {
    let bg = if is_active {
        style.tab_bg_active
    } else {
        style.tab_bg_inactive
    };
    let text_color = if is_active {
        style.tab_text_active
    } else {
        style.tab_text_inactive
    };

    let tab_ent = commands
        .spawn((
            ChildOf(tab_row),
            DockTab {
                window_entity,
                area_entity,
                tab_index,
            },
            UNode {
                width: UVal::Auto,
                height: UVal::Percent(1.0),
                padding: USides::axes(10.0, 0.0),
                background_color: bg,
                border_radius: UCornerRadius::top(style.tab_radius),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: bg,
                hovered: if is_active { bg } else { style.tab_bg_hover },
                pressed: style.tab_bg_active,
            },
        ))
        .id();

    // Active top accent bar.
    commands.spawn((
        ChildOf(tab_ent),
        DockTabAccent,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(style.tab_accent_thickness),
            background_color: style.accent_color,
            ..default()
        },
        USelf {
            position_type: UPositionType::Absolute,
            left: UVal::Px(0.0),
            right: UVal::Px(0.0),
            top: UVal::Px(0.0),
            ..default()
        },
        if is_active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        },
    ));

    if let Some(icon) = &win.icon {
        commands.spawn((
            ChildOf(tab_ent),
            UNode::default(),
            UText {
                text: icon.clone(),
                font_size: 13.0,
                font: icon_font.clone(),
                color: text_color,
                ..default()
            },
        ));
    }

    commands.spawn((
        ChildOf(tab_ent),
        DockTabLabel,
        UNode::default(),
        UText {
            text: win.name.clone(),
            font_size: style.tab_font_size,
            font: text_font,
            color: text_color,
            ..default()
        },
    ));

    if win.closeable {
        let close_btn = commands
            .spawn((
                ChildOf(tab_ent),
                DockTabCloseButton {
                    window_entity,
                    area_entity,
                },
                UNode {
                    width: UVal::Px(18.0),
                    height: UVal::Px(18.0),
                    border_radius: UCornerRadius::all(4.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    ..default()
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: Color::NONE,
                    hovered: style.close_hover_bg,
                    pressed: style.close_hover_bg,
                },
            ))
            .id();

        commands.spawn((
            ChildOf(close_btn),
            UNode::default(),
            UText {
                text: Icon::X.to_string(),
                font_size: 11.0,
                font: icon_font,
                color: style.icon_color,
                ..default()
            },
        ));
    }

    tab_ent
}

fn on_dock_tab_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    tabs_q: Query<&DockTab>,
    close_q: Query<(), With<DockTabCloseButton>>,
    mut active_q: Query<&mut ActiveDockWindow>,
    leaf_bindings: Query<&LeafBinding>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    // Let close button handle its own clicks without selecting the closing tab.
    if find_ancestor_or_self(trigger.entity, &close_q, &parents).is_some() {
        return;
    }
    let Some(tab_ent) = find_ancestor_or_self(trigger.entity, &tabs_q, &parents) else {
        return;
    };
    let Ok(tab) = tabs_q.get(tab_ent) else {
        return;
    };

    match trigger.event().button {
        PointerButton::Primary => {
            trigger.propagate(false);
            activate_window_in_area(
                &mut commands,
                tab.area_entity,
                tab.window_entity,
                &mut active_q,
                &leaf_bindings,
                &children_q,
                &content_q,
                &window_q,
                dock_tree.as_deref_mut(),
            );
        }
        PointerButton::Middle => {
            trigger.propagate(false);
            close_window_in_area(
                &mut commands,
                tab.area_entity,
                tab.window_entity,
                &mut active_q,
                &leaf_bindings,
                &children_q,
                &content_q,
                &window_q,
                dock_tree.as_deref_mut(),
            );
        }
        _ => {}
    }
}

fn on_dock_tab_close_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    close_q: Query<&DockTabCloseButton>,
    mut active_q: Query<&mut ActiveDockWindow>,
    leaf_bindings: Query<&LeafBinding>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(btn_ent) = find_ancestor_or_self(trigger.entity, &close_q, &parents) else {
        return;
    };
    let Ok(btn) = close_q.get(btn_ent) else {
        return;
    };

    trigger.propagate(false);
    close_window_in_area(
        &mut commands,
        btn.area_entity,
        btn.window_entity,
        &mut active_q,
        &leaf_bindings,
        &children_q,
        &content_q,
        &window_q,
        dock_tree.as_deref_mut(),
    );
}

/// Plugin registering tab-bar layout sync and click/middle-click/close observers.
pub struct DockTabPlugin;

impl Plugin for DockTabPlugin {
    fn build(&self, app: &mut App) {
        super::configure_panel_pipeline(app);
        app.add_systems(
            Update,
            (
                ensure_area_structure.in_set(super::PanelSystemSet::AreaChrome),
                sync_tabs_and_visibility.in_set(super::PanelSystemSet::SyncVisuals),
            ),
        )
        .add_observer(on_dock_tab_click)
        .add_observer(on_dock_tab_close_click);
    }
}
