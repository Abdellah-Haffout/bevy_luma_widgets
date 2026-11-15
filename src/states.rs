//! State screens, user feedback placeholders, and overlay prompts.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::button::{Button, ButtonClicked, LumaButtonLabel};
use crate::helpers::{find_ancestor, visit_descendants, DescendantVisit};
use crate::navigation::SceneList;
use crate::prelude::*;

// ============================================================================
// 4.10 Empty State
// ============================================================================

/// Empty state placeholder with illustration icon, title, description, and CTA.
#[derive(Clone, Debug, Reflect)]
pub struct EmptyState {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub cta_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for EmptyState {
    fn default() -> Self {
        Self {
            title: "No Items Found".into(),
            description: "Your list is currently empty. Get started by creating your first item.".into(),
            icon: Icon::INBOX.to_string(),
            cta_label: "Create New".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl EmptyState {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            ..default()
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn cta(mut self, cta: impl Into<String>) -> Self {
        self.cta_label = cta.into();
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

impl Scene for EmptyState {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let description = self.description;
        let icon = self.icon;
        let cta_label = self.cta_label;
        let font = self.font;
        let icon_font = self.icon_font;
        let title_font = font.clone();
        let desc_font = font.clone();
        let btn_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(340.0),
                padding: USides::all(32.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
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
                gap: 16.0,
            }
            Children [
                UNode {
                    width: UVal::Px(56.0),
                    height: UVal::Px(56.0),
                    background_color: Color::srgba(0.35, 0.65, 1.0, 0.1),
                    border_radius: UCornerRadius::all(28.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: icon,
                        font_size: 24.0,
                        color: Color::srgb(0.35, 0.65, 1.0),
                        font: icon_font,
                    }
                ]
                --
                UText {
                    text: title,
                    font_size: 16.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: description,
                    font_size: 13.0,
                    color: Color::srgb(0.60, 0.65, 0.75),
                    font: desc_font,
                }
                --
                LumaStateAction { kind: StateActionKind::EmptyCta }
                UNode::default()
                Children [
                    @{Button::new(cta_label).size(ButtonSize::Medium).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.11 Error 404 State
// ============================================================================

/// 404 Page Not Found error presentation card.
#[derive(Clone, Debug, Reflect)]
pub struct Error404State {
    pub code: String,
    pub title: String,
    pub message: String,
    pub home_label: String,
    pub font: Handle<Font>,
}

impl Default for Error404State {
    fn default() -> Self {
        Self {
            code: "404".into(),
            title: "Page Not Found".into(),
            message: "The page or resource you are looking for does not exist or has been moved.".into(),
            home_label: "Return to Home".into(),
            font: Handle::default(),
        }
    }
}

impl Error404State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn home_label(mut self, label: impl Into<String>) -> Self {
        self.home_label = label.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Error404State {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let code = self.code;
        let title = self.title;
        let message = self.message;
        let home_label = self.home_label;
        let font = self.font;
        let code_font = font.clone();
        let title_font = font.clone();
        let msg_font = font.clone();
        let btn_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(360.0),
                padding: USides::all(36.0),
                background_color: Color::srgb(0.09, 0.10, 0.15),
                border_radius: UCornerRadius::all(14.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 16.0,
            }
            Children [
                UText {
                    text: code,
                    font_size: 48.0,
                    color: Color::srgb(0.94, 0.27, 0.27),
                    font: code_font,
                }
                --
                UText {
                    text: title,
                    font_size: 18.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: message,
                    font_size: 13.0,
                    color: Color::srgb(0.60, 0.65, 0.75),
                    font: msg_font,
                }
                --
                LumaStateAction { kind: StateActionKind::ErrorHome }
                UNode::default()
                Children [
                    @{Button::new(home_label).size(ButtonSize::Medium).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.12 Offline State
// ============================================================================

/// Connection lost offline warning banner.
#[derive(Clone, Debug, Reflect)]
pub struct OfflineState {
    pub title: String,
    pub description: String,
    pub retry_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for OfflineState {
    fn default() -> Self {
        Self {
            title: "You Are Currently Offline".into(),
            description: "Please check your network connection. Changes will sync once back online.".into(),
            retry_label: "Retry Connection".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl OfflineState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn retry_label(mut self, label: impl Into<String>) -> Self {
        self.retry_label = label.into();
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

impl Scene for OfflineState {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let description = self.description;
        let retry_label = self.retry_label;
        let font = self.font;
        let icon_font = self.icon_font;
        let title_font = font.clone();
        let desc_font = font.clone();
        let btn_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(340.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.94, 0.27, 0.27, 0.08),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgba(0.94, 0.27, 0.27, 0.3),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { Icon::CIRCLE_ALERT.to_string() },
                    font_size: 28.0,
                    color: Color::srgb(0.94, 0.27, 0.27),
                    font: icon_font,
                }
                --
                UText {
                    text: title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: description,
                    font_size: 12.0,
                    color: Color::srgb(0.70, 0.75, 0.85),
                    font: desc_font,
                }
                --
                LumaStateAction { kind: StateActionKind::OfflineRetry }
                UNode::default()
                Children [
                    @{Button::new(retry_label).danger().size(ButtonSize::Small).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.13 Success Screen / Receipt
// ============================================================================

/// Transaction confirmation and receipt celebration screen.
#[derive(Clone, Debug, Reflect)]
pub struct SuccessScreen {
    pub title: String,
    pub description: String,
    pub amount: String,
    pub done_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for SuccessScreen {
    fn default() -> Self {
        Self {
            title: "Payment Successful!".into(),
            description: "Your order #10842 has been placed and confirmed.".into(),
            amount: "$199.99".into(),
            done_label: "View Order Details".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl SuccessScreen {
    pub fn new(title: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            amount: amount.into(),
            ..default()
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn done_label(mut self, label: impl Into<String>) -> Self {
        self.done_label = label.into();
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

impl Scene for SuccessScreen {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let description = self.description;
        let amount = self.amount;
        let done_label = self.done_label;
        let font = self.font;
        let icon_font = self.icon_font;
        let title_font = font.clone();
        let desc_font = font.clone();
        let amount_font = font.clone();
        let btn_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(340.0),
                padding: USides::all(32.0),
                background_color: Color::srgb(0.09, 0.11, 0.16),
                border_radius: UCornerRadius::all(14.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.80, 0.44),
                width: 1.5,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 16.0,
            }
            Children [
                UNode {
                    width: UVal::Px(56.0),
                    height: UVal::Px(56.0),
                    background_color: Color::srgba(0.18, 0.80, 0.44, 0.15),
                    border_radius: UCornerRadius::all(28.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: { Icon::CHECK.to_string() },
                        font_size: 28.0,
                        color: Color::srgb(0.18, 0.80, 0.44),
                        font: icon_font,
                    }
                ]
                --
                UText {
                    text: title,
                    font_size: 18.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: amount,
                    font_size: 24.0,
                    color: Color::srgb(0.18, 0.80, 0.44),
                    font: amount_font,
                }
                --
                UText {
                    text: description,
                    font_size: 12.0,
                    color: Color::srgb(0.60, 0.65, 0.75),
                    font: desc_font,
                }
                --
                LumaStateAction { kind: StateActionKind::SuccessDone }
                UNode::default()
                Children [
                    @{Button::new(done_label).size(ButtonSize::Medium).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.14 Coachmark
// ============================================================================

/// Onboarding step spotlight coachmark tooltip.
#[derive(Clone, Debug, Reflect)]
pub struct Coachmark {
    pub step_text: String,
    pub title: String,
    pub description: String,
    pub next_label: String,
    pub skip_label: String,
    pub font: Handle<Font>,
}

impl Default for Coachmark {
    fn default() -> Self {
        Self {
            step_text: "Step 1 of 3".into(),
            title: "Command Palette".into(),
            description: "Press Ctrl + K anytime to open quick search and command actions.".into(),
            next_label: "Next".into(),
            skip_label: "Skip".into(),
            font: Handle::default(),
        }
    }
}

impl Coachmark {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            ..default()
        }
    }

    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.step_text = step.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Coachmark {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let step_text = self.step_text;
        let title = self.title;
        let description = self.description;
        let skip_label = self.skip_label;
        let next_label = self.next_label;
        let font = self.font;
        let step_font = font.clone();
        let title_font = font.clone();
        let desc_font = font.clone();
        let skip_font = font.clone();
        let next_font = font;

        let s = bsn! {
            LumaCoachmark
            UNode {
                width: UVal::Px(280.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.12, 0.14, 0.20),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.35, 0.65, 1.0),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
            }
            Children [
                LumaCoachmarkStepText
                UText {
                    text: step_text,
                    font_size: 11.0,
                    color: Color::srgb(0.35, 0.65, 1.0),
                    font: step_font,
                }
                --
                UText {
                    text: title,
                    font_size: 14.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: description,
                    font_size: 12.0,
                    color: Color::srgb(0.70, 0.75, 0.85),
                    font: desc_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaCoachmarkAction::Skip
                    UNode::default()
                    Children [
                        @{Button::new(skip_label).ghost().size(ButtonSize::Small).font(skip_font)}
                    ]
                    --
                    LumaCoachmarkAction::Next
                    UNode::default()
                    Children [
                        @{Button::new(next_label).size(ButtonSize::Small).font(next_font)}
                    ]
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.15 Cookie Consent Banner
// ============================================================================

/// Floating privacy and cookie compliance consent banner.
#[derive(Clone, Debug, Reflect)]
pub struct CookieConsent {
    pub message: String,
    pub accept_label: String,
    pub settings_label: String,
    pub font: Handle<Font>,
}

impl Default for CookieConsent {
    fn default() -> Self {
        Self {
            message: "We use cookies to analyze traffic and provide personalized user experiences.".into(),
            accept_label: "Accept All".into(),
            settings_label: "Preferences".into(),
            font: Handle::default(),
        }
    }
}

impl CookieConsent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for CookieConsent {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let message = self.message;
        let accept_label = self.accept_label;
        let settings_label = self.settings_label;
        let font = self.font;
        let msg_font = font.clone();
        let set_font = font.clone();
        let acc_font = font;

        let s = bsn! {
            LumaCookieConsent
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(24.0, 16.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 16.0,
            }
            Children [
                UText {
                    text: message,
                    font_size: 13.0,
                    color: Color::srgb(0.85, 0.88, 0.95),
                    font: msg_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaCookieConsentAction::Preferences
                    UNode::default()
                    Children [
                        @{Button::new(settings_label).outline().size(ButtonSize::Small).font(set_font)}
                    ]
                    --
                    LumaCookieConsentAction::Accept
                    UNode::default()
                    Children [
                        @{Button::new(accept_label).size(ButtonSize::Small).font(acc_font)}
                    ]
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 4.19 Paywall / Upgrade Prompt
// ============================================================================

/// Feature gate lock and upgrade promotion prompt card.
#[derive(Clone, Debug, Reflect)]
pub struct Paywall {
    pub title: String,
    pub subtitle: String,
    pub features: Vec<String>,
    pub cta_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Paywall {
    fn default() -> Self {
        Self {
            title: "Unlock Pro Features".into(),
            subtitle: "This capability requires a Pro subscription or active license.".into(),
            features: vec![
                "Full SDF UI Shader Pipelines".into(),
                "Realtime Vector Graphics Rasterizer".into(),
                "Priority Technical Support".into(),
            ],
            cta_label: "Upgrade to Pro".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Paywall {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn features(mut self, feats: Vec<impl Into<String>>) -> Self {
        self.features = feats.into_iter().map(|s| s.into()).collect();
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

impl Scene for Paywall {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let title = self.title;
        let subtitle = self.subtitle;
        let cta_label = self.cta_label;

        let feats: Vec<_> = self.features.into_iter().map(|f_str| {
            let f = font.clone();
            let ifont = icon_font.clone();
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    UText {
                        text: { Icon::CHECK.to_string() },
                        font_size: 13.0,
                        color: Color::srgb(0.35, 0.65, 1.0),
                        font: ifont,
                    }
                    --
                    UText {
                        text: f_str,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let feats_scene = SceneList::new(feats);
        let title_font = font.clone();
        let sub_font = font.clone();
        let btn_font = font;

        let s = bsn! {
            UNode {
                width: UVal::Px(320.0),
                padding: USides::all(28.0),
                background_color: Color::srgb(0.09, 0.11, 0.16),
                border_radius: UCornerRadius::all(14.0),
            }
            UBorder {
                color: Color::srgb(0.35, 0.65, 1.0),
                width: 1.5,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
            }
            Children [
                UText {
                    text: { Icon::LOCK.to_string() },
                    font_size: 32.0,
                    color: Color::srgb(0.35, 0.65, 1.0),
                    font: icon_font,
                }
                --
                UText {
                    text: title,
                    font_size: 18.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: subtitle,
                    font_size: 12.0,
                    color: Color::srgb(0.60, 0.65, 0.75),
                    font: sub_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    {feats_scene}
                ]
                --
                LumaStateAction { kind: StateActionKind::PaywallUpgrade }
                UNode::default()
                Children [
                    @{Button::new(cta_label).size(ButtonSize::Medium).font(btn_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// Runtime State Components, Events & Observers
// ============================================================================

/// Semantic action category emitted by state and onboarding screen CTAs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum StateActionKind {
    #[default]
    EmptyCta,
    ErrorHome,
    OfflineRetry,
    SuccessDone,
    CoachmarkSkip,
    CoachmarkNext,
    CookiePreferences,
    CookieAccept,
    PaywallUpgrade,
}

/// Marker identifying the CTA wrapper of a state screen.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStateAction {
    pub kind: StateActionKind,
}

/// Runtime root marker for [`Coachmark`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCoachmark;

/// Marker on [`Coachmark`]'s step indicator text node.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCoachmarkStepText;

/// Action button wrapper marker inside [`Coachmark`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum LumaCoachmarkAction {
    Skip,
    #[default]
    Next,
}

/// Runtime root marker for [`CookieConsent`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCookieConsent;

/// Action button wrapper marker inside [`CookieConsent`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum LumaCookieConsentAction {
    Preferences,
    #[default]
    Accept,
}

/// Event emitted when a state screen CTA button is activated.
#[derive(Event, Clone, Debug, Reflect)]
pub struct StateActionTriggered {
    pub entity: Entity,
    pub kind: StateActionKind,
    pub label: String,
}

/// Event emitted when a [`Coachmark`] advances to a step or completes.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct CoachmarkNext {
    pub entity: Entity,
    pub step: usize,
}

/// Event emitted when a [`Coachmark`] is skipped and hidden.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct CoachmarkSkipped {
    pub entity: Entity,
}

/// Event emitted when the user responds to a [`CookieConsent`] banner.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct CookieConsentResponded {
    pub entity: Entity,
    pub accepted: bool,
}

fn read_button_label(
    btn_entity: Entity,
    children_query: &Query<&Children>,
    label_marker: &Query<(), With<LumaButtonLabel>>,
    text_query: &Query<&mut UText>,
) -> String {
    let mut found = String::new();
    visit_descendants(btn_entity, children_query, |e| {
        if label_marker.contains(e) {
            if let Ok(t) = text_query.get(e) {
                found = t.text.clone();
                return DescendantVisit::Stop;
            }
        }
        DescendantVisit::Continue
    });
    found
}

fn parse_coachmark_step(text: &str) -> Option<(usize, usize)> {
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower.split_whitespace().collect();
    if tokens.len() >= 4 && tokens[0] == "step" && tokens[2] == "of" {
        let cur = tokens[1].parse::<usize>().ok()?;
        let tot = tokens[3].parse::<usize>().ok()?;
        return Some((cur, tot));
    }
    None
}

fn on_states_button_clicked(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    state_action_q: Query<&LumaStateAction>,
    cm_action_q: Query<&LumaCoachmarkAction>,
    cm_q: Query<(), With<LumaCoachmark>>,
    cookie_action_q: Query<&LumaCookieConsentAction>,
    cookie_q: Query<(), With<LumaCookieConsent>>,
    step_text_marker: Query<(), With<LumaCoachmarkStepText>>,
    btn_label_marker: Query<(), With<LumaButtonLabel>>,
    mut layout_query: Query<&mut ULayout>,
    mut text_query: Query<&mut UText>,
) {
    let btn = trigger.0;

    if let Some(action_ent) = find_ancestor(btn, &state_action_q, &parents_query) {
        if let Ok(action) = state_action_q.get(action_ent) {
            let label = read_button_label(btn, &children_query, &btn_label_marker, &text_query);
            commands.trigger(StateActionTriggered {
                entity: action_ent,
                kind: action.kind,
                label,
            });
        }
        return;
    }

    if let Some(cm_act_ent) = find_ancestor(btn, &cm_action_q, &parents_query) {
        let Ok(action) = cm_action_q.get(cm_act_ent).copied() else {
            return;
        };
        let Some(cm_ent) = find_ancestor(cm_act_ent, &cm_q, &parents_query) else {
            return;
        };
        let label = read_button_label(btn, &children_query, &btn_label_marker, &text_query);
        match action {
            LumaCoachmarkAction::Skip => {
                if let Ok(mut layout) = layout_query.get_mut(cm_ent) {
                    layout.display = UDisplay::None;
                }
                commands.trigger(CoachmarkSkipped { entity: cm_ent });
                commands.trigger(StateActionTriggered {
                    entity: cm_ent,
                    kind: StateActionKind::CoachmarkSkip,
                    label,
                });
            }
            LumaCoachmarkAction::Next => {
                let mut next_step_val = 1usize;
                let mut should_close = false;
                let mut reached_final = false;
                visit_descendants(cm_ent, &children_query, |e| {
                    if step_text_marker.contains(e) {
                        if let Ok(mut t) = text_query.get_mut(e) {
                            if let Some((cur, total)) = parse_coachmark_step(&t.text) {
                                if cur < total {
                                    next_step_val = cur + 1;
                                    t.text = format!("Step {next_step_val} of {total}");
                                    reached_final = next_step_val >= total;
                                } else {
                                    next_step_val = total;
                                    should_close = true;
                                }
                            } else {
                                should_close = true;
                            }
                        }
                        return DescendantVisit::Stop;
                    }
                    DescendantVisit::Continue
                });
                if reached_final {
                    visit_descendants(btn, &children_query, |e| {
                        if btn_label_marker.contains(e) {
                            if let Ok(mut t) = text_query.get_mut(e) {
                                t.text = "Done".to_string();
                            }
                            return DescendantVisit::Stop;
                        }
                        DescendantVisit::Continue
                    });
                }
                if should_close {
                    if let Ok(mut layout) = layout_query.get_mut(cm_ent) {
                        layout.display = UDisplay::None;
                    }
                }
                commands.trigger(CoachmarkNext {
                    entity: cm_ent,
                    step: next_step_val,
                });
                commands.trigger(StateActionTriggered {
                    entity: cm_ent,
                    kind: StateActionKind::CoachmarkNext,
                    label,
                });
            }
        }
        return;
    }

    if let Some(cookie_act_ent) = find_ancestor(btn, &cookie_action_q, &parents_query) {
        let Ok(action) = cookie_action_q.get(cookie_act_ent).copied() else {
            return;
        };
        let Some(cookie_ent) = find_ancestor(cookie_act_ent, &cookie_q, &parents_query) else {
            return;
        };
        let label = read_button_label(btn, &children_query, &btn_label_marker, &text_query);
        let accepted = action == LumaCookieConsentAction::Accept;
        if accepted {
            if let Ok(mut layout) = layout_query.get_mut(cookie_ent) {
                layout.display = UDisplay::None;
            }
        }
        commands.trigger(CookieConsentResponded {
            entity: cookie_ent,
            accepted,
        });
        commands.trigger(StateActionTriggered {
            entity: cookie_ent,
            kind: if accepted {
                StateActionKind::CookieAccept
            } else {
                StateActionKind::CookiePreferences
            },
            label,
        });
    }
}

/// Plugin enabling interactive CTA callbacks, coachmark stepping, and cookie banner dismissal.
pub struct LumaStatesPlugin;

impl Plugin for LumaStatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StateActionKind>()
            .register_type::<LumaStateAction>()
            .register_type::<LumaCoachmark>()
            .register_type::<LumaCoachmarkStepText>()
            .register_type::<LumaCoachmarkAction>()
            .register_type::<LumaCookieConsent>()
            .register_type::<LumaCookieConsentAction>()
            .register_type::<StateActionTriggered>()
            .register_type::<CoachmarkNext>()
            .register_type::<CoachmarkSkipped>()
            .register_type::<CookieConsentResponded>()
            .add_observer(on_states_button_clicked);
    }
}
