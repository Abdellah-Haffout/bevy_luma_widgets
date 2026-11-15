//! Comprehensive unit and ECS integration tests for the `panel` docking system.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::camera::NormalizedRenderTarget;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::{ButtonState, InputPlugin};
use bevy::picking::backend::HitData;
use bevy::picking::events::{Pointer, PointerClick, PointerDrag, PointerDragEnd, PointerDragStart};
use bevy::picking::pointer::{Location, PointerButton, PointerId};
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::text::TextPlugin;
use bevy_luma::interaction_crate::interaction::text_input::UTextInputSubmit;
use bevy_luma::prelude::*;

use crate::LumaWidgetsPlugin;

use super::*;

fn test_panel_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        TextPlugin,
        ScenePlugin,
        InputPlugin,
        LumaWidgetsPlugin,
    ));
    app
}

fn test_pointer(pos: Vec2) -> Pointer {
    Pointer::new(
        PointerId::Mouse,
        Location {
            target: NormalizedRenderTarget::None {
                width: 1280,
                height: 720,
            },
            position: pos,
        },
    )
}

fn dummy_hit(entity: Entity) -> HitData {
    HitData::new(entity, 0.0, None, None)
}

fn trigger_click(app: &mut App, entity: Entity, button: PointerButton) {
    app.world_mut().trigger(PointerClick {
        entity,
        pointer: test_pointer(Vec2::ZERO),
        button,
        hit: dummy_hit(entity),
        duration: Duration::from_millis(10),
        count: 1,
    });
}

fn press_escape(app: &mut App) {
    app.world_mut().write_message(KeyboardInput {
        key_code: KeyCode::Escape,
        logical_key: Key::Escape,
        state: ButtonState::Pressed,
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    });
}

fn assert_serde<T: serde::Serialize + for<'de> serde::Deserialize<'de>>() {}

#[test]
fn serde_bounds_compile_for_tree_layout_and_workspaces() {
    assert_serde::<DockTree>();
    assert_serde::<LayoutState>();
    assert_serde::<WorkspacesPersist>();
}

#[test]
fn dock_tree_operations_and_queries() {
    let leaf1 = DockLeaf::new(
        NodeId(1),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(2), "scene", "Scene"),
            DockTabEntry::new(TabId(3), "console", "Console"),
        ],
    )
    .with_name("main");

    let mut tree = DockTree::new(leaf1);
    assert!(tree.has_window_kind("scene"));
    assert_eq!(tree.find_window_kind("console"), Some((NodeId(1), 1, TabId(3))));

    tree.alias_window_kind("console", "log_viewer");
    assert!(!tree.has_window_kind("console"));
    assert!(tree.has_window_kind("log_viewer"));

    let t4 = tree.add_tab(NodeId(1), "inspector", "Inspector").unwrap();
    assert_eq!(tree.find_leaf(NodeId(1)).unwrap().active, 2);
    assert_eq!(tree.find_tab(t4), Some((NodeId(1), 2)));

    // Same-leaf reorder: move tab 2 -> index 0
    assert!(tree.move_tab(NodeId(1), 2, NodeId(1), 0));
    assert_eq!(tree.find_leaf(NodeId(1)).unwrap().tabs[0].tab_id, t4);

    // Move tab 0 to new split right (35% allocated to the newly created right leaf -> 65% first)
    let split_id = tree
        .move_tab_to_new_split(NodeId(1), 0, NodeId(1), Edge::Right, 0.35)
        .unwrap();
    let split = tree.find_split(split_id).unwrap();
    assert_eq!(split.axis, SplitAxis::Horizontal);
    assert!((split.fraction - 0.65).abs() < 1e-4);

    // Remove all log_viewer tabs across tree
    tree.remove_window_kind("log_viewer");
    assert!(!tree.has_window_kind("log_viewer"));

    let snap = LayoutState::from_tree(&tree);
    assert_eq!(snap.areas.len(), 2);
}

#[test]
fn split_panel_normalization_drag_and_double_click_reset() {
    let mut app = test_panel_app();

    let group_ent = app.world_mut().spawn(panel_group(SplitDirection::Row)).id();
    let p1 = app.world_mut().spawn((ChildOf(group_ent), panel(0.3))).id();
    let handle = spawn_panel_handle(
        &mut app.world_mut().commands(),
        group_ent,
        &DockStyle::default(),
    );
    let p2 = app.world_mut().spawn((ChildOf(group_ent), panel(0.7))).id();
    app.world_mut()
        .entity_mut(group_ent)
        .insert(ComputedSize {
            width: 1006.0,
            height: 600.0,
            ..default()
        });
    app.update();

    let g1 = app.world().get::<USelf>(p1).unwrap().flex_grow.unwrap();
    let g2 = app.world().get::<USelf>(p2).unwrap().flex_grow.unwrap();
    assert!((g1 - 0.3).abs() < 1e-3);
    assert!((g2 - 0.7).abs() < 1e-3);

    // Drag handle right by 200px (out of 1000px available -> +0.20)
    app.world_mut().trigger(PointerDrag {
        entity: handle,
        pointer: test_pointer(Vec2::new(500.0, 300.0)),
        button: PointerButton::Primary,
        distance: Vec2::new(200.0, 0.0),
        delta: Vec2::new(200.0, 0.0),
    });
    app.update();

    let r1 = app.world().get::<Panel>(p1).unwrap().ratio;
    let r2 = app.world().get::<Panel>(p2).unwrap().ratio;
    assert!((r1 - 0.5).abs() < 1e-2, "expected ~0.5, got {r1}");
    assert!((r2 - 0.5).abs() < 1e-2, "expected ~0.5, got {r2}");

    // Drag far left -> clamps to min_ratio (0.08)
    app.world_mut().trigger(PointerDrag {
        entity: handle,
        pointer: test_pointer(Vec2::new(10.0, 300.0)),
        button: PointerButton::Primary,
        distance: Vec2::new(-800.0, 0.0),
        delta: Vec2::new(-800.0, 0.0),
    });
    app.update();
    assert!((app.world().get::<Panel>(p1).unwrap().ratio - 0.08).abs() < 1e-3);

    // Hiding second panel automatically hides the adjacent PanelHandle.
    app.world_mut().get_mut::<Panel>(p2).unwrap().visible = false;
    app.update();
    assert_eq!(
        app.world().get::<ULayout>(handle).unwrap().display,
        UDisplay::None
    );

    // Restore visibility & verify double-click resets split to 0.5
    app.world_mut().get_mut::<Panel>(p2).unwrap().visible = true;
    app.update();
    assert_eq!(
        app.world().get::<ULayout>(handle).unwrap().display,
        UDisplay::Flex
    );

    for _ in 0..2 {
        trigger_click(&mut app, handle, PointerButton::Primary);
        app.update();
    }
    let reset_r1 = app.world().get::<Panel>(p1).unwrap().ratio;
    assert!((reset_r1 - 0.5).abs() < 1e-3, "expected reset 0.5, got {reset_r1}");
}

#[test]
fn reconcile_tabs_close_middle_click_and_hot_reload() {
    let mut app = test_panel_app();
    let build_count = Arc::new(AtomicUsize::new(0));
    let bc = build_count.clone();

    {
        let mut reg = app.world_mut().resource_mut::<WindowRegistry>();
        reg.register(DockWindowDescriptor::new("a", "Window A", move |cmds, parent| {
            bc.fetch_add(1, Ordering::SeqCst);
            cmds.spawn((ChildOf(parent), UNode::default()));
        }));
        reg.register(DockWindowDescriptor::new("b", "Window B", |cmds, parent| {
            cmds.spawn((ChildOf(parent), UNode::default()));
        }));
    }

    let leaf = DockLeaf::new(
        NodeId(1),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(2), "a", "Window A"),
            DockTabEntry::new(TabId(3), "b", "Window B"),
        ],
    );
    *app.world_mut().resource_mut::<DockTree>() = DockTree::new(leaf);

    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));
    app.update();
    app.update();

    assert_eq!(build_count.load(Ordering::SeqCst), 1);

    let mut area_q = app.world_mut().query::<(Entity, &LeafBinding, &ActiveDockWindow)>();
    let (area_ent, leaf_binding, active) = area_q.single(app.world()).unwrap();
    assert_eq!(leaf_binding.0, NodeId(1));
    let initial_active = active.0.unwrap();

    // Changing ONLY active tab preserves live DockArea entity ID and switches visibility cleanly.
    app.world_mut()
        .resource_mut::<DockTree>()
        .set_active(NodeId(1), 1);
    app.update();
    app.update();

    let (area_ent_after, _, active_after) = area_q.single(app.world()).unwrap();
    assert_eq!(area_ent_after, area_ent, "active switch must not recreate area");
    assert_ne!(active_after.0.unwrap(), initial_active);

    // Verify visibility toggled properly (`UDisplay::None` on old, `UDisplay::Flex` on new).
    assert_eq!(
        app.world().get::<ULayout>(initial_active).unwrap().display,
        UDisplay::None
    );
    assert_eq!(
        app.world().get::<ULayout>(active_after.0.unwrap()).unwrap().display,
        UDisplay::Flex
    );

    // Hot-reload window descriptor `"a"` via register_or_replace rebuilds `"a"` cleanly.
    let bc2 = build_count.clone();
    app.world_mut()
        .resource_mut::<WindowRegistry>()
        .register_or_replace(DockWindowDescriptor::new("a", "Window A", move |cmds, parent| {
            bc2.fetch_add(10, Ordering::SeqCst);
            cmds.spawn((ChildOf(parent), UNode::default()));
        }));
    app.update();
    assert_eq!(build_count.load(Ordering::SeqCst), 11);

    // Click close button on tab 0 -> removes tab `"a"` without selecting it first.
    let close_btns: Vec<(Entity, DockTabCloseButton)> = app
        .world_mut()
        .query::<(Entity, &DockTabCloseButton)>()
        .iter(app.world())
        .map(|(e, b)| (e, *b))
        .collect();
    assert_eq!(close_btns.len(), 2);
    let close_first = close_btns
        .iter()
        .find(|(_, b)| b.window_entity == initial_active)
        .unwrap()
        .0;

    trigger_click(&mut app, close_first, PointerButton::Primary);
    app.update();
    app.update();

    assert_eq!(
        app.world().resource::<DockTree>().find_leaf(NodeId(1)).unwrap().tabs.len(),
        1
    );

    // Middle-click remaining DockTab -> removes final tab (and keeps persistent root valid).
    let remaining_tab = app
        .world_mut()
        .query_filtered::<Entity, With<DockTab>>()
        .single(app.world())
        .unwrap();

    trigger_click(&mut app, remaining_tab, PointerButton::Middle);
    app.update();
    app.update();

    let tree = app.world().resource::<DockTree>();
    let root_leaf = tree.find_leaf(tree.root().id()).unwrap();
    assert!(root_leaf.tabs.is_empty());
    assert!(root_leaf.persistent);
}

#[test]
fn icon_sidebar_and_add_window_popup_workflow() {
    let mut app = test_panel_app();
    {
        let mut reg = app.world_mut().resource_mut::<WindowRegistry>();
        reg.register(DockWindowDescriptor::new("h", "Hierarchy", |_, _| {}).with_priority(0));
        reg.register(DockWindowDescriptor::new("p", "Properties", |_, _| {}).with_priority(1));
    }

    let left = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::IconSidebar,
        vec![
            DockTabEntry::new(TabId(10), "h", "Hierarchy"),
            DockTabEntry::new(TabId(11), "p", "Properties"),
        ],
    );
    let right = DockLeaf::new(
        NodeId(3),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(12), "h", "Hierarchy")],
    );
    *app.world_mut().resource_mut::<DockTree>() = DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(4),
        axis: SplitAxis::Horizontal,
        fraction: 0.3,
        first: Box::new(DockNode::Leaf(left)),
        second: Box::new(DockNode::Leaf(right)),
    }));

    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));
    app.update();
    app.update();

    // Two sidebar icons should exist; clicking `"p"` switches active tab of NodeId(2).
    let sb_icons: Vec<(Entity, DockSidebarIcon)> = app
        .world_mut()
        .query::<(Entity, &DockSidebarIcon)>()
        .iter(app.world())
        .map(|(e, s)| (e, *s))
        .collect();
    assert_eq!(sb_icons.len(), 2);

    let props_icon = sb_icons
        .iter()
        .find(|(_, s)| app.world().get::<DockWindow>(s.window_entity).unwrap().window_id == "p")
        .unwrap()
        .0;

    trigger_click(&mut app, props_icon, PointerButton::Primary);
    app.update();
    assert_eq!(
        app.world().resource::<DockTree>().find_leaf(NodeId(2)).unwrap().active,
        1
    );

    // Click `+` button on right tab bar -> opens AddWindowPopup.
    let add_btn = app
        .world_mut()
        .query_filtered::<Entity, With<DockTabAddButton>>()
        .single(app.world())
        .unwrap();
    trigger_click(&mut app, add_btn, PointerButton::Primary);
    app.update();

    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<AddWindowPopup>>()
            .iter(app.world())
            .count(),
        1
    );

    // Pressing Escape dismisses popup cleanly.
    press_escape(&mut app);
    app.update();
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<AddWindowPopup>>()
            .iter(app.world())
            .count(),
        0
    );

    // Re-open popup and verify item click adds a new tab to NodeId(3).
    trigger_click(&mut app, add_btn, PointerButton::Primary);
    app.update();

    let popup_items: Vec<Entity> = app
        .world_mut()
        .query::<Entity>()
        .iter(app.world())
        .filter(|&e| app.world().get::<AddWindowPopup>(e).is_some())
        .collect();
    assert_eq!(popup_items.len(), 1);

    // Find clickable popup items by walking children of the popup overlay card.
    // Any entity with UInteraction + UInteractionColors spawned by popup is an item.
    let item_entity = app
        .world_mut()
        .query_filtered::<Entity, (With<UInteraction>, With<UInteractionColors>, With<ChildOf>)>()
        .iter(app.world())
        .find(|&e| {
            let mut cur = e;
            while let Some(c) = app.world().get::<ChildOf>(cur) {
                cur = c.parent();
                if app.world().get::<AddWindowPopup>(cur).is_some() {
                    return true;
                }
            }
            false
        })
        .unwrap();

    trigger_click(&mut app, item_entity, PointerButton::Primary);
    app.update();
    app.update();

    assert_eq!(
        app.world().resource::<DockTree>().find_leaf(NodeId(3)).unwrap().tabs.len(),
        2
    );
}

#[test]
fn workspace_strip_switch_add_close_and_rename() {
    let mut app = test_panel_app();

    let tree_a = DockTree::new(
        DockLeaf::new(
            NodeId(1),
            DockAreaStyle::TabBar,
            vec![DockTabEntry::new(TabId(2), "scene", "Scene")],
        )
        .with_persistent(true),
    );
    let tree_b = DockTree::new(
        DockLeaf::new(
            NodeId(1),
            DockAreaStyle::TabBar,
            vec![DockTabEntry::new(TabId(5), "inspector", "Inspector")],
        )
        .with_persistent(true),
    );

    app.world_mut()
        .resource_scope(|world, mut ws: Mut<WorkspaceRegistry>| {
            let t_a = tree_a.clone();
            ws.register(WorkspaceDescriptor::new("a", "Alpha", move || t_a.clone()));
            let t_b = tree_b.clone();
            ws.register(WorkspaceDescriptor::new("b", "Beta", move || t_b.clone()));
            let mut live = world.resource_mut::<DockTree>();
            ws.load_active_into(&mut live);
        });

    app.world_mut().spawn(workspace_tab_strip());
    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));
    app.update();
    app.update();

    // Mutate tree A by adding a tab; verify switching to B and back to A preserves the edit!
    app.world_mut()
        .resource_mut::<DockTree>()
        .add_tab(NodeId(1), "console", "Console");
    app.update();

    let beta_tab = app
        .world_mut()
        .query::<(Entity, &WorkspaceTab)>()
        .iter(app.world())
        .find(|(_, w)| w.workspace_id == "b")
        .unwrap()
        .0;

    trigger_click(&mut app, beta_tab, PointerButton::Primary);
    app.update();
    app.update();

    assert_eq!(app.world().resource::<WorkspaceRegistry>().active_id(), Some("b"));
    assert!(app.world().resource::<DockTree>().has_window_kind("inspector"));
    assert!(!app.world().resource::<DockTree>().has_window_kind("console"));

    let alpha_tab = app
        .world_mut()
        .query::<(Entity, &WorkspaceTab)>()
        .iter(app.world())
        .find(|(_, w)| w.workspace_id == "a")
        .unwrap()
        .0;
    trigger_click(&mut app, alpha_tab, PointerButton::Primary);
    app.update();
    assert!(app.world().resource::<DockTree>().has_window_kind("console"));

    // Click `+` workspace button -> duplicates Alpha into a 3rd workspace and switches to it.
    let add_ws = app
        .world_mut()
        .query_filtered::<Entity, With<WorkspaceTabAddButton>>()
        .single(app.world())
        .unwrap();
    trigger_click(&mut app, add_ws, PointerButton::Primary);
    app.update();
    app.update();
    assert_eq!(app.world().resource::<WorkspaceRegistry>().workspaces().len(), 3);

    // Close Beta workspace via `X` -> 2 workspaces left.
    let close_beta = app
        .world_mut()
        .query::<(Entity, &WorkspaceTabCloseButton)>()
        .iter(app.world())
        .find(|(_, c)| c.workspace_id == "b")
        .unwrap()
        .0;
    trigger_click(&mut app, close_beta, PointerButton::Primary);
    app.update();
    assert_eq!(app.world().resource::<WorkspaceRegistry>().workspaces().len(), 2);

    // Double-click Alpha to enter inline rename mode & submit new name.
    let alpha_tab = app
        .world_mut()
        .query::<(Entity, &WorkspaceTab)>()
        .iter(app.world())
        .find(|(_, w)| w.workspace_id == "a")
        .unwrap()
        .0;
    for _ in 0..2 {
        trigger_click(&mut app, alpha_tab, PointerButton::Primary);
        app.update();
    }

    let rename_ent = app
        .world_mut()
        .query_filtered::<Entity, With<WorkspaceRenameInput>>()
        .single(app.world())
        .unwrap();
    app.world_mut().trigger(UTextInputSubmit {
        entity: rename_ent,
        value: "Main Editing".to_string(),
    });
    app.update();

    let renamed = app
        .world()
        .resource::<WorkspaceRegistry>()
        .workspaces()
        .iter()
        .find(|w| w.id == "a")
        .unwrap()
        .name
        .as_str();
    assert_eq!(renamed, "Main Editing");
}

#[test]
fn dock_drag_drop_tab_row_split_and_escape_cancel() {
    let mut app = test_panel_app();

    {
        let mut reg = app.world_mut().resource_mut::<WindowRegistry>();
        reg.register(DockWindowDescriptor::new("w1", "One", |_, _| {}));
        reg.register(DockWindowDescriptor::new("w2", "Two", |_, _| {}));
        reg.register(DockWindowDescriptor::new("w3", "Three", |_, _| {}));
    }

    let left = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(10), "w1", "One"),
            DockTabEntry::new(TabId(11), "w2", "Two"),
        ],
    );
    let right = DockLeaf::new(
        NodeId(3),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(12), "w3", "Three")],
    );
    *app.world_mut().resource_mut::<DockTree>() = DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(4),
        axis: SplitAxis::Horizontal,
        fraction: 0.5,
        first: Box::new(DockNode::Leaf(left)),
        second: Box::new(DockNode::Leaf(right)),
    }));

    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));
    app.update();
    app.update();

    // Assign explicit screen-space transforms and computed sizes to both areas and right tab bar
    // (window default fallback is 1280x720, world centered at (0,0), +Y up).
    let areas: Vec<(Entity, NodeId)> = app
        .world_mut()
        .query::<(Entity, &LeafBinding)>()
        .iter(app.world())
        .map(|(e, b)| (e, b.0))
        .collect();
    let left_area = areas.iter().find(|(_, id)| *id == NodeId(2)).unwrap().0;
    let right_area = areas.iter().find(|(_, id)| *id == NodeId(3)).unwrap().0;

    app.world_mut().entity_mut(left_area).insert((
        GlobalTransform::from_translation(Vec3::new(-320.0, 0.0, 0.0)),
        ComputedSize {
            width: 640.0,
            height: 720.0,
            ..default()
        },
    ));
    app.world_mut().entity_mut(right_area).insert((
        GlobalTransform::from_translation(Vec3::new(320.0, 0.0, 0.0)),
        ComputedSize {
            width: 640.0,
            height: 720.0,
            ..default()
        },
    ));

    // Also give right DockTabBar and its DockTab real geometry so we test tab-row insertion index!
    let right_bar = app
        .world()
        .get::<Children>(right_area)
        .unwrap()
        .iter()
        .find(|&c| app.world().get::<DockTabBar>(c).is_some())
        .unwrap();
    app.world_mut().entity_mut(right_bar).insert((
        GlobalTransform::from_translation(Vec3::new(320.0, 344.0, 0.0)),
        ComputedSize {
            width: 640.0,
            height: 32.0,
            ..default()
        },
    ));

    let right_tab_three = app
        .world_mut()
        .query::<(Entity, &DockTab)>()
        .iter(app.world())
        .find(|(_, t)| t.area_entity == right_area)
        .unwrap()
        .0;
    // Screen x=[650..750], y=[0..32] => center (700, 16) => world (+60, +344)
    app.world_mut().entity_mut(right_tab_three).insert((
        GlobalTransform::from_translation(Vec3::new(60.0, 344.0, 0.0)),
        ComputedSize {
            width: 100.0,
            height: 32.0,
            ..default()
        },
    ));

    let tab_one = app
        .world_mut()
        .query::<(Entity, &DockTab)>()
        .iter(app.world())
        .find(|(_, t)| t.area_entity == left_area && t.tab_index == 0)
        .unwrap()
        .0;

    app.world_mut().trigger(PointerDragStart {
        entity: tab_one,
        pointer: test_pointer(Vec2::new(80.0, 16.0)),
        button: PointerButton::Primary,
        hit: dummy_hit(tab_one),
    });
    app.update();
    assert!(app.world().resource::<DockDragState>().is_dragging());

    // Hover over LEFT side of `Three` (screen x = 665 < center 700, y = 16 inside right_bar) -> insert_index 0!
    app.world_mut().trigger(PointerDrag {
        entity: tab_one,
        pointer: test_pointer(Vec2::new(665.0, 16.0)),
        button: PointerButton::Primary,
        distance: Vec2::new(585.0, 0.0),
        delta: Vec2::new(585.0, 0.0),
    });
    app.update();
    assert_eq!(
        app.world().resource::<DockDragState>().active.as_ref().unwrap().drop_target,
        Some(DropTarget::TabBar {
            area_entity: right_area,
            insert_index: 0,
        })
    );

    // Drag to bottom edge of right area (screen (960, 680) => ny = 680/720 > 0.75 => Bottom)
    app.world_mut().trigger(PointerDrag {
        entity: tab_one,
        pointer: test_pointer(Vec2::new(960.0, 680.0)),
        button: PointerButton::Primary,
        distance: Vec2::new(880.0, 664.0),
        delta: Vec2::new(295.0, 664.0),
    });
    app.update();
    assert_eq!(
        app.world().resource::<DockDragState>().active.as_ref().unwrap().drop_target,
        Some(DropTarget::Area {
            area_entity: right_area,
            edge: DropEdge::Bottom,
        })
    );

    // End drag -> creates a 3rd leaf in DockTree!
    app.world_mut().trigger(PointerDragEnd {
        entity: tab_one,
        pointer: test_pointer(Vec2::new(960.0, 680.0)),
        button: PointerButton::Primary,
        distance: Vec2::new(880.0, 664.0),
    });
    app.update();
    app.update();

    assert!(!app.world().resource::<DockDragState>().is_dragging());
    assert_eq!(app.world().resource::<DockTree>().leaf_ids().len(), 3);

    // Start another drag and cancel with Escape -> tree stays at 3 leaves.
    let any_tab = app
        .world_mut()
        .query_filtered::<Entity, With<DockTab>>()
        .iter(app.world())
        .next()
        .unwrap();
    app.world_mut().trigger(PointerDragStart {
        entity: any_tab,
        pointer: test_pointer(Vec2::new(50.0, 16.0)),
        button: PointerButton::Primary,
        hit: dummy_hit(any_tab),
    });
    app.update();
    assert!(app.world().resource::<DockDragState>().is_dragging());

    press_escape(&mut app);
    app.update();
    assert!(!app.world().resource::<DockDragState>().is_dragging());
    assert_eq!(app.world().resource::<DockTree>().leaf_ids().len(), 3);
}

#[test]
fn idle_frames_produce_zero_panel_mutations_or_rebuilds() {
    let mut app = test_panel_app();

    {
        let mut reg = app.world_mut().resource_mut::<WindowRegistry>();
        reg.register(DockWindowDescriptor::new("scene", "Scene", |cmds, parent| {
            cmds.spawn((ChildOf(parent), UNode::default()));
        }));
        reg.register(DockWindowDescriptor::new(
            "inspector",
            "Inspector",
            |cmds, parent| {
                cmds.spawn((ChildOf(parent), UNode::default()));
            },
        ));
        reg.register(DockWindowDescriptor::new("tools", "Tools", |cmds, parent| {
            cmds.spawn((ChildOf(parent), UNode::default()));
        }));
    }

    let left_leaf = DockLeaf::new(
        NodeId(1),
        DockAreaStyle::IconSidebar,
        vec![
            DockTabEntry::new(TabId(10), "tools", "Tools"),
            DockTabEntry::new(TabId(11), "scene", "Scene"),
        ],
    );
    let right_leaf = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(12), "inspector", "Inspector")],
    );
    let split_tree = DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(3),
        axis: SplitAxis::Horizontal,
        fraction: 0.3,
        first: Box::new(DockNode::Leaf(left_leaf)),
        second: Box::new(DockNode::Leaf(right_leaf)),
    }));

    app.world_mut()
        .resource_scope(|world, mut ws: Mut<WorkspaceRegistry>| {
            let t = split_tree.clone();
            ws.register(WorkspaceDescriptor::new("a", "Alpha", move || t.clone()));
            let t2 = split_tree.clone();
            ws.register(WorkspaceDescriptor::new("b", "Beta", move || t2.clone()));
            let mut live = world.resource_mut::<DockTree>();
            ws.load_active_into(&mut live);
        });

    app.world_mut().spawn(workspace_tab_strip());
    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));

    // Settle initial spawn + reconciliation + split normalization + chrome creation.
    for _ in 0..4 {
        app.update();
    }

    let initial_tabs: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<DockTab>>()
        .iter(app.world())
        .collect();
    let initial_icons: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<DockSidebarIcon>>()
        .iter(app.world())
        .collect();
    let initial_ws_tabs: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<WorkspaceTab>>()
        .iter(app.world())
        .collect();
    assert!(!initial_tabs.is_empty());
    assert!(!initial_icons.is_empty());
    assert_eq!(initial_ws_tabs.len(), 2);

    // Baseline change-detection queries.
    let mut panel_q = app.world_mut().query::<(Entity, Ref<Panel>)>();
    let mut icolors_q = app.world_mut().query::<(Entity, Ref<UInteractionColors>)>();
    let mut vis_q = app.world_mut().query::<(Entity, Ref<Visibility>)>();
    let _ = panel_q.iter(app.world()).count();
    let _ = icolors_q.iter(app.world()).count();
    let _ = vis_q.iter(app.world()).count();

    // Run multiple idle frames with zero user input or resource edits.
    for _ in 0..5 {
        app.update();
    }

    let changed_panels = panel_q
        .iter(app.world())
        .filter(|(_, p)| p.is_changed())
        .count();
    let changed_icolors = icolors_q
        .iter(app.world())
        .filter(|(_, c)| c.is_changed())
        .count();
    let changed_vis = vis_q
        .iter(app.world())
        .filter(|(_, v)| v.is_changed())
        .count();

    assert_eq!(changed_panels, 0, "Panel should not mutate on idle frames");
    assert_eq!(
        changed_icolors, 0,
        "UInteractionColors should not mutate on idle frames"
    );
    assert_eq!(
        changed_vis, 0,
        "Visibility should not mutate on idle frames"
    );

    let after_tabs: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<DockTab>>()
        .iter(app.world())
        .collect();
    let after_icons: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<DockSidebarIcon>>()
        .iter(app.world())
        .collect();
    let after_ws_tabs: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<WorkspaceTab>>()
        .iter(app.world())
        .collect();

    assert_eq!(after_tabs, initial_tabs);
    assert_eq!(after_icons, initial_icons);
    assert_eq!(after_ws_tabs, initial_ws_tabs);
}

#[test]
fn single_frame_panel_add_remove_and_split_have_no_blank_frame() {
    let mut app = test_panel_app();

    {
        let mut reg = app.world_mut().resource_mut::<WindowRegistry>();
        for (id, name) in [
            ("scene", "Scene"),
            ("inspector", "Inspector"),
            ("tools", "Tools"),
        ] {
            reg.register(DockWindowDescriptor::new(id, name, |cmds, parent| {
                cmds.spawn((ChildOf(parent), UNode::default()));
            }));
        }
    }

    let left_leaf = DockLeaf::new(
        NodeId(1),
        DockAreaStyle::IconSidebar,
        vec![DockTabEntry::new(TabId(10), "tools", "Tools")],
    );
    let right_leaf = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(11), "inspector", "Inspector")],
    );
    let initial_tree = DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(3),
        axis: SplitAxis::Horizontal,
        fraction: 0.3,
        first: Box::new(DockNode::Leaf(left_leaf)),
        second: Box::new(DockNode::Leaf(right_leaf)),
    }));
    *app.world_mut().resource_mut::<DockTree>() = initial_tree;

    app.world_mut()
        .spawn((DockTreeHost, UNode::default(), ULayout::default()));

    // Settle initial scene.
    app.update();

    let find_leaf_entity = |world: &mut World, leaf_id: NodeId| -> Entity {
        let mut q = world.query::<(Entity, &LeafBinding)>();
        q.iter(world)
            .find_map(|(e, b)| (b.0 == leaf_id).then_some(e))
            .expect("leaf area entity should exist")
    };
    let find_group_entity = |world: &mut World, group_id: NodeId| -> Entity {
        let mut q = world.query::<(Entity, &NodeBinding, &PanelGroup)>();
        q.iter(world)
            .find_map(|(e, b, _)| (b.0 == group_id).then_some(e))
            .expect("split group entity should exist")
    };
    let area_tabs_count = |world: &mut World, area_ent: Entity| -> usize {
        let mut q = world.query::<(Entity, &DockTab)>();
        q.iter(world)
            .filter(|(_, tab)| tab.area_entity == area_ent)
            .count()
    };

    let world = app.world_mut();
    let initial_group = find_group_entity(world, NodeId(3));
    let initial_left_area = find_leaf_entity(world, NodeId(1));
    let initial_right_area = find_leaf_entity(world, NodeId(2));
    let initial_inspector_win = world
        .get::<ActiveDockWindow>(initial_right_area)
        .and_then(|a| a.0)
        .expect("right area should have active inspector window");
    let initial_tools_win = world
        .get::<ActiveDockWindow>(initial_left_area)
        .and_then(|a| a.0)
        .expect("left area should have active tools window");

    // 1. Add a tab to right_leaf -> verify complete Frame-1 update after ONLY 1 app.update().
    {
        let mut tree = app.world_mut().resource_mut::<DockTree>();
        tree.add_tab(NodeId(2), "scene", "Scene")
            .expect("should add scene tab");
    }
    app.update();

    let world = app.world_mut();
    assert_eq!(find_group_entity(world, NodeId(3)), initial_group);
    assert_eq!(find_leaf_entity(world, NodeId(1)), initial_left_area);
    assert_eq!(find_leaf_entity(world, NodeId(2)), initial_right_area);
    assert_eq!(
        area_tabs_count(world, initial_right_area),
        2,
        "added tab pill must appear in the same frame"
    );
    let scene_win = world
        .get::<ActiveDockWindow>(initial_right_area)
        .and_then(|a| a.0)
        .expect("newly added scene tab must immediately be active");
    assert_ne!(scene_win, initial_inspector_win);
    assert_eq!(
        world.get::<ULayout>(scene_win).unwrap().display,
        UDisplay::Flex
    );
    assert_eq!(
        *world.get::<Visibility>(scene_win).unwrap(),
        Visibility::Inherited
    );
    assert_eq!(
        world.get::<ULayout>(initial_inspector_win).unwrap().display,
        UDisplay::None
    );
    assert_eq!(
        *world.get::<Visibility>(initial_inspector_win).unwrap(),
        Visibility::Hidden
    );
    assert_eq!(
        world
            .get::<ActiveDockWindow>(initial_left_area)
            .and_then(|a| a.0),
        Some(initial_tools_win)
    );

    // 2. Simulate bevy_luma hiding inactive window via scale=ZERO, then close the active tab.
    app.world_mut()
        .get_mut::<Transform>(initial_inspector_win)
        .unwrap()
        .scale = Vec3::ZERO;
    {
        let mut tree = app.world_mut().resource_mut::<DockTree>();
        tree.remove_tab(NodeId(2), 1)
            .expect("should remove added scene tab");
    }
    app.update();

    let world = app.world_mut();
    assert_eq!(find_group_entity(world, NodeId(3)), initial_group);
    assert_eq!(find_leaf_entity(world, NodeId(2)), initial_right_area);
    assert_eq!(
        area_tabs_count(world, initial_right_area),
        1,
        "closed tab pill must disappear in the same frame"
    );
    assert_eq!(
        world
            .get::<ActiveDockWindow>(initial_right_area)
            .and_then(|a| a.0),
        Some(initial_inspector_win)
    );
    assert_eq!(
        world.get::<ULayout>(initial_inspector_win).unwrap().display,
        UDisplay::Flex
    );
    assert_eq!(
        *world.get::<Visibility>(initial_inspector_win).unwrap(),
        Visibility::Inherited
    );
    assert_eq!(
        world.get::<Transform>(initial_inspector_win).unwrap().scale,
        Vec3::ONE,
        "restored window must unscale immediately on the same frame without a 1-frame flicker"
    );

    // 3. Split right_leaf vertically in a single frame via move_tab_to_new_split.
    {
        let mut tree = app.world_mut().resource_mut::<DockTree>();
        tree.add_tab(NodeId(2), "scene", "Scene")
            .expect("should add second tab before split");
    }
    app.update();

    let split_tab_win = app
        .world()
        .get::<ActiveDockWindow>(initial_right_area)
        .and_then(|a| a.0)
        .expect("scene tab should be active before moving to new split");

    let (new_inner_split_id, new_split_leaf_id) = {
        let mut tree = app.world_mut().resource_mut::<DockTree>();
        let split_id = tree
            .move_tab_to_new_split(NodeId(2), 1, NodeId(2), Edge::Bottom, 0.5)
            .expect("should move scene tab into a new bottom split");
        let new_leaf_id = tree
            .leaf_ids()
            .into_iter()
            .find(|&id| id != NodeId(1) && id != NodeId(2))
            .expect("new split leaf id should exist");
        (split_id, new_leaf_id)
    };
    app.update();

    let world = app.world_mut();
    assert_eq!(find_group_entity(world, NodeId(3)), initial_group);
    let _ = find_group_entity(world, new_inner_split_id);
    assert_eq!(find_leaf_entity(world, NodeId(1)), initial_left_area);
    assert_eq!(find_leaf_entity(world, NodeId(2)), initial_right_area);
    let new_split_area = find_leaf_entity(world, new_split_leaf_id);
    assert_eq!(
        world
            .get::<ActiveDockWindow>(new_split_area)
            .and_then(|a| a.0),
        Some(split_tab_win),
        "moved DockWindow entity must survive split creation without respawning"
    );
    assert_eq!(area_tabs_count(world, initial_right_area), 1);
    assert_eq!(area_tabs_count(world, new_split_area), 1);
    assert_eq!(
        world.get::<ULayout>(initial_inspector_win).unwrap().display,
        UDisplay::Flex
    );
    assert_eq!(
        world.get::<ULayout>(split_tab_win).unwrap().display,
        UDisplay::Flex
    );

    // Verify new vertical split's Panel & PanelHandle were already normalized in the same frame.
    let inner_panel_ent = world
        .get::<ChildOf>(new_split_area)
        .expect("new area must sit inside a normalized Panel")
        .parent();
    assert_eq!(
        world.get::<UNode>(inner_panel_ent).unwrap().height,
        UVal::Px(0.0),
        "Column Panel height must already be normalized to UVal::Px(0.0) on frame 1"
    );
    let inner_group_ent = world.get::<ChildOf>(inner_panel_ent).unwrap().parent();
    let inner_children = world.get::<Children>(inner_group_ent).unwrap();
    let handle_ent = inner_children
        .iter()
        .find(|&c| world.get::<PanelHandle>(c).is_some())
        .expect("inner split group should contain a handle");
    assert_eq!(
        world.get::<UNode>(handle_ent).unwrap().width,
        UVal::Percent(1.0),
        "Column PanelHandle width must already be normalized to Percent(1.0) on frame 1"
    );

    // 4. Close the sole tab of the new split -> split simplifies away while existing areas survive.
    {
        let mut tree = app.world_mut().resource_mut::<DockTree>();
        tree.remove_tab(new_split_leaf_id, 0)
            .expect("should close split tab");
    }
    app.update();

    let world = app.world_mut();
    assert_eq!(find_group_entity(world, NodeId(3)), initial_group);
    assert_eq!(find_leaf_entity(world, NodeId(1)), initial_left_area);
    assert_eq!(find_leaf_entity(world, NodeId(2)), initial_right_area);
    assert!(world.get_entity(new_split_area).is_err());
    assert_eq!(
        world.get::<ULayout>(initial_inspector_win).unwrap().display,
        UDisplay::Flex
    );
    assert_eq!(
        *world.get::<Visibility>(initial_inspector_win).unwrap(),
        Visibility::Inherited
    );
}
