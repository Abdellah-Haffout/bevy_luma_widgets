//! Declarative UI macro for bevy_luma_widgets
//!
//! Provides a clean DSL for building UI with `luma_ui!` macro.
//! Focuses on structural composition; callbacks are added via Bevy observers.

// Imports are intentionally minimal - the macro expands to code that uses these
// but the macro itself doesn't need them directly.

/// Internal macro to build the scene using bevy_luma's bsn! macro
#[macro_export]
macro_rules! luma_ui {
    // Entry point - build a complete scene
    (@build_scene $($tt:tt)*) => {
        bsn! { $($tt)* }
    };

    // Internal: build a container node expression for use in Children arrays
    (@make_node
        $node:expr,
        $layout:expr,
        $(Children [ $($child:expr),* ])?
    ) => {
        (
            $node
            $layout
            $(Children [ $($child),* ])?
        )
    };

    // URootUi with shortcuts
    (URootUi $($rest:tt)*) => {
        luma_ui!(@build_scene URootUi::screen() $($rest)*)
    };

    // UNode with layout shortcuts
    (UNode { $($prop:ident: $val:expr),* $(,)? } $($rest:tt)*) => {
        luma_ui!(@build_scene
            UNode { $($prop: $val.into()),* }
            $($rest)*
        )
    };

    // ULayout with common shortcuts
    (ULayout { $($key:ident: $val:expr),* $(,)? } $($rest:tt)*) => {
        luma_ui!(@build_scene
            ULayout {
                $( $key: $val.into(), )*
                ..default()
            }
            $($rest)*
        )
    };

    // Children array
    (Children [ $($child:expr),* $(,)? ] ) => {
        luma_ui!(@build_scene Children [ $($child),* ])
    };

    // Row - horizontal flex container (expands to node expression for Children)
    (Row { $(gap: $gap:expr)? $(, justify: $justify:expr)? $(, align: $align:expr)? $(, children: [ $($child:expr),* $(,)? ] )? $(, $($rest:tt)*)? }) => {
        luma_ui!(@make_node
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: $($justify)?(UJustifyContent::Start),
                align_items: $($align)?(UAlignItems::Center),
                gap: $($gap)?(0.0),
                ..default()
            },
            $(Children [ $($child),* ])?
        )
    };

    // Col - vertical flex container (expands to node expression for Children)
    (Col { $(gap: $gap:expr)? $(, justify: $justify:expr)? $(, align: $align:expr)? $(, children: [ $($child:expr),* $(,)? ] )? $(, $($rest:tt)*)? }) => {
        luma_ui!(@make_node
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: $($justify)?(UJustifyContent::Start),
                align_items: $($align)?(UAlignItems::Start),
                gap: $($gap)?(0.0),
                ..default()
            },
            $(Children [ $($child),* ])?
        )
    };

    // Center - centered container (expands to node expression for Children)
    (Center { $(child: $child:expr)? $(, $($rest:tt)*)? }) => {
        luma_ui!(@make_node
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            $(Children [ $child ])?
        )
    };

    // Stack - overlay container (expands to node expression for Children)
    (Stack { $(children: [ $($child:expr),* $(,)? ] )? $(, $($rest:tt)*)? }) => {
        luma_ui!(@make_node
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                ..default()
            },
            $(Children [ $($child),* ])?
        )
    };

    // ===========================================
    // Widget Shortcuts (struct composition only)
    // ===========================================

    // Button
    (Button {
        label: $label:expr
        $(,)? $(variant: $variant:expr)?
        $(,)? $(size: $size:expr)?
        $(,)? $(disabled: $disabled:expr)?
        $(,)? $(icon: $icon:expr)?
        $(,)? $(font: $font:expr)?
        $(,)? $(icon_font: $icon_font:expr)?
    }) => {{
        let mut btn = Button::new($label)
            $(.variant($variant))?
            $(.size($size))?
            $(.disabled($disabled))?
            $(.icon($icon))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        btn
    }};

    // Toggle
    (Toggle {
        label: $label:expr
        $(,)? $(default: $def:expr)?
        $(,)? $(disabled: $disabled:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut t = Toggle::new($($def)?)
            .label($label)
            $(.disabled($disabled))?
            $(.font($font))?;
        t
    }};

    // Checkbox
    (Checkbox {
        label: $label:expr
        $(,)? $(default: $def:expr)?
        $(,)? $(disabled: $disabled:expr)?
        $(,)? $(font: $font:expr)?
        $(,)? $(icon_font: $icon_font:expr)?
    }) => {{
        let mut cb = Checkbox::new($label, $($def)?)
            $(.disabled($disabled))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        cb
    }};

    // Slider
    (Slider {
        $(label: $label:expr)?
        $(,)? $(range: $range:expr)?
        $(,)? $(default: $def:expr)?
        $(,)? $(step: $step:expr)?
        $(,)? $(width: $width:expr)?
        $(,)? $(disabled: $disabled:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut s = {
            // If range is provided, it should be a tuple (min, max)
            let (min, max) = $range;
            Slider::new($($def)?, min, max, $($width)?(200.0))
        }
            $(.step($step))?
            $(.disabled($disabled))?
            $(.font($font))?;
        s
    }};

    // Progress Bar
    (Progress {
        value: $value:expr
        $(,)? $(variant: $variant:expr)?
        $(,)? $(width: $width:expr)?
        $(,)? $(height: $height:expr)?
    }) => {{
        ProgressBar::new($value)
            $(.variant($variant))?
            $(.width($width.into()))?
            $(.height($height))?
    }};

    // Card - simplified (no title/children methods exist on Card)
    (Card {
        $(padding: $pad:expr)?
        $(,)? $(gap: $gap:expr)?
        $(,)? $(width: $width:expr)?
    }) => {{
        let mut card = Card::new()
            $(.padding($pad))?
            $(.gap($gap))?
            $(.width($width))?;
        card
    }};

    // Divider
    (Divider {
        $(horizontal: $horizontal:expr)?
        $(,)? $(label: $label:expr)?
        $(,)? $(font: $font:expr)?
        $(,)? $(vertical_height: $height:expr)?
    }) => {{
        let mut d = Divider::horizontal()
            $(.label($label))?
            $(.font($font))?;
        $(
            d = Divider::vertical($height);
        )?
        d
    }};

    // Input
    (Input {
        $(placeholder: $placeholder:expr)?
        $(,)? $(value: $value:expr)?
        $(,)? $(disabled: $disabled:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut i = Input::new()
            $(.placeholder($placeholder))?
            $(.value($value))?
            $(.disabled($disabled))?
            $(.font($font))?;
        i
    }};

    // Radio Group
    (RadioGroup { group: $group:expr $(, selected: $selected:expr)? $(, options: [ $($option:expr),* $(,)? ] )? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        let mut rg = RadioGroup::new($group)
            $(.selected($selected))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        $(
            rg = rg.options(vec![$($option),*]);
        )?
        rg
    }};

    // Radio Option (used within RadioGroup)
    (RadioOption { group: $group:expr, value: $value:expr, label: $label:expr $(, selected: $selected:expr)? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        RadioOption::new($group, $value, $label)
            $(.selected($selected))?
            $(.font($font))?
            $(.icon_font($icon_font))?
    }};

    // Tabs
    (Tabs { group: $group:expr $(, active: $active:expr)? $(, tabs: [ $($tab:expr),* $(,)? ] )? $(, font: $font:expr)? }) => {{
        let mut t = Tabs::new($group)
            $(.active($active))?
            $(.font($font))?;
        $(
            t = t.tabs(vec![$($tab),*]);
        )?
        t
    }};

    // Tab Item
    (Tab { label: $label:expr, value: $value:expr $(, font: $font:expr)? $(, icon: $icon:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        TabItem::new($label, $value)
            $(.font($font))?
            $(.icon($icon))?
            $(.icon_font($icon_font))?
    }};

    // Accordion
    (Accordion { $(items: [ $($item:expr),* $(,)? ] )? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        let mut a = Accordion::new()
            $(.font($font))?
            $(.icon_font($icon_font))?;
        $(
            a = a.items(vec![$($item),*]);
        )?
        a
    }};

    // Accordion Item
    (AccordionItem { title: $title:expr $(, expanded: $expanded:expr)? $(, children: [ $($child:expr),* $(,)? ] )? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        let mut ai = AccordionItem::new($title)
            $(.expanded($expanded))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        $(
            ai = ai.children(vec![$($child),*]);
        )?
        ai
    }};

    // Alert
    (Alert { title: $title:expr $(, message: $message:expr)? $(, variant: $variant:expr)? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        Alert::new($title)
            $(.message($message))?
            $(.variant($variant))?
            $(.font($font))?
            $(.icon_font($icon_font))?
    }};

    // Avatar
    (Avatar { name: $name:expr $(, size: $size:expr)? $(, status: $status:expr)? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        Avatar::new($name)
            $(.size($size))?
            $(.status($status))?
            $(.font($font))?
            $(.icon_font($icon_font))?
    }};

    // Skeleton
    (Skeleton { $(width: $width:expr)? $(, height: $height:expr)? $(, radius: $radius:expr)? }) => {{
        Skeleton::new()
            $(.width($width))?
            $(.height($height))?
            $(.radius($radius))?
    }};

    // Tooltip
    (Tooltip { content: $content:expr $(, font: $font:expr)? }) => {{
        Tooltip::new($content)
            $(.font($font))?
    }};

    // Stat Card
    (StatCard { title: $title:expr, value: $value:expr $(, trend: $trend:expr)? $(, description: $desc:expr)? $(, icon: $icon:expr)? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        StatCard::new($title, $value)
            $(.trend($trend))?
            $(.description($desc))?
            $(.icon($icon))?
            $(.font($font))?
            $(.icon_font($icon_font))?
    }};

    // Stepper
    (Stepper { value: $value:expr $(, min: $min:expr)? $(, max: $max:expr)? $(, step: $step:expr)? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        Stepper::new($value, $($min)?(0.0), $($max)?(100.0), $($step)?(1.0))
            $(.font($font))?
            $(.icon_font($icon_font))?
    }};

    // Rating
    (Rating { value: $value:expr $(, max: $max:expr)? $(, interactive: $interactive:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        Rating::new($value)
            $(.max_stars($max))?
            $(.interactive($interactive))?
            $(.icon_font($icon_font))?
    }};

    // Breadcrumb
    (Breadcrumb { $(items: [ $($item:expr),* $(,)? ] )? $(, font: $font:expr)? $(, icon_font: $icon_font:expr)? }) => {{
        let mut b = BreadcrumbTrail::new()
            $(.font($font))?
            $(.icon_font($icon_font))?;
        $(
            b = b.items(vec![$($item),*]);
        )?
        b
    }};

    // Breadcrumb Item
    (BreadcrumbItem { index: $index:expr, id: $id:expr, label: $label:expr $(, current: $current:expr)? $(, font: $font:expr)? }) => {{
        BreadcrumbItem::new($index, $id, $label)
            $(.current($current))?
            $(.font($font))?
    }};

    // Segmented Control
    (SegmentedControl {
        group: $group:expr
        $(,)? $(active: $active:expr)?
        $(,)? $(options: [ $($option:expr),* $(,)? ])?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut sc = SegmentedControl::new($group, $($active)?)
            $(.font($font))?;
        $(
            sc = sc.options(vec![$($option),*]);
        )?
        sc
    }};

    // Segment Option
    (SegmentOption {
        group: $group:expr
        $(,)? $(value: $value:expr)?
        $(,)? $(label: $label:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        SegmentOption::new($group, $($value)?, $($label)?)
            $(.font($font))?
    }};

    // Chip
    (Chip {
        label: $label:expr
        $(,)? $(icon: $icon:expr)?
        $(,)? $(removable: $removable:expr)?
        $(,)? $(font: $font:expr)?
        $(,)? $(icon_font: $icon_font:expr)?
    }) => {{
        let mut c = Chip::new($label)
            $(.icon($icon))?
            $(.removable($removable))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        c
    }};

    // Toast
    (Toast {
        title: $title:expr
        $(,)? $(message: $message:expr)?
        $(,)? $(variant: $variant:expr)?
        $(,)? $(duration: $duration:expr)?
        $(,)? $(font: $font:expr)?
        $(,)? $(icon_font: $icon_font:expr)?
    }) => {{
        let mut t = Toast::new($title, $($message)?)
            $(.variant($variant))?
            $(.duration($duration))?
            $(.font($font))?
            $(.icon_font($icon_font))?;
        t
    }};

    // Swatch
    (Swatch {
        color: $color:expr
        $(,)? $(label: $label:expr)?
        $(,)? $(selected: $selected:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut s = Swatch::new($color)
            $(.label($label))?
            $(.selected($selected))?
            $(.font($font))?;
        s
    }};

    // Kbd
    (Kbd {
        key: $key:expr
        $(,)? $(size: $size:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        Kbd::new($key)
            $(.size($size))?
            $(.font($font))?
    }};

    // Modal
    (Modal {
        title: $title:expr
        $(,)? $(message: $message:expr)?
        $(,)? $(confirm_text: $confirm:expr)?
        $(,)? $(cancel_text: $cancel:expr)?
        $(,)? $(font: $font:expr)?
    }) => {{
        let mut m = Modal::new($title, $($message)?)
            $(.confirm($confirm))?
            $(.cancel($cancel))?
            $(.font($font))?;
        m
    }};
}

// Re-export the macro
pub use luma_ui;

/// Helper macro for creating horizontal flex layouts (Row).
#[macro_export]
macro_rules! luma_row {
    // All combinations of optional justify/align
    (gap: $gap:expr, justify: $justify:expr, align: $align:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        let gap_val: f32 = $gap;
        bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: $justify,
                align_items: $align,
                gap: gap_val,
            }
            Children [ $($child),* ]
        }
    }};
    (gap: $gap:expr, align: $align:expr, justify: $justify:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_row!(gap: $gap, justify: $justify, align: $align, children: [ $($child),* ])
    }};
    (gap: $gap:expr, justify: $justify:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_row!(gap: $gap, justify: $justify, align: UAlignItems::Center, children: [ $($child),* ])
    }};
    (gap: $gap:expr, align: $align:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_row!(gap: $gap, justify: UJustifyContent::Start, align: $align, children: [ $($child),* ])
    }};
    (gap: $gap:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_row!(gap: $gap, justify: UJustifyContent::Start, align: UAlignItems::Center, children: [ $($child),* ])
    }};
    (children: [ $($child:expr),* $(,)? ] $(,)?) => {
        luma_row!(gap: 0.0, children: [ $($child),* ])
    };
}

/// Helper macro for creating vertical flex layouts (Column).
#[macro_export]
macro_rules! luma_col {
    // All combinations of optional justify/align
    (gap: $gap:expr, justify: $justify:expr, align: $align:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        let gap_val: f32 = $gap;
        bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: $justify,
                align_items: $align,
                gap: gap_val,
            }
            Children [ $($child),* ]
        }
    }};
    (gap: $gap:expr, align: $align:expr, justify: $justify:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_col!(gap: $gap, justify: $justify, align: $align, children: [ $($child),* ])
    }};
    (gap: $gap:expr, justify: $justify:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_col!(gap: $gap, justify: $justify, align: UAlignItems::Start, children: [ $($child),* ])
    }};
    (gap: $gap:expr, align: $align:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_col!(gap: $gap, justify: UJustifyContent::Start, align: $align, children: [ $($child),* ])
    }};
    (gap: $gap:expr, children: [ $($child:expr),* $(,)? ] $(,)?) => {{
        luma_col!(gap: $gap, justify: UJustifyContent::Start, align: UAlignItems::Start, children: [ $($child),* ])
    }};
    (children: [ $($child:expr),* $(,)? ] $(,)?) => {
        luma_col!(gap: 0.0, children: [ $($child),* ])
    };
}

/// Helper macro for creating centered layouts.
#[macro_export]
macro_rules! luma_center {
    (child: $child:expr $(,)?) => {
        bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [ $child ]
        }
    };
    (children: [ $($child:expr),* $(,)? ] $(,)?) => {
        bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [ $($child),* ]
        }
    };
}