//! Comprehensive interactive showcase of `bevy_luma_widgets::panel`:
//! Split panels, tabbed dock areas, icon sidebars, `+` add-window popup,
//! drag-and-drop tab reordering / docking / edge splitting, and multi-workspace strips.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LumaWidgetsPlugin))
        .add_systems(Startup, setup_dock_showcase)
        .run();
}

fn register_demo_windows(registry: &mut WindowRegistry, font: Handle<Font>) {
    let vp_font = font.clone();
    registry.register(
        DockWindowDescriptor::new("viewport", "Viewport", move |commands, container| {
            commands.spawn((
                ChildOf(container),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgb_u8(0x12, 0x13, 0x16),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                    ..default()
                },
                children![(
                    UNode::default(),
                    UText {
                        text: "Interactive 3D / Canvas Viewport\nDrag tabs by their pill, grip icon, or sidebar icon!"
                            .to_string(),
                        font_size: 14.0,
                        font: vp_font.clone(),
                        color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
                        ..default()
                    },
                )],
            ));
        })
        .with_icon(Icon::BOX)
        .with_default_area("main")
        .with_priority(0),
    );

    let shader_font = font.clone();
    registry.register(
        DockWindowDescriptor::from_scene("shader", "Shader Graph", move || {
            let f = shader_font.clone();
            bsn! {
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgb_u8(0x13, 0x15, 0x1A),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 10.0,
                }
                Children [
                    UText {
                        text: { "Node Shader Editor (built with bsn! + DockWindowDescriptor::from_scene)".to_string() },
                        font_size: 13.5,
                        font: f,
                        color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
                    }
                ]
            }
        })
        .with_icon(Icon::CODESANDBOX)
        .with_default_area("main")
        .with_priority(1),
    );

    let h_font = font.clone();
    registry.register(
        DockWindowDescriptor::new("hierarchy", "Hierarchy", move |commands, container| {
            spawn_label_list(
                commands,
                container,
                "Scene Entities",
                &["MainCamera", "DirectionalLight", "PlayerRig", "Environment", "PostProcess"],
                h_font.clone(),
            );
        })
        .with_icon(Icon::PANELS_TOP_LEFT)
        .with_default_area("left")
        .with_priority(2),
    );

    let a_font = font.clone();
    registry.register(
        DockWindowDescriptor::new("assets", "Assets", move |commands, container| {
            spawn_label_list(
                commands,
                container,
                "Project Folders",
                &["materials/", "meshes/", "shaders/", "textures/", "prefabs/"],
                a_font.clone(),
            );
        })
        .with_icon(Icon::FOLDER)
        .with_default_area("left")
        .with_priority(3),
    );

    let i_font = font.clone();
    registry.register(
        DockWindowDescriptor::new("inspector", "Inspector", move |commands, container| {
            spawn_label_list(
                commands,
                container,
                "Transform & Material",
                &["Position: (0.0, 1.5, -4.0)", "Rotation: (0°, 25°, 0°)", "Roughness: 0.42", "Metallic: 0.15"],
                i_font.clone(),
            );
        })
        .with_icon(Icon::SETTINGS)
        .with_default_area("right")
        .with_priority(4),
    );

    let c_font = font.clone();
    registry.register(
        DockWindowDescriptor::new("console", "Console", move |commands, container| {
            spawn_label_list(
                commands,
                container,
                "Output Log",
                &["[INFO] Renderer initialized", "[INFO] DockTree reconciled cleanly", "[OK] 60 FPS"],
                c_font.clone(),
            );
        })
        .with_icon(Icon::INBOX)
        .with_default_area("bottom")
        .with_priority(5),
    );

    let p_font = font;
    registry.register(
        DockWindowDescriptor::new("profiler", "Profiler", move |commands, container| {
            spawn_label_list(
                commands,
                container,
                "Frame Budget",
                &["UI Layout: 0.24 ms", "SDF Pass: 0.41 ms", "Render Queue: 1.08 ms"],
                p_font.clone(),
            );
        })
        .with_icon(Icon::LAYOUT_GRID)
        .with_default_area("bottom")
        .with_priority(6),
    );
}

fn spawn_label_list(
    commands: &mut Commands,
    container: Entity,
    heading: &str,
    rows: &[&str],
    font: Handle<Font>,
) {
    let list = commands
        .spawn((
            ChildOf(container),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Stretch,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(list),
        UNode::default(),
        UText {
            text: heading.to_string(),
            font_size: 13.5,
            font: font.clone(),
            color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            ..default()
        },
    ));

    for row in rows {
        commands.spawn((
            ChildOf(list),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(8.0, 5.0),
                border_radius: UCornerRadius::all(4.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                ..default()
            },
            UText {
                text: (*row).to_string(),
                font_size: 12.5,
                font: font.clone(),
                color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
                ..default()
            },
        ));
    }
}

fn build_studio_tree() -> DockTree {
    let left_sidebar = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::IconSidebar,
        vec![
            DockTabEntry::new(TabId(10), "hierarchy", "Hierarchy"),
            DockTabEntry::new(TabId(11), "assets", "Assets"),
        ],
    )
    .with_name("left");

    let center_main = DockLeaf::new(
        NodeId(3),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(12), "viewport", "Viewport"),
            DockTabEntry::new(TabId(13), "shader", "Shader Graph"),
        ],
    )
    .with_name("main")
    .with_persistent(true);

    let bottom_console = DockLeaf::new(
        NodeId(4),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(14), "console", "Console"),
            DockTabEntry::new(TabId(15), "profiler", "Profiler"),
        ],
    )
    .with_name("bottom");

    let center_vertical = DockNode::Split(DockSplit {
        id: NodeId(5),
        axis: SplitAxis::Vertical,
        fraction: 0.70,
        first: Box::new(DockNode::Leaf(center_main)),
        second: Box::new(DockNode::Leaf(bottom_console)),
    });

    let left_and_center = DockNode::Split(DockSplit {
        id: NodeId(6),
        axis: SplitAxis::Horizontal,
        fraction: 0.22,
        first: Box::new(DockNode::Leaf(left_sidebar)),
        second: Box::new(center_vertical),
    });

    let right_inspector = DockLeaf::new(
        NodeId(7),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(16), "inspector", "Inspector")],
    )
    .with_name("right");

    DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(8),
        axis: SplitAxis::Horizontal,
        fraction: 0.78,
        first: Box::new(left_and_center),
        second: Box::new(DockNode::Leaf(right_inspector)),
    }))
}

fn build_compact_tree() -> DockTree {
    let left = DockLeaf::new(
        NodeId(2),
        DockAreaStyle::TabBar,
        vec![
            DockTabEntry::new(TabId(20), "viewport", "Viewport"),
            DockTabEntry::new(TabId(21), "console", "Console"),
        ],
    )
    .with_name("main")
    .with_persistent(true);

    let right = DockLeaf::new(
        NodeId(3),
        DockAreaStyle::TabBar,
        vec![DockTabEntry::new(TabId(22), "inspector", "Inspector")],
    )
    .with_name("right");

    DockTree::from_root(DockNode::Split(DockSplit {
        id: NodeId(4),
        axis: SplitAxis::Horizontal,
        fraction: 0.68,
        first: Box::new(DockNode::Leaf(left)),
        second: Box::new(DockNode::Leaf(right)),
    }))
}

fn setup_dock_showcase(
    mut commands: Commands,
    mut registry: ResMut<WindowRegistry>,
    mut workspaces: ResMut<WorkspaceRegistry>,
    mut dock_tree: ResMut<DockTree>,
    theme: Res<Theme>,
    style: Res<DockStyle>,
) {
    commands.spawn(Camera2d);

    register_demo_windows(&mut registry, theme.text.font.inter_regular.clone());

    workspaces.register(
        WorkspaceDescriptor::new("studio", "Studio", build_studio_tree)
            .with_icon(Icon::LAYOUT_GRID),
    );
    workspaces.register(
        WorkspaceDescriptor::new("compact", "Compact", build_compact_tree)
            .with_icon(Icon::PANELS_LEFT_BOTTOM),
    );
    workspaces.load_active_into(&mut dock_tree);

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: style.canvas_bg,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Stretch,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(root), workspace_tab_strip()));

    commands.spawn((
        ChildOf(root),
        DockTreeHost,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(0.0),
            padding: USides::all(4.0),
            background_color: style.canvas_bg,
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
    ));
}
