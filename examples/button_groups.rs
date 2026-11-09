//! Joined toolbar button groups — dark neutral surfaces, hairline dividers,
//! emerald accent and muted / danger tones.
//!
//! Run with: `cargo run --release --example button_groups`

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma_widgets::navigation::SceneList;
use bevy_luma_widgets::prelude::*;

const APP_BG: Color = Color::srgb(0.039, 0.039, 0.043);
const STAR_ON: Color = Color::srgb(0.98, 0.78, 0.12);
const STAR_OFF: Color = Color::srgb(0.30, 0.30, 0.33);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Button Groups".into(),
                resolution: (1160, 920).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(APP_BG))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<ButtonGroupClicked>| {
            let e = trigger.event();
            info!(
                "ButtonGroupClicked: index={} value={:?} selected={}",
                e.index, e.value, e.selected
            );
        })
        .run();
}

/// A horizontally centered row of groups.
struct Row(Vec<ButtonGroup>);

impl Scene for Row {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let groups = SceneList::new(self.0);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 14.0,
            }
            Children [
                {groups}
            ]
        };
        s.resolve(context, scene)
    }
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    let g = |segments: Vec<Segment>| {
        ButtonGroup::new(segments)
            .font(font.clone())
            .icon_font(icon_font.clone())
    };
    let star = |on: bool| {
        Segment::icon(Icon::STAR)
            .width(24.0)
            .icon_color(if on { STAR_ON } else { STAR_OFF })
    };

    let rows = vec![
        // 1
        Row(vec![
            g(vec![
                Segment::icon(Icon::CHEVRONS_LEFT),
                Segment::icon(Icon::CHEVRON_LEFT),
                Segment::addon("3 / 8"),
                Segment::icon(Icon::CHEVRON_RIGHT),
                Segment::icon(Icon::CHEVRONS_RIGHT),
            ]),
            g(vec![
                Segment::addon("https://"),
                Segment::input_value("example.com", 116.0),
            ]),
            g(vec![
                Segment::text("Years"),
                Segment::text("Months"),
                Segment::text("Days"),
            ]),
            g(vec![
                Segment::addon("Amount"),
                Segment::input("0.00", 124.0),
                Segment::select("usd")
                    .menu(vec!["USD ($)", "EUR (€)", "GBP (£)", "CAD ($)", "JPY (¥)"])
                    .recessed(),
            ]),
        ]),
        // 2
        Row(vec![
            g(vec![
                Segment::icon(Icon::MAP_PIN),
                Segment::input("Search location...", 336.0),
                Segment::text("Go").accent(),
            ]),
            g(vec![
                Segment::icon(Icon::LAYOUT_GRID),
                Segment::icon(Icon::COLUMNS_2),
                Segment::icon(Icon::SERVER),
            ]),
            g(vec![
                Segment::icon(Icon::MINUS),
                Segment::addon("1").width(40.0),
                Segment::icon(Icon::PLUS),
            ]),
            g(vec![
                Segment::icon_text(Icon::ARROW_LEFT, "Previous"),
                Segment::text_icon("Next", Icon::ARROW_RIGHT),
            ]),
        ]),
        // 3
        Row(vec![
            g(vec![
                Segment::icon_text(Icon::CALENDAR, "Range"),
                Segment::select("7d")
                    .menu(vec!["7d (Week)", "24h (Today)", "30d (Month)", "90d (Quarter)", "1y (Year)"])
                    .recessed(),
            ]),
            g(vec![star(true), star(true), star(true), star(false), star(false)])
                .recessed()
                .no_dividers(),
            g(vec![
                Segment::icon_text(Icon::SAVE, "Save"),
                Segment::dropdown()
                    .menu(vec!["Save & Publish", "Save as Draft", "Export Template", "Duplicate"]),
            ])
            .accent(),
            g(vec![
                Segment::text("Active"),
                Segment::addon("Disabled").muted(),
                Segment::text("Active"),
            ]),
            g(vec![Segment::text("Save"), Segment::text("Publish")]).accent(),
        ]),
        // 4
        Row(vec![
            g(vec![
                Segment::icon_text(Icon::BELL, "Subscribe"),
                Segment::dropdown()
                    .menu(vec!["All Notifications", "Mentions Only", "Custom Alerts", "Mute Notifications"]),
            ]),
            g(vec![
                Segment::icon_text(Icon::FUNNEL, "Filter"),
                Segment::icon(Icon::REFRESH_CW),
            ]),
            g(vec![
                Segment::icon_text(Icon::PLAY, "Run"),
                Segment::dropdown()
                    .menu(vec!["Run with Debugger", "Run Tests & Benchmarks", "Profile Performance", "Step by Step"]),
            ]),
            g(vec![
                Segment::icon_text(Icon::BOOKMARK, "Save"),
                Segment::addon("1337").recessed(),
            ]),
            g(vec![
                Segment::text("Copy"),
                Segment::text("Cut"),
                Segment::text("Paste"),
            ]),
        ]),
        // 5
        Row(vec![
            g(vec![
                Segment::input("Search projects...", 356.0),
                Segment::text("Search"),
            ]),
            g(vec![
                Segment::icon_text(Icon::CREDIT_CARD, "Pay $49").accent(),
                Segment::icon_text(Icon::CLOUD, "Invoice"),
            ]),
            g(vec![
                Segment::icon_text(Icon::DOWNLOAD, "Download"),
                Segment::dropdown()
                    .menu(vec!["Download as ZIP", "Download as TAR.GZ", "Clone via HTTPS", "Clone via SSH"]),
            ]),
        ]),
        // 6
        Row(vec![
            g(vec![Segment::icon(Icon::ARROW_LEFT), Segment::icon(Icon::ARROW_RIGHT)]),
            g(vec![
                Segment::text("Draft"),
                Segment::text("Preview"),
                Segment::text("Publish"),
            ]),
            g(vec![Segment::text("Delete"), Segment::text("Block")]).danger(),
            g(vec![
                Segment::icon_text(Icon::EYE, "Preview"),
                Segment::icon_text(Icon::CODE, "Code"),
                Segment::icon_text(Icon::PEN, "Edit"),
            ]),
            g(vec![
                Segment::icon_text(Icon::LAYOUT_GRID, "Grid").selected(true),
                Segment::icon_text(Icon::SERVER, "List"),
            ])
            .exclusive(),
        ]),
        // 7
        Row(vec![
            g(vec![
                Segment::icon_text(Icon::SUN, "Light").selected(true),
                Segment::icon_text(Icon::MOON, "Dark"),
            ])
            .exclusive(),
            g(vec![
                Segment::icon(Icon::COPY),
                Segment::icon(Icon::SHARE_2),
                Segment::icon(Icon::SAVE),
                Segment::icon(Icon::TRASH_2),
            ]),
            g(vec![
                Segment::text("Left"),
                Segment::text("Center"),
                Segment::text("Right"),
            ]),
            g(vec![
                Segment::icon_text(Icon::SERVER, "Region"),
                Segment::select("us")
                    .menu(vec!["US (East)", "US (West)", "EU (Frankfurt)", "AP (Tokyo)"])
                    .recessed(),
            ]),
        ]),
        // 8
        Row(vec![
            g(vec![
                Segment::input("Paste link...", 360.0),
                Segment::icon(Icon::LINK),
                Segment::icon(Icon::CHECK),
            ]),
            g(vec![
                Segment::icon(Icon::ALIGN_LEFT),
                Segment::icon(Icon::ALIGN_CENTER),
                Segment::icon(Icon::ALIGN_RIGHT),
            ]),
            g(vec![
                Segment::icon(Icon::SKIP_BACK),
                Segment::icon(Icon::PLAY),
                Segment::icon(Icon::SKIP_FORWARD),
                Segment::icon(Icon::REPEAT),
            ]),
            g(vec![
                Segment::icon(Icon::BOLD),
                Segment::icon(Icon::ITALIC),
                Segment::icon(Icon::UNDERLINE),
            ]),
            g(vec![Segment::icon(Icon::LIST), Segment::icon(Icon::INDENT_INCREASE)]),
        ]),
        // 9
        Row(vec![
            g(vec![
                Segment::icon(Icon::BOLD),
                Segment::icon(Icon::ITALIC),
                Segment::icon(Icon::ZOOM_OUT),
                Segment::icon(Icon::ZOOM_IN),
            ]),
            g(vec![
                Segment::icon_text(Icon::THUMBS_UP, "24"),
                Segment::icon(Icon::THUMBS_DOWN),
            ]),
        ]),
        // 10 — Unattached gap (Bold / Italic / Underline not touching) + Inset track + Pill
        Row(vec![
            g(vec![
                Segment::icon(Icon::BOLD).value("bold").selected(true),
                Segment::icon(Icon::ITALIC).value("italic"),
                Segment::icon(Icon::UNDERLINE).value("underline").selected(true),
            ])
            .gap(5.0)
            .multi_select(),
            g(vec![
                Segment::text("Overview").selected(true),
                Segment::text("Analytics").badge("12"),
                Segment::text("Logs"),
            ])
            .recessed()
            .gap(4.0)
            .inset(3.0)
            .exclusive(),
            g(vec![
                Segment::icon_text(Icon::ZAP, "Fast Mode").selected(true),
                Segment::icon_text(Icon::TERMINAL, "Shell"),
                Segment::icon_text(Icon::CPU, "Compute"),
            ])
            .recessed()
            .pill()
            .segment_radius(999.0)
            .gap(4.0)
            .inset(3.0)
            .exclusive(),
        ]),
        // 11 — Unattached action buttons with gap + Compact & Large size presets
        Row(vec![
            g(vec![
                Segment::icon_text(Icon::GIT_BRANCH, "main"),
                Segment::icon_text(Icon::GIT_COMMIT_HORIZONTAL, "42").badge("new"),
                Segment::icon(Icon::REFRESH_CW),
            ])
            .gap(5.0),
            g(vec![
                Segment::icon(Icon::ALIGN_LEFT).selected(true),
                Segment::icon(Icon::ALIGN_CENTER),
                Segment::icon(Icon::ALIGN_RIGHT),
            ])
            .gap(4.0)
            .exclusive(),
            g(vec![
                Segment::text("1m"),
                Segment::text("5m").selected(true),
                Segment::text("15m"),
                Segment::text("1h"),
            ])
            .compact()
            .recessed()
            .gap(2.0)
            .inset(2.0)
            .exclusive(),
            g(vec![
                Segment::icon_text(Icon::SEND, "Deploy Production").accent(),
                Segment::dropdown()
                    .menu(vec!["Deploy Staging", "Canary Deployment (10%)", "Emergency Rollback", "View Deployment Logs"]),
            ])
            .large(),
        ]),
    ];

    let rows = SceneList::new(rows);

    commands.spawn_scene(bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: APP_BG,
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 22.0,
        }
        Children [
            {rows}
        ]
    });
}
