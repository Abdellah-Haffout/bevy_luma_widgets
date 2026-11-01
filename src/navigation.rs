//! Navigation and structural widgets for Bevy Luma.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::prelude::*;
use crate::button::Button;

/// Helper wrapper allowing dynamic vectors of scenes to resolve sequentially into a BSN container.
pub struct SceneList<T: Scene>(pub Vec<T>);

impl<T: Scene> SceneList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self(items)
    }
}

impl<T: Scene> Scene for SceneList<T> {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        for item in self.0 {
            item.resolve(context, scene)?;
        }
        Ok(())
    }
}

impl<T: Scene> bevy::scene::SceneList for SceneList<T> {
    fn resolve_list(
        self,
        context: &mut ResolveContext,
        scenes: &mut Vec<ResolvedScene>,
    ) -> Result<(), ResolveSceneError> {
        self.0.resolve_list(context, scenes)
    }

    fn register_dependencies(&self, dependencies: &mut bevy::scene::SceneDependencies) {
        self.0.register_dependencies(dependencies);
    }
}

// ============================================================================
// 2.1 Navbar / Top App Bar
// ============================================================================

/// Top navigation bar with brand, page links, and action slots.
#[derive(Clone, Debug, Reflect)]
pub struct Navbar {
    pub brand: String,
    pub items: Vec<String>,
    pub active_index: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Navbar {
    fn default() -> Self {
        Self {
            brand: "Bevy App".into(),
            items: vec!["Home".into(), "Features".into(), "Docs".into(), "Pricing".into()],
            active_index: 0,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Navbar {
    pub fn new(brand: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            ..default()
        }
    }

    pub fn items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
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

impl Scene for Navbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let brand = self.brand;
        let font = self.font;
        let icon_font = self.icon_font;
        let active_index = self.active_index;

        let nav_links: Vec<_> = self.items.into_iter().enumerate().map(|(idx, item)| {
            let is_active = idx == active_index;
            let text_color = if is_active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.65, 0.70, 0.80)
            };
            let weight_color = if is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::NONE
            };
            let font_c = font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(12.0, 8.0),
                }
                UBorder {
                    color: weight_color,
                    width: 2.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: item,
                        font_size: 14.0,
                        color: text_color,
                        font: font_c,
                    }
                ]
            }
        }).collect();

        let nav_links_scene = SceneList::new(nav_links);
        let brand_font = font.clone();
        let signin_font = font.clone();
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(56.0),
                padding: USides::axes(20.0, 0.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.95),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
            }
            Children [
                // Brand / Logo
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    @{Button::icon_button(Icon::STAR).size(ButtonSize::Small).icon_font(icon_font)}
                    --
                    UText {
                        text: brand,
                        font_size: 16.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: brand_font,
                    }
                ]
                --
                // Navigation links
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 4.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    {nav_links_scene}
                ]
                --
                // Right actions / Profile
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{Button::new("Sign In").outline().size(ButtonSize::Small).font(signin_font)}
                    --
                    @{Avatar::new("AD").size(AvatarSize::Small).font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.2 Bottom Navigation Bar
// ============================================================================

/// Mobile bottom navigation bar with icon & label destinations.
#[derive(Clone, Debug, Reflect)]
pub struct BottomNav {
    pub items: Vec<(String, String)>, // (icon, label)
    pub active_index: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for BottomNav {
    fn default() -> Self {
        Self {
            items: vec![
                (Icon::STAR.to_string(), "Home".into()),
                (Icon::SEARCH.to_string(), "Search".into()),
                (Icon::BELL.to_string(), "Alerts".into()),
                (Icon::USER.to_string(), "Profile".into()),
            ],
            active_index: 0,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl BottomNav {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<(String, String)>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
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

impl Scene for BottomNav {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active_index = self.active_index;
        let font = self.font;
        let icon_font = self.icon_font;

        let nav_items: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (icon, label))| {
            let is_active = idx == active_index;
            let color = if is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgb(0.55, 0.60, 0.70)
            };
            let f = font.clone();
            let ifont = icon_font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(8.0, 4.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    gap: 4.0,
                }
                Children [
                    UText {
                        text: icon,
                        font_size: 18.0,
                        color,
                        font: ifont,
                    }
                    --
                    UText {
                        text: label,
                        font_size: 11.0,
                        color,
                        font: f,
                    }
                ]
            }
        }).collect();

        let nav_items_scene = SceneList::new(nav_items);
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(58.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceAround,
                align_items: UAlignItems::Center,
            }
            Children [
                {nav_items_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.3 Sidebar / Navigation Drawer
// ============================================================================

/// Vertical navigation sidebar with brand header and menu items.
#[derive(Clone, Debug, Reflect)]
pub struct Sidebar {
    pub brand: String,
    pub items: Vec<(String, String)>, // (icon, label)
    pub active_index: usize,
    pub collapsed: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            brand: "Dashboard".into(),
            items: vec![
                (Icon::STAR.to_string(), "Overview".into()),
                (Icon::FOLDER.to_string(), "Projects".into()),
                (Icon::USER.to_string(), "Team".into()),
                (Icon::SETTINGS.to_string(), "Settings".into()),
            ],
            active_index: 0,
            collapsed: false,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn brand(mut self, brand: impl Into<String>) -> Self {
        self.brand = brand.into();
        self
    }

    pub fn items(mut self, items: Vec<(String, String)>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
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

impl Scene for Sidebar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active_index = self.active_index;
        let font = self.font;
        let icon_font = self.icon_font;
        let brand = self.brand;

        if self.collapsed {
            let menu_items: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (icon, _))| {
                let is_active = idx == active_index;
                let bg = if is_active { Color::srgba(0.35, 0.65, 1.0, 0.15) } else { Color::NONE };
                let text_color = if is_active { Color::srgb(0.35, 0.65, 1.0) } else { Color::srgb(0.70, 0.75, 0.85) };
                let ifont = icon_font.clone();
                bsn! {
                    UNode {
                        height: UVal::Px(40.0),
                        background_color: bg,
                        border_radius: UCornerRadius::all(6.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: icon,
                            font_size: 16.0,
                            color: text_color,
                            font: ifont,
                        }
                    ]
                }
            }).collect();

            let menu_items_scene = SceneList::new(menu_items);
            let s = bsn! {
                UNode {
                    width: UVal::Px(64.0),
                    height: UVal::Percent(1.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgba(0.07, 0.08, 0.12, 0.98),
                }
                UBorder {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                    width: 1.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 8.0,
                    }
                    Children [
                        {menu_items_scene}
                    ]
                    --
                    UNode {
                        padding: USides::all(8.0),
                    }
                    Children [
                        @{Avatar::new("US").size(AvatarSize::Small).font(font)}
                    ]
                ]
            };
            s.resolve(context, scene)
        } else {
            let menu_items: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (icon, label))| {
                let is_active = idx == active_index;
                let bg = if is_active { Color::srgba(0.35, 0.65, 1.0, 0.15) } else { Color::NONE };
                let text_color = if is_active { Color::srgb(0.35, 0.65, 1.0) } else { Color::srgb(0.70, 0.75, 0.85) };
                let f = font.clone();
                let ifont = icon_font.clone();
                bsn! {
                    UNode {
                        height: UVal::Px(40.0),
                        padding: USides::axes(12.0, 0.0),
                        background_color: bg,
                        border_radius: UCornerRadius::all(6.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 12.0,
                    }
                    Children [
                        UText {
                            text: icon,
                            font_size: 16.0,
                            color: text_color,
                            font: ifont,
                        }
                        --
                        UText {
                            text: label,
                            font_size: 14.0,
                            color: text_color,
                            font: f,
                        }
                    ]
                }
            }).collect();

            let menu_items_scene = SceneList::new(menu_items);
            let header_font = font.clone();
            let s = bsn! {
                UNode {
                    width: UVal::Px(240.0),
                    height: UVal::Percent(1.0),
                    padding: USides::all(16.0),
                    background_color: Color::srgba(0.07, 0.08, 0.12, 0.98),
                }
                UBorder {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                    width: 1.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 8.0,
                    }
                    Children [
                        UNode {
                            height: UVal::Px(40.0),
                            padding: USides::axes(8.0, 0.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            UText {
                                text: brand,
                                font_size: 16.0,
                                color: Color::srgb(0.96, 0.97, 0.99),
                                font: header_font,
                            }
                        ]
                        --
                        {menu_items_scene}
                    ]
                    --
                    UNode {
                        padding: USides::all(8.0),
                    }
                    UBorder {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                        width: 1.0,
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{Avatar::new("US").size(AvatarSize::Small).font(font)}
                    ]
                ]
            };
            s.resolve(context, scene)
        }
    }
}

// ============================================================================
// 2.4 Breadcrumbs Wrapper
// ============================================================================

/// Declarative Breadcrumbs trail for multiple path items.
#[derive(Clone, Debug, Reflect)]
pub struct Breadcrumbs {
    pub items: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self {
            items: vec!["Home".into(), "Projects".into(), "Luma UI".into()],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Breadcrumbs {
    pub fn new(items: Vec<impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(|s| s.into()).collect(),
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

impl Scene for Breadcrumbs {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total = self.items.len();
        let font = self.font;

        let mut crumb_nodes = Vec::new();
        for (idx, item) in self.items.into_iter().enumerate() {
            let is_last = idx + 1 == total;
            let f = font.clone();
            crumb_nodes.push(
                BreadcrumbItem::new(format!("crumb_{idx}"), item)
                    .index(idx)
                    .current(is_last)
                    .font(f)
            );
        }

        let crumb_nodes_scene = SceneList::new(crumb_nodes);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                {crumb_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.6 Pagination
// ============================================================================

/// Pagination row with previous, page number pills, and next buttons.
#[derive(Clone, Debug, Reflect)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 5,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Pagination {
    pub fn new(current: usize, total: usize) -> Self {
        Self {
            current_page: current,
            total_pages: total,
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

impl Scene for Pagination {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let current = self.current_page;
        let font = self.font;
        let icon_font = self.icon_font;
        let prev_icon_font = icon_font.clone();

        let page_buttons: Vec<_> = (1..=self.total_pages).map(|p| {
            let is_active = p == current;
            let f = font.clone();
            if is_active {
                Button::new(p.to_string()).size(ButtonSize::Small).font(f)
            } else {
                Button::new(p.to_string()).outline().size(ButtonSize::Small).font(f)
            }
        }).collect();

        let page_buttons_scene = SceneList::new(page_buttons);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                @{Button::icon_button(Icon::CHEVRON_LEFT).outline().size(ButtonSize::Small).icon_font(prev_icon_font)}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 4.0,
                }
                Children [
                    {page_buttons_scene}
                ]
                --
                @{Button::icon_button(Icon::CHEVRON_RIGHT).outline().size(ButtonSize::Small).icon_font(icon_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.7 StepTracker / Progress Stepper
// ============================================================================

/// Multi-step workflow progression tracker.
#[derive(Clone, Debug, Reflect)]
pub struct StepTracker {
    pub steps: Vec<String>,
    pub current_step: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for StepTracker {
    fn default() -> Self {
        Self {
            steps: vec!["Details".into(), "Payment".into(), "Review".into()],
            current_step: 1,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl StepTracker {
    pub fn new(steps: Vec<impl Into<String>>) -> Self {
        Self {
            steps: steps.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn current(mut self, step: usize) -> Self {
        self.current_step = step;
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

impl Scene for StepTracker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let current = self.current_step;
        let total = self.steps.len();
        let font = self.font;
        let icon_font = self.icon_font;

        let step_nodes: Vec<_> = self.steps.into_iter().enumerate().map(|(idx, title)| {
            let is_done = idx < current;
            let is_active = idx == current;
            let circle_bg = if is_done || is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };
            let text_color = if is_active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.6, 0.65, 0.75)
            };
            let f = font.clone();
            let ifont = icon_font.clone();
            let step_num = (idx + 1).to_string();

            let (node_text, node_font) = if is_done {
                (Icon::CHECK.to_string(), ifont)
            } else {
                (step_num, f.clone())
            };

            let line_color = if idx < current {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };
            let line_w = if idx + 1 < total { UVal::Px(32.0) } else { UVal::Px(0.0) };

            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    // Circle node
                    UNode {
                        width: UVal::Px(28.0),
                        height: UVal::Px(28.0),
                        border_radius: UCornerRadius::all(14.0),
                        background_color: circle_bg,
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: node_text,
                            font_size: 12.0,
                            color: Color::WHITE,
                            font: node_font,
                        }
                    ]
                    --
                    // Title
                    UText {
                        text: title,
                        font_size: 13.0,
                        color: text_color,
                        font: f,
                    }
                    --
                    // Connecting line if not last
                    UNode {
                        width: line_w,
                        height: UVal::Px(2.0),
                        background_color: line_color,
                        margin: USides::axes(4.0, 0.0),
                    }
                ]
            }
        }).collect();

        let step_nodes_scene = SceneList::new(step_nodes);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                {step_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.8 Hamburger Menu Icon
// ============================================================================

/// Hamburger menu button toggling between 3-bars and X.
#[derive(Clone, Debug, Reflect)]
pub struct HamburgerMenu {
    pub is_open: bool,
    pub icon_font: Handle<Font>,
}

impl Default for HamburgerMenu {
    fn default() -> Self {
        Self {
            is_open: false,
            icon_font: Handle::default(),
        }
    }
}

impl HamburgerMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for HamburgerMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let icon = if self.is_open { Icon::X } else { Icon::MENU };
        let btn = Button::icon_button(icon)
            .ghost()
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

// ============================================================================
// 2.9 Mega Menu
// ============================================================================

/// Multi-column categorized mega menu popover panel.
#[derive(Clone, Debug, Reflect)]
pub struct MegaMenu {
    pub title: String,
    pub categories: Vec<(String, Vec<String>)>,
    pub font: Handle<Font>,
}

impl Default for MegaMenu {
    fn default() -> Self {
        Self {
            title: "Products & Solutions".into(),
            categories: vec![
                ("Design System".into(), vec!["Components".into(), "Icons".into(), "Tokens".into()]),
                ("Framework".into(), vec!["SDF Pipeline".into(), "BSN Macro".into(), "ECS Layout".into()]),
                ("Ecosystem".into(), vec!["Community".into(), "Plugins".into(), "Showcase".into()]),
            ],
            font: Handle::default(),
        }
    }
}

impl MegaMenu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn categories(mut self, cats: Vec<(String, Vec<String>)>) -> Self {
        self.categories = cats;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for MegaMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let cols: Vec<_> = self.categories.into_iter().map(|(cat_title, links)| {
            let f1 = font.clone();
            let links_nodes: Vec<_> = links.into_iter().map(|lnk| {
                let f2 = font.clone();
                bsn! {
                    UNode {
                        padding: USides::axes(0.0, 4.0),
                    }
                    Children [
                        UText {
                            text: lnk,
                            font_size: 13.0,
                            color: Color::srgb(0.70, 0.75, 0.85),
                            font: f2,
                        }
                    ]
                }
            }).collect();

            let links_scene = SceneList::new(links_nodes);
            bsn! {
                UNode {
                    width: UVal::Px(160.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    UText {
                        text: cat_title,
                        font_size: 14.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: f1,
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 4.0,
                    }
                    Children [
                        {links_scene}
                    ]
                ]
            }
        }).collect();

        let cols_scene = SceneList::new(cols);
        let s = bsn! {
            UNode {
                padding: USides::all(20.0),
                background_color: Color::srgba(0.09, 0.10, 0.15, 0.98),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.12),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 24.0,
            }
            Children [
                {cols_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.10 Footer
// ============================================================================

/// Site footer with multi-column links and copyright row.
#[derive(Clone, Debug, Reflect)]
pub struct Footer {
    pub copyright: String,
    pub columns: Vec<(String, Vec<String>)>,
    pub font: Handle<Font>,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            copyright: "© 2026 Bevy Luma UI. All rights reserved.".into(),
            columns: vec![
                ("Product".into(), vec!["Overview".into(), "Features".into(), "Releases".into()]),
                ("Resources".into(), vec!["Docs".into(), "Learn".into(), "Blog".into()]),
                ("Company".into(), vec!["About".into(), "Careers".into(), "Contact".into()]),
            ],
            font: Handle::default(),
        }
    }
}

impl Footer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copyright(mut self, cr: impl Into<String>) -> Self {
        self.copyright = cr.into();
        self
    }

    pub fn columns(mut self, cols: Vec<(String, Vec<String>)>) -> Self {
        self.columns = cols;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Footer {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let copyright = self.copyright;

        let col_nodes: Vec<_> = self.columns.into_iter().map(|(col_title, items)| {
            let f = font.clone();
            let item_nodes: Vec<_> = items.into_iter().map(|item| {
                let f2 = font.clone();
                bsn! {
                    UText {
                        text: item,
                        font_size: 13.0,
                        color: Color::srgb(0.60, 0.65, 0.75),
                        font: f2,
                    }
                }
            }).collect();

            let item_nodes_scene = SceneList::new(item_nodes);
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    UText {
                        text: col_title,
                        font_size: 14.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: f,
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 6.0,
                    }
                    Children [
                        {item_nodes_scene}
                    ]
                ]
            }
        }).collect();

        let col_nodes_scene = SceneList::new(col_nodes);
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(32.0),
                background_color: Color::srgba(0.06, 0.07, 0.10, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    {col_nodes_scene}
                ]
                --
                @{Divider::horizontal()}
                --
                UText {
                    text: copyright,
                    font_size: 12.0,
                    color: Color::srgb(0.45, 0.50, 0.60),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.11 Context Menu
// ============================================================================

/// Right-click contextual popover menu.
#[derive(Clone, Debug, Reflect)]
pub struct ContextMenu {
    pub items: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            items: vec!["Cut".into(), "Copy".into(), "Paste".into(), "Delete".into()],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ContextMenu {
    pub fn new(items: Vec<impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(|s| s.into()).collect(),
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

impl Scene for ContextMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let rows: Vec<_> = self.items.into_iter().map(|item| {
            let f = font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    UText {
                        text: item,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            UNode {
                width: UVal::Px(160.0),
                padding: USides::all(6.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
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
                gap: 2.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.12 Command Palette
// ============================================================================

/// Fast keyboard command launcher (Ctrl + K) overlay dialog.
#[derive(Clone, Debug, Reflect)]
pub struct CommandPalette {
    pub placeholder: String,
    pub commands: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            placeholder: "Type a command or search...".into(),
            commands: vec![
                "Open Workspace Settings".into(),
                "Toggle Dark / Light Theme".into(),
                "Go to File...".into(),
                "Open Documentation".into(),
            ],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl CommandPalette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn commands(mut self, cmds: Vec<impl Into<String>>) -> Self {
        self.commands = cmds.into_iter().map(|s| s.into()).collect();
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

impl Scene for CommandPalette {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;

        let cmd_rows: Vec<_> = self.commands.into_iter().map(|cmd| {
            let f = font.clone();
            let ifont = icon_font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(12.0, 8.0),
                    border_radius: UCornerRadius::all(6.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    UText {
                        text: { Icon::SEARCH.to_string() },
                        font_size: 14.0,
                        color: Color::srgb(0.5, 0.55, 0.65),
                        font: ifont,
                    }
                    --
                    UText {
                        text: cmd,
                        font_size: 13.0,
                        color: Color::srgb(0.9, 0.92, 0.98),
                        font: f,
                    }
                ]
            }
        }).collect();

        let cmd_rows_scene = SceneList::new(cmd_rows);
        let input_font = font.clone();
        let input_ifont = icon_font.clone();
        let s = bsn! {
            UNode {
                width: UVal::Px(480.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.09, 0.10, 0.15, 0.98),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                @{TextInput::new(self.placeholder).leading_icon(Icon::SEARCH).font(input_font).icon_font(input_ifont)}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    {cmd_rows_scene}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.13 Custom Scrollbar
// ============================================================================

/// Styled SDF scrollbar track & thumb.
#[derive(Clone, Debug, Reflect)]
pub struct CustomScrollbar {
    pub progress: f32, // 0.0 to 1.0
    pub is_horizontal: bool,
}

impl Default for CustomScrollbar {
    fn default() -> Self {
        Self {
            progress: 0.25,
            is_horizontal: false,
        }
    }
}

impl CustomScrollbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }
}

impl Scene for CustomScrollbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let thumb_margin = self.progress * 100.0;
        if self.is_horizontal {
            let s = bsn! {
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(8.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    UNode {
                        width: UVal::Px(40.0),
                        height: UVal::Px(8.0),
                        margin: USides { top: 0.0, right: 0.0, bottom: 0.0, left: thumb_margin },
                        background_color: Color::srgba(0.4, 0.45, 0.6, 0.7),
                        border_radius: UCornerRadius::all(4.0),
                    }
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                UNode {
                    width: UVal::Px(8.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    UNode {
                        width: UVal::Px(8.0),
                        height: UVal::Px(40.0),
                        margin: USides { top: thumb_margin, right: 0.0, bottom: 0.0, left: 0.0 },
                        background_color: Color::srgba(0.4, 0.45, 0.6, 0.7),
                        border_radius: UCornerRadius::all(4.0),
                    }
                ]
            };
            s.resolve(context, scene)
        }
    }
}

// ============================================================================
// 2.14 Back to Top Button
// ============================================================================

/// Floating circle button to scroll back to top of page.
#[derive(Clone, Debug, Reflect)]
pub struct BackToTopButton {
    pub icon_font: Handle<Font>,
}

impl Default for BackToTopButton {
    fn default() -> Self {
        Self {
            icon_font: Handle::default(),
        }
    }
}

impl BackToTopButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for BackToTopButton {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let btn = Button::fab_button(Icon::CHEVRON_UP)
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

// ============================================================================
// 2.15 Action Sheet
// ============================================================================

/// Bottom mobile modal sheet for contextual action menus.
#[derive(Clone, Debug, Reflect)]
pub struct ActionSheet {
    pub title: String,
    pub actions: Vec<String>,
    pub cancel_label: String,
    pub font: Handle<Font>,
}

impl Default for ActionSheet {
    fn default() -> Self {
        Self {
            title: "Select an Action".into(),
            actions: vec!["Share Link".into(), "Save to Library".into(), "Report".into()],
            cancel_label: "Cancel".into(),
            font: Handle::default(),
        }
    }
}

impl ActionSheet {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn actions(mut self, actions: Vec<impl Into<String>>) -> Self {
        self.actions = actions.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn cancel(mut self, cancel: impl Into<String>) -> Self {
        self.cancel_label = cancel.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for ActionSheet {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let title = self.title;
        let action_btns: Vec<_> = self.actions.into_iter().map(|act| {
            let f = font.clone();
            Button::new(act).secondary().size(ButtonSize::Large).font(f)
        }).collect();

        let action_btns_scene = SceneList::new(action_btns);
        let cancel_font = font.clone();
        let title_font = font.clone();
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
                border_radius: UCornerRadius::top(16.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                UText {
                    text: title,
                    font_size: 14.0,
                    color: Color::srgb(0.65, 0.7, 0.8),
                    font: title_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    {action_btns_scene}
                ]
                --
                @{Divider::horizontal()}
                --
                @{Button::new(self.cancel_label).outline().size(ButtonSize::Large).font(cancel_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.16 Bottom Sheet
// ============================================================================

/// Draggable mobile bottom drawer sheet panel.
#[derive(Clone, Debug, Reflect)]
pub struct BottomSheet {
    pub title: String,
    pub font: Handle<Font>,
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self {
            title: "Details Panel".into(),
            font: Handle::default(),
        }
    }
}

impl BottomSheet {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for BottomSheet {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let font = self.font;
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(220.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
                border_radius: UCornerRadius::top(16.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.12),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                // Grab handle
                UNode {
                    width: UVal::Px(36.0),
                    height: UVal::Px(4.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                    border_radius: UCornerRadius::all(2.0),
                }
                --
                UText {
                    text: title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.17 Splitter / Resizable Handle
// ============================================================================

/// Resizable dividing pane splitter bar.
#[derive(Clone, Debug, Reflect)]
pub struct Splitter {
    pub is_horizontal: bool,
}

impl Default for Splitter {
    fn default() -> Self {
        Self {
            is_horizontal: false,
        }
    }
}

impl Splitter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }
}

impl Scene for Splitter {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        if self.is_horizontal {
            let s = bsn! {
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(6.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UNode {
                        width: UVal::Px(32.0),
                        height: UVal::Px(3.0),
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                        border_radius: UCornerRadius::all(1.5),
                    }
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                UNode {
                    width: UVal::Px(6.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UNode {
                        width: UVal::Px(3.0),
                        height: UVal::Px(32.0),
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                        border_radius: UCornerRadius::all(1.5),
                    }
                ]
            };
            s.resolve(context, scene)
        }
    }
}

// ============================================================================
// 2.18 Tree View
// ============================================================================

/// Hierarchical tree directory view with expandable nodes.
#[derive(Clone, Debug, Reflect)]
pub struct TreeView {
    pub items: Vec<(usize, String, bool)>, // (depth, label, is_folder)
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TreeView {
    fn default() -> Self {
        Self {
            items: vec![
                (0, "src".into(), true),
                (1, "main.rs".into(), false),
                (1, "lib.rs".into(), false),
                (0, "assets".into(), true),
                (1, "textures".into(), true),
            ],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl TreeView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<(usize, String, bool)>) -> Self {
        self.items = items;
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

impl Scene for TreeView {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;

        let rows: Vec<_> = self.items.into_iter().map(|(depth, label, is_folder)| {
            let indent = depth as f32 * 16.0;
            let icon = if is_folder { Icon::FOLDER } else { Icon::SETTINGS };
            let f = font.clone();
            let ifont = icon_font.clone();
            let pad_left = indent + 8.0;
            bsn! {
                UNode {
                    padding: USides { top: 4.0, right: 8.0, bottom: 4.0, left: pad_left },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    UText {
                        text: icon,
                        font_size: 14.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: ifont,
                    }
                    --
                    UText {
                        text: label,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.19 Carousel Controls
// ============================================================================

/// Previous/Next navigation controls with pagination indicator dots.
#[derive(Clone, Debug, Reflect)]
pub struct CarouselControls {
    pub total: usize,
    pub current: usize,
    pub icon_font: Handle<Font>,
}

impl Default for CarouselControls {
    fn default() -> Self {
        Self {
            total: 4,
            current: 0,
            icon_font: Handle::default(),
        }
    }
}

impl CarouselControls {
    pub fn new(total: usize, current: usize) -> Self {
        Self {
            total,
            current,
            ..default()
        }
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for CarouselControls {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let current = self.current;
        let dots: Vec<_> = (0..self.total).map(|idx| {
            let is_active = idx == current;
            let (w, bg) = if is_active {
                (20.0_f32, Color::srgb(0.35, 0.65, 1.0))
            } else {
                (6.0_f32, Color::srgba(1.0, 1.0, 1.0, 0.3))
            };
            bsn! {
                UNode {
                    width: UVal::Px(w),
                    height: UVal::Px(6.0),
                    background_color: bg,
                    border_radius: UCornerRadius::all(3.0),
                }
            }
        }).collect();

        let dots_scene = SceneList::new(dots);
        let prev_ifont = self.icon_font.clone();
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                @{Button::icon_button(Icon::CHEVRON_LEFT).size(ButtonSize::Small).icon_font(prev_ifont)}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    {dots_scene}
                ]
                --
                @{Button::icon_button(Icon::CHEVRON_RIGHT).size(ButtonSize::Small).icon_font(self.icon_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.20 Table of Contents / Spy Scroll
// ============================================================================

/// Side index highlighting the current active document section.
#[derive(Clone, Debug, Reflect)]
pub struct TableOfContents {
    pub sections: Vec<String>,
    pub active_index: usize,
    pub font: Handle<Font>,
}

impl Default for TableOfContents {
    fn default() -> Self {
        Self {
            sections: vec![
                "Introduction".into(),
                "Architecture".into(),
                "Installation".into(),
                "Usage Examples".into(),
            ],
            active_index: 0,
            font: Handle::default(),
        }
    }
}

impl TableOfContents {
    pub fn new(sections: Vec<impl Into<String>>) -> Self {
        Self {
            sections: sections.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for TableOfContents {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active = self.active_index;
        let font = self.font;

        let rows: Vec<_> = self.sections.into_iter().enumerate().map(|(idx, sec)| {
            let is_active = idx == active;
            let (bar_color, text_color) = if is_active {
                (Color::srgb(0.35, 0.65, 1.0), Color::srgb(0.96, 0.97, 0.99))
            } else {
                (Color::NONE, Color::srgb(0.60, 0.65, 0.75))
            };
            let f = font.clone();
            bsn! {
                UNode {
                    padding: USides::axes(10.0, 4.0),
                }
                UBorder {
                    color: bar_color,
                    width: 2.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: sec,
                        font_size: 13.0,
                        color: text_color,
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            UNode {
                width: UVal::Px(200.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.21 Filter & Sort Bar
// ============================================================================

/// Composite toolbar for filtering, sorting, and view style toggling.
#[derive(Clone, Debug, Reflect)]
pub struct FilterSortBar {
    pub active_filter: String,
    pub sort_label: String,
    pub is_grid_view: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for FilterSortBar {
    fn default() -> Self {
        Self {
            active_filter: "All Items".into(),
            sort_label: "Newest First".into(),
            is_grid_view: true,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl FilterSortBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.active_filter = filter.into();
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort_label = sort.into();
        self
    }

    pub fn grid_view(mut self, grid: bool) -> Self {
        self.is_grid_view = grid;
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

impl Scene for FilterSortBar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let view_icon = if self.is_grid_view { Icon::FOLDER } else { Icon::MENU };
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(8.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                border_radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{Chip::new(self.active_filter).font(self.font.clone())}
                    --
                    @{Chip::new("Featured").font(self.font.clone())}
                    --
                    @{Chip::new("Archived").font(self.font.clone())}
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{Button::new(self.sort_label).outline().size(ButtonSize::Small).font(self.font)}
                    --
                    @{Button::icon_button(view_icon).size(ButtonSize::Small).icon_font(self.icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.22 Floating Toolbar
// ============================================================================

/// Contextual selection pill toolbar.
#[derive(Clone, Debug, Reflect)]
pub struct FloatingToolbar {
    pub icon_font: Handle<Font>,
}

impl Default for FloatingToolbar {
    fn default() -> Self {
        Self {
            icon_font: Handle::default(),
        }
    }
}

impl FloatingToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for FloatingToolbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let ifont1 = self.icon_font.clone();
        let ifont2 = self.icon_font.clone();
        let ifont3 = self.icon_font.clone();
        let ifont4 = self.icon_font;
        let s = bsn! {
            UNode {
                padding: USides::all(4.0),
                background_color: Color::srgba(0.1, 0.12, 0.18, 0.98),
                border_radius: UCornerRadius::all(20.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(20.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 4.0,
            }
            Children [
                @{Button::icon_button(Icon::STAR).ghost().size(ButtonSize::Small).icon_font(ifont1)}
                --
                @{Button::icon_button(Icon::LINK).ghost().size(ButtonSize::Small).icon_font(ifont2)}
                --
                @{Button::icon_button(Icon::CHECK).ghost().size(ButtonSize::Small).icon_font(ifont3)}
                --
                @{Button::icon_button(Icon::X).ghost().size(ButtonSize::Small).icon_font(ifont4)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.23 Workspace Switcher
// ============================================================================

/// Dropdown menu to switch workspaces or organizations.
#[derive(Clone, Debug, Reflect)]
pub struct WorkspaceSwitcher {
    pub current_workspace: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for WorkspaceSwitcher {
    fn default() -> Self {
        Self {
            current_workspace: "Acme Corp".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl WorkspaceSwitcher {
    pub fn new(workspace: impl Into<String>) -> Self {
        Self {
            current_workspace: workspace.into(),
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

impl Scene for WorkspaceSwitcher {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let initial = self.current_workspace.chars().next().unwrap_or('A').to_string();
        let ws_name = self.current_workspace;
        let font = self.font;
        let icon_font = self.icon_font;
        let s = bsn! {
            UNode {
                padding: USides { top: 6.0, right: 12.0, bottom: 6.0, left: 8.0 },
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                border_radius: UCornerRadius::all(6.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
                radius: UCornerRadius::all(6.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                @{Avatar::new(initial).size(AvatarSize::Small).font(font.clone())}
                --
                UText {
                    text: ws_name,
                    font_size: 13.0,
                    color: Color::srgb(0.92, 0.94, 0.98),
                    font,
                }
                --
                UText {
                    text: { Icon::CHEVRON_DOWN.to_string() },
                    font_size: 12.0,
                    color: Color::srgb(0.55, 0.6, 0.7),
                    font: icon_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.24 Language & Currency Selector
// ============================================================================

/// Language and currency switcher dropdown trigger.
#[derive(Clone, Debug, Reflect)]
pub struct LocaleSelector {
    pub current_locale: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for LocaleSelector {
    fn default() -> Self {
        Self {
            current_locale: "EN / USD".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl LocaleSelector {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            current_locale: locale.into(),
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

impl Scene for LocaleSelector {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let current_locale = self.current_locale;
        let ifont_folder = self.icon_font.clone();
        let font = self.font;
        let icon_font = self.icon_font;
        let s = bsn! {
            UNode {
                padding: USides::axes(10.0, 6.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.04),
                border_radius: UCornerRadius::all(6.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
                radius: UCornerRadius::all(6.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                UText {
                    text: { Icon::FOLDER.to_string() },
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.65, 0.75),
                    font: ifont_folder,
                }
                --
                UText {
                    text: current_locale,
                    font_size: 12.0,
                    color: Color::srgb(0.9, 0.92, 0.98),
                    font,
                }
                --
                UText {
                    text: { Icon::CHEVRON_DOWN.to_string() },
                    font_size: 12.0,
                    color: Color::srgb(0.55, 0.6, 0.7),
                    font: icon_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.25 Theme Toggle
// ============================================================================

/// Dark and Light mode switcher toggle button.
#[derive(Clone, Debug, Reflect)]
pub struct ThemeToggle {
    pub is_dark: bool,
    pub icon_font: Handle<Font>,
}

impl Default for ThemeToggle {
    fn default() -> Self {
        Self {
            is_dark: true,
            icon_font: Handle::default(),
        }
    }
}

impl ThemeToggle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for ThemeToggle {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let icon = if self.is_dark { Icon::MOON } else { Icon::SUN };
        let btn = Button::icon_button(icon)
            .ghost()
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}
