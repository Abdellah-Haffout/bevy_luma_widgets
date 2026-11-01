//! Advanced data visualization, feeds, charts, and workflow view widgets.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::navigation::SceneList;
use crate::prelude::*;
use crate::button::Button;

// ============================================================================
// 3.10 Avatar Group
// ============================================================================

/// Overlapping avatars row with trailing +N remaining indicator.
#[derive(Clone, Debug, Reflect)]
pub struct AvatarGroup {
    pub initials: Vec<String>,
    pub max_display: usize,
    pub font: Handle<Font>,
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self {
            initials: vec!["JD".into(), "AK".into(), "ML".into(), "TS".into(), "RB".into()],
            max_display: 3,
            font: Handle::default(),
        }
    }
}

impl AvatarGroup {
    pub fn new(initials: Vec<impl Into<String>>) -> Self {
        Self {
            initials: initials.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max_display = max;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for AvatarGroup {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total = self.initials.len();
        let display_count = total.min(self.max_display);
        let remainder = total.saturating_sub(display_count);
        let font = self.font;

        let mut items: Vec<String> = self.initials.into_iter().take(display_count).collect();
        if remainder > 0 {
            items.push(format!("+{remainder}"));
        }

        let avatar_nodes: Vec<_> = items.into_iter().enumerate().map(|(idx, init)| {
            let margin_left = if idx == 0 { 0.0_f32 } else { -8.0_f32 };
            let f = font.clone();
            bsn! {
                UNode {
                    margin: USides { top: 0.0, right: 0.0, bottom: 0.0, left: margin_left },
                }
                Children [
                    @{Avatar::new(init).size(AvatarSize::Small).font(f)}
                ]
            }
        }).collect();

        let avatar_nodes_scene = SceneList::new(avatar_nodes);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
            }
            Children [
                {avatar_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.12 Notification Counter
// ============================================================================

/// Circular numeric count badge with highlight styling.
#[derive(Clone, Debug, Reflect)]
pub struct NotificationCounter {
    pub count: usize,
    pub max: usize,
    pub font: Handle<Font>,
}

impl Default for NotificationCounter {
    fn default() -> Self {
        Self {
            count: 5,
            max: 99,
            font: Handle::default(),
        }
    }
}

impl NotificationCounter {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            ..default()
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for NotificationCounter {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let label = if self.count > self.max {
            format!("{}+", self.max)
        } else {
            self.count.to_string()
        };
        let font = self.font;

        let s = bsn! {
            UNode {
                min_height: 18.0,
                padding: USides::axes(6.0, 2.0),
                background_color: Color::srgb(0.94, 0.27, 0.27),
                border_radius: UCornerRadius::all(9.0),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                UText {
                    text: label,
                    font_size: 11.0,
                    color: Color::WHITE,
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.14 Data Table
// ============================================================================

/// Multi-column structured data table with header and alternate row striping.
#[derive(Clone, Debug, Reflect)]
pub struct DataTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub font: Handle<Font>,
}

impl Default for DataTable {
    fn default() -> Self {
        Self {
            headers: vec!["ID".into(), "Name".into(), "Status".into(), "Role".into()],
            rows: vec![
                vec!["#01".into(), "Alice Martin".into(), "Active".into(), "Admin".into()],
                vec!["#02".into(), "Bob Vance".into(), "Offline".into(), "Editor".into()],
                vec!["#03".into(), "Carol Danvers".into(), "Active".into(), "Viewer".into()],
            ],
            font: Handle::default(),
        }
    }
}

impl DataTable {
    pub fn new(headers: Vec<impl Into<String>>, rows: Vec<Vec<impl Into<String>>>) -> Self {
        Self {
            headers: headers.into_iter().map(|s| s.into()).collect(),
            rows: rows.into_iter().map(|row| row.into_iter().map(|c| c.into()).collect()).collect(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for DataTable {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;

        let header_cells: Vec<_> = self.headers.into_iter().map(|h| {
            let f = font.clone();
            bsn! {
                UNode {
                    width: UVal::Px(120.0),
                    padding: USides::axes(8.0, 6.0),
                }
                Children [
                    UText {
                        text: h,
                        font_size: 12.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: f,
                    }
                ]
            }
        }).collect();

        let header_cells_scene = SceneList::new(header_cells);
        let header_row = bsn! {
            UNode {
                padding: USides::axes(12.0, 6.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.04),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
            }
            Children [
                {header_cells_scene}
            ]
        };

        let row_nodes: Vec<_> = self.rows.into_iter().enumerate().map(|(r_idx, row)| {
            let bg = if r_idx % 2 == 1 {
                Color::srgba(1.0, 1.0, 1.0, 0.02)
            } else {
                Color::NONE
            };
            let cells: Vec<_> = row.into_iter().map(|cell| {
                let f = font.clone();
                bsn! {
                    UNode {
                        width: UVal::Px(120.0),
                        padding: USides::axes(8.0, 8.0),
                    }
                    Children [
                        UText {
                            text: cell,
                            font_size: 13.0,
                            color: Color::srgb(0.85, 0.88, 0.95),
                            font: f,
                        }
                    ]
                }
            }).collect();

            let cells_scene = SceneList::new(cells);
            bsn! {
                UNode {
                    padding: USides::axes(12.0, 0.0),
                    background_color: bg,
                }
                UBorder {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    width: 1.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                }
                Children [
                    {cells_scene}
                ]
            }
        }).collect();

        let row_nodes_scene = SceneList::new(row_nodes);
        let s = bsn! {
            UNode {
                background_color: Color::srgb(0.09, 0.10, 0.14),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
            }
            Children [
                {header_row}
                --
                {row_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.16 Timeline
// ============================================================================

/// Vertical activity timeline with connecting guide line and event markers.
#[derive(Clone, Debug, Reflect)]
pub struct Timeline {
    pub events: Vec<(String, String, String)>, // (time, title, desc)
    pub font: Handle<Font>,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            events: vec![
                ("10:00 AM".into(), "Build Completed".into(), "Bevy engine target compiled successfully.".into()),
                ("11:30 AM".into(), "Design System Updated".into(), "Merged 100 new declarative UI widgets.".into()),
                ("02:15 PM".into(), "Release V0.1.0".into(), "Published crate to cargo registry.".into()),
            ],
            font: Handle::default(),
        }
    }
}

impl Timeline {
    pub fn new(events: Vec<(impl Into<String>, impl Into<String>, impl Into<String>)>) -> Self {
        Self {
            events: events.into_iter().map(|(t, tit, d)| (t.into(), tit.into(), d.into())).collect(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Timeline {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;

        let event_nodes: Vec<_> = self.events.into_iter().map(|(time, title, desc)| {
            let f1 = font.clone();
            let f2 = font.clone();
            let f3 = font.clone();
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 12.0,
                }
                Children [
                    // Dot and line column
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UNode {
                            width: UVal::Px(10.0),
                            height: UVal::Px(10.0),
                            background_color: Color::srgb(0.35, 0.65, 1.0),
                            border_radius: UCornerRadius::all(5.0),
                        }
                        --
                        UNode {
                            width: UVal::Px(2.0),
                            height: UVal::Px(44.0),
                            background_color: Color::srgba(0.35, 0.65, 1.0, 0.3),
                        }
                    ]
                    --
                    // Content
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 2.0,
                    }
                    Children [
                        UText {
                            text: time,
                            font_size: 11.0,
                            color: Color::srgb(0.50, 0.55, 0.65),
                            font: f1,
                        }
                        --
                        UText {
                            text: title,
                            font_size: 14.0,
                            color: Color::srgb(0.96, 0.97, 0.99),
                            font: f2,
                        }
                        --
                        UText {
                            text: desc,
                            font_size: 12.0,
                            color: Color::srgb(0.65, 0.70, 0.80),
                            font: f3,
                        }
                    ]
                ]
            }
        }).collect();

        let event_nodes_scene = SceneList::new(event_nodes);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                {event_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.18 Comment Thread
// ============================================================================

/// Nested conversation comment thread with author and reply actions.
#[derive(Clone, Debug, Reflect)]
pub struct CommentThread {
    pub author: String,
    pub time_ago: String,
    pub text: String,
    pub font: Handle<Font>,
}

impl Default for CommentThread {
    fn default() -> Self {
        Self {
            author: "DevUser".into(),
            time_ago: "10m ago".into(),
            text: "This architecture makes assembling large-scale game interfaces so clean!".into(),
            font: Handle::default(),
        }
    }
}

impl CommentThread {
    pub fn new(author: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            author: author.into(),
            text: text.into(),
            ..default()
        }
    }

    pub fn time_ago(mut self, time: impl Into<String>) -> Self {
        self.time_ago = time.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for CommentThread {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let author = self.author;
        let time_ago = self.time_ago;
        let text = self.text;
        let font = self.font;
        let author_font = font.clone();
        let time_font = font.clone();
        let text_font = font.clone();
        let initial = author.chars().next().unwrap_or('D').to_string();

        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.02),
                border_radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
            }
            Children [
                @{Avatar::new(initial).size(AvatarSize::Small).font(font.clone())}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        UText {
                            text: author,
                            font_size: 13.0,
                            color: Color::srgb(0.96, 0.97, 0.99),
                            font: author_font,
                        }
                        --
                        UText {
                            text: time_ago,
                            font_size: 11.0,
                            color: Color::srgb(0.50, 0.55, 0.65),
                            font: time_font,
                        }
                    ]
                    --
                    UText {
                        text: text,
                        font_size: 13.0,
                        color: Color::srgb(0.80, 0.85, 0.92),
                        font: text_font,
                    }
                    --
                    @{Button::new("Reply").ghost().size(ButtonSize::Small).font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.19 Chat Bubble
// ============================================================================

/// Messaging chat bubble supporting sent (right) and received (left) modes.
#[derive(Clone, Debug, Reflect)]
pub struct ChatBubble {
    pub message: String,
    pub timestamp: String,
    pub is_outgoing: bool,
    pub font: Handle<Font>,
}

impl Default for ChatBubble {
    fn default() -> Self {
        Self {
            message: "Hello! Did you review the latest pull request?".into(),
            timestamp: "12:45 PM".into(),
            is_outgoing: false,
            font: Handle::default(),
        }
    }
}

impl ChatBubble {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..default()
        }
    }

    pub fn outgoing(mut self, outgoing: bool) -> Self {
        self.is_outgoing = outgoing;
        self
    }

    pub fn timestamp(mut self, time: impl Into<String>) -> Self {
        self.timestamp = time.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for ChatBubble {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (bg, text_color, align_justify) = if self.is_outgoing {
            (Color::srgb(0.20, 0.45, 0.85), Color::WHITE, UJustifyContent::End)
        } else {
            (Color::srgb(0.14, 0.16, 0.22), Color::srgb(0.92, 0.94, 0.98), UJustifyContent::Start)
        };
        let message = self.message;
        let timestamp = self.timestamp;
        let font = self.font;
        let msg_font = font.clone();
        let time_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: align_justify,
            }
            Children [
                UNode {
                    padding: USides::axes(14.0, 10.0),
                    background_color: bg,
                    border_radius: UCornerRadius::all(14.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    UText {
                        text: message,
                        font_size: 13.0,
                        color: text_color,
                        font: msg_font,
                    }
                    --
                    UText {
                        text: timestamp,
                        font_size: 10.0,
                        color: Color::srgba(1.0, 1.0, 1.0, 0.6),
                        font: time_font,
                    }
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.20 Media Player
// ============================================================================

/// Audio / Video media playback controller card.
#[derive(Clone, Debug, Reflect)]
pub struct MediaPlayer {
    pub title: String,
    pub artist: String,
    pub progress: f32, // 0.0 to 1.0
    pub is_playing: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self {
            title: "Ambient Synthesizer Theme".into(),
            artist: "Luma Soundscape".into(),
            progress: 0.45,
            is_playing: true,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl MediaPlayer {
    pub fn new(title: impl Into<String>, artist: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            artist: artist.into(),
            ..default()
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn playing(mut self, playing: bool) -> Self {
        self.is_playing = playing;
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
}

impl Scene for MediaPlayer {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let play_icon = if self.is_playing { Icon::X } else { Icon::CIRCLE_PLAY };
        let title = self.title;
        let artist = self.artist;
        let progress = self.progress;
        let font = self.font;
        let icon_font = self.icon_font;
        let title_font = font.clone();
        let artist_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(320.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 2.0,
                }
                Children [
                    UText {
                        text: title,
                        font_size: 15.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: title_font,
                    }
                    --
                    UText {
                        text: artist,
                        font_size: 12.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: artist_font,
                    }
                ]
                --
                @{ProgressBar::new(progress)}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 16.0,
                }
                Children [
                    @{Button::icon_button(Icon::CHEVRON_LEFT).ghost().size(ButtonSize::Small).icon_font(icon_font.clone())}
                    --
                    @{Button::icon_button(play_icon).size(ButtonSize::Medium).icon_font(icon_font.clone())}
                    --
                    @{Button::icon_button(Icon::CHEVRON_RIGHT).ghost().size(ButtonSize::Small).icon_font(icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.21 Image Lightbox
// ============================================================================

/// Full-screen image preview overlay modal.
#[derive(Clone, Debug, Reflect)]
pub struct ImageLightbox {
    pub caption: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ImageLightbox {
    fn default() -> Self {
        Self {
            caption: "High-resolution render preview".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ImageLightbox {
    pub fn new(caption: impl Into<String>) -> Self {
        Self {
            caption: caption.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for ImageLightbox {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let caption = self.caption;
        let font = self.font;
        let icon_font = self.icon_font;
        let s = bsn! {
            UNode {
                width: UVal::Px(480.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.05, 0.06, 0.09, 0.98),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                // Top close action
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::End,
                }
                Children [
                    @{Button::icon_button(Icon::X).ghost().size(ButtonSize::Small).icon_font(icon_font)}
                ]
                --
                // Image canvas placeholder
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(260.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    border_radius: UCornerRadius::all(8.0),
                }
                --
                UText {
                    text: caption,
                    font_size: 13.0,
                    color: Color::srgb(0.85, 0.88, 0.95),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.22 Bar Chart
// ============================================================================

/// SDF Bar chart visualization representing numerical categorical data.
#[derive(Clone, Debug, Reflect)]
pub struct BarChart {
    pub data: Vec<(String, f32)>, // (label, value 0.0..=1.0)
    pub font: Handle<Font>,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            data: vec![
                ("Mon".into(), 0.3),
                ("Tue".into(), 0.65),
                ("Wed".into(), 0.45),
                ("Thu".into(), 0.85),
                ("Fri".into(), 0.70),
            ],
            font: Handle::default(),
        }
    }
}

impl BarChart {
    pub fn new(data: Vec<(impl Into<String>, f32)>) -> Self {
        Self {
            data: data.into_iter().map(|(l, v)| (l.into(), v.clamp(0.0, 1.0))).collect(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for BarChart {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;

        let bars: Vec<_> = self.data.into_iter().map(|(label, val)| {
            let f = font.clone();
            let bar_h = UVal::Px(val * 100.0);
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::End,
                    gap: 6.0,
                }
                Children [
                    UNode {
                        width: UVal::Px(24.0),
                        height: bar_h,
                        background_color: Color::srgb(0.35, 0.65, 1.0),
                        border_radius: UCornerRadius::top(4.0),
                    }
                    --
                    UText {
                        text: label,
                        font_size: 11.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: f,
                    }
                ]
            }
        }).collect();

        let bars_scene = SceneList::new(bars);
        let s = bsn! {
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(160.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.09, 0.10, 0.15),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceAround,
                align_items: UAlignItems::End,
            }
            Children [
                {bars_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.23 Donut Chart
// ============================================================================

/// Circular donut progress and metric representation.
#[derive(Clone, Debug, Reflect)]
pub struct DonutChart {
    pub percent: f32, // 0.0 to 1.0
    pub label: String,
    pub font: Handle<Font>,
}

impl Default for DonutChart {
    fn default() -> Self {
        Self {
            percent: 0.78,
            label: "Storage Used".into(),
            font: Handle::default(),
        }
    }
}

impl DonutChart {
    pub fn new(percent: f32, label: impl Into<String>) -> Self {
        Self {
            percent: percent.clamp(0.0, 1.0),
            label: label.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for DonutChart {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let label = self.label;
        let font = self.font;
        let p_font = font.clone();
        let l_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(180.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.09, 0.10, 0.15),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 10.0,
            }
            Children [
                // Circular progress ring
                @{ CircularProgress::new(self.percent).size(90.0).thickness(8.0).font(p_font) }
                --
                UText {
                    text: label,
                    font_size: 13.0,
                    color: Color::srgb(0.60, 0.65, 0.75),
                    font: l_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.6 Notification Center
// ============================================================================

/// Popover card listing recent user notifications.
#[derive(Clone, Debug, Reflect)]
pub struct NotificationCenter {
    pub notifications: Vec<(String, String)>, // (title, time)
    pub font: Handle<Font>,
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self {
            notifications: vec![
                ("New build ready for testing".into(), "5m ago".into()),
                ("Security audit passed".into(), "1h ago".into()),
                ("Server storage reached 80%".into(), "2h ago".into()),
            ],
            font: Handle::default(),
        }
    }
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for NotificationCenter {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let notif_rows: Vec<_> = self.notifications.into_iter().map(|(tit, time)| {
            let f1 = font.clone();
            let f2 = font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(12.0, 8.0),
                    border_radius: UCornerRadius::all(6.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 2.0,
                }
                Children [
                    UText {
                        text: tit,
                        font_size: 13.0,
                        color: Color::srgb(0.92, 0.94, 0.98),
                        font: f1,
                    }
                    --
                    UText {
                        text: time,
                        font_size: 11.0,
                        color: Color::srgb(0.50, 0.55, 0.65),
                        font: f2,
                    }
                ]
            }
        }).collect();

        let notif_scene = SceneList::new(notif_rows);
        let header_font = font.clone();
        let header_title = "Notifications".to_string();
        let s = bsn! {
            UNode {
                width: UVal::Px(320.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
            }
            Children [
                UText {
                    text: header_title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: header_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    {notif_scene}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.16 Cart Drawer
// ============================================================================

/// Shopping cart sliding drawer with product items and checkout button.
#[derive(Clone, Debug, Reflect)]
pub struct CartDrawer {
    pub subtotal: String,
    pub font: Handle<Font>,
}

impl Default for CartDrawer {
    fn default() -> Self {
        Self {
            subtotal: "$399.98".into(),
            font: Handle::default(),
        }
    }
}

impl CartDrawer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subtotal(mut self, subtotal: impl Into<String>) -> Self {
        self.subtotal = subtotal.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for CartDrawer {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let subtotal = self.subtotal;
        let font = self.font;
        let title_font = font.clone();
        let sub_font = font.clone();
        let val_font = font.clone();
        let btn_font = font;
        let cart_title = "Your Cart".to_string();
        let subtotal_label = "Subtotal".to_string();

        let s = bsn! {
            UNode {
                width: UVal::Px(360.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                background_color: Color::srgb(0.09, 0.11, 0.16),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::default(),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 16.0,
                }
                Children [
                    UText {
                        text: cart_title,
                        font_size: 18.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: title_font,
                    }
                ]
                --
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 16.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: subtotal_label,
                            font_size: 14.0,
                            color: Color::srgb(0.65, 0.70, 0.80),
                            font: sub_font,
                        }
                        --
                        UText {
                            text: subtotal,
                            font_size: 16.0,
                            color: Color::srgb(0.96, 0.97, 0.99),
                            font: val_font,
                        }
                    ]
                    --
                    @{Button::new("Proceed to Checkout").size(ButtonSize::Large).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.17 Kanban Column & Card
// ============================================================================

/// Individual task card within a Kanban board column.
#[derive(Clone, Debug, Reflect)]
pub struct KanbanCard {
    pub title: String,
    pub tag: String,
    pub font: Handle<Font>,
}

impl Default for KanbanCard {
    fn default() -> Self {
        Self {
            title: "Implement Signed Distance Field Text".into(),
            tag: "High Priority".into(),
            font: Handle::default(),
        }
    }
}

impl KanbanCard {
    pub fn new(title: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            tag: tag.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for KanbanCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let tag = self.tag;
        let font = self.font;
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.12, 0.14, 0.20),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                @{Badge::danger(tag)}
                --
                UText {
                    text: title,
                    font_size: 13.0,
                    color: Color::srgb(0.92, 0.94, 0.98),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

/// Kanban board vertical column container.
#[derive(Clone, Debug, Reflect)]
pub struct KanbanColumn {
    pub title: String,
    pub count: usize,
    pub font: Handle<Font>,
}

impl Default for KanbanColumn {
    fn default() -> Self {
        Self {
            title: "In Progress".into(),
            count: 3,
            font: Handle::default(),
        }
    }
}

impl KanbanColumn {
    pub fn new(title: impl Into<String>, count: usize) -> Self {
        Self {
            title: title.into(),
            count,
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for KanbanColumn {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let count_str = self.count.to_string();
        let font = self.font;
        let title_font = font.clone();
        let count_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(280.0),
                padding: USides::all(14.0),
                background_color: Color::srgb(0.08, 0.09, 0.13),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.16, 0.19, 0.26),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: title,
                        font_size: 14.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: title_font,
                    }
                    --
                    @{Badge::neutral(count_str).font(count_font)}
                ]
                --
                @{KanbanCard::default()}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.18 Calendar Schedule View
// ============================================================================

/// Day planner schedule view displaying hourly agenda slots.
#[derive(Clone, Debug, Reflect)]
pub struct CalendarSchedule {
    pub day_label: String,
    pub events: Vec<(String, String)>, // (time, event)
    pub font: Handle<Font>,
}

impl Default for CalendarSchedule {
    fn default() -> Self {
        Self {
            day_label: "Wednesday, October 15".into(),
            events: vec![
                ("09:00 AM".into(), "Design System Architecture Sync".into()),
                ("11:30 AM".into(), "SDF Shader Pipeline Review".into()),
                ("03:00 PM".into(), "Bevy UI Demo & Showcase".into()),
            ],
            font: Handle::default(),
        }
    }
}

impl CalendarSchedule {
    pub fn new(day: impl Into<String>) -> Self {
        Self {
            day_label: day.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for CalendarSchedule {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let day_label = self.day_label;
        let font = self.font;
        let day_font = font.clone();

        let slots: Vec<_> = self.events.into_iter().map(|(time, evt)| {
            let f1 = font.clone();
            let f2 = font.clone();
            bsn! {
                UNode {
                    padding: USides::all(10.0),
                    background_color: Color::srgba(0.35, 0.65, 1.0, 0.08),
                    border_radius: UCornerRadius::all(6.0),
                }
                UBorder {
                    color: Color::srgba(0.35, 0.65, 1.0, 0.25),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    UText {
                        text: time,
                        font_size: 12.0,
                        color: Color::srgb(0.35, 0.65, 1.0),
                        font: f1,
                    }
                    --
                    UText {
                        text: evt,
                        font_size: 13.0,
                        color: Color::srgb(0.92, 0.94, 0.98),
                        font: f2,
                    }
                ]
            }
        }).collect();

        let slots_scene = SceneList::new(slots);
        let s = bsn! {
            UNode {
                width: UVal::Px(340.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.09, 0.10, 0.15),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: day_label,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: day_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    {slots_scene}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}
