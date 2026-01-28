//! Element Gallery - React Native Elements Example App replica
//!
//! Drawer-navigation showcase with 31 screens demonstrating every
//! component in the makepad-element library.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_element::components::text::*;
    use makepad_element::components::divider::*;
    use makepad_element::components::icon::*;
    use makepad_element::components::button::*;
    use makepad_element::components::image::*;
    use makepad_element::components::switch::*;
    use makepad_element::components::check_box::*;
    use makepad_element::components::slider::*;
    use makepad_element::components::input::*;
    use makepad_element::components::card::*;
    use makepad_element::components::avatar::*;
    use makepad_element::components::badge::*;
    use makepad_element::components::chip::*;
    use makepad_element::components::header::*;
    use makepad_element::components::list_item::*;
    use makepad_element::components::tab::*;
    use makepad_element::components::search_bar::*;
    use makepad_element::components::linear_progress::*;
    use makepad_element::components::rating::*;
    use makepad_element::components::fab::*;
    use makepad_element::components::tooltip::*;
    use makepad_element::components::overlay::*;
    use makepad_element::components::dialog::*;
    use makepad_element::components::tile::*;
    use makepad_element::components::pricing_card::*;
    use makepad_element::components::skeleton::*;
    use makepad_element::components::speed_dial::*;
    use makepad_element::components::social_icon::*;
    use makepad_element::components::markdown::*;
    use makepad_element::components::svg::*;
    use makepad_element::components::fade_view::*;
    use makepad_element::components::blur_view::*;
    use makepad_element::components::fading_bar::*;
    use makepad_element::components::staggered_grid::*;
    use makepad_element::theme::font::*;

    pub FONT_MANROPE = <ELEMENT_FONT_MANROPE> {}
    pub THEME_FONT_REGULAR = <FONT_MANROPE> {}

    // ── Color tokens ──────────────────────────────────────────────
    pub THEME_COLOR_LABEL_INNER = #333333
    pub THEME_COLOR_LABEL_INNER_HOVER = #111111
    pub THEME_COLOR_LABEL_INNER_ACTIVE = #397af8
    pub THEME_COLOR_TEXT = #333333

    // ── Reusable helpers ──────────────────────────────────────────

    SubHeader = <View> {
        width: Fill, height: 40,
        padding: {left: 16},
        align: {y: 0.5},
        show_bg: true,
        draw_bg: { color: #397af8 }
        label = <Label> {
            draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } }
            text: ""
        }
    }

    SectionTitle = <Label> {
        width: Fit, height: Fit,
        margin: {top: 8, bottom: 4},
        draw_text: { color: #8693a0, text_style: <THEME_FONT_REGULAR> { font_size: 13.0 } }
    }

    ScreenTitle = <Label> {
        width: Fit, height: Fit,
        margin: {bottom: 8},
        draw_text: { color: #333333, text_style: <THEME_FONT_REGULAR> { font_size: 22.0 } }
    }

    DrawerItem = <Button> {
        width: Fill, height: 48,
        padding: {left: 20},
        align: {y: 0.5},
        draw_bg: {
            fn pixel(self) -> vec4 {
                return mix(#ffffff, #f0f0f6, self.hover);
            }
        }
        draw_text: { color: #43484d, text_style: <THEME_FONT_REGULAR> { font_size: 15.0 } }
        text: ""
    }

    // ── Screen template ───────────────────────────────────────────
    // Each screen is a ScrollYView, visible: false by default

    // ══════════════════════════════════════════════════════════════
    //  APP
    // ══════════════════════════════════════════════════════════════

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            width: Fill, height: Fill,
            draw_bg: { fn pixel(self) -> vec4 { return #f5f5f5; } }

            body = <View> {
                width: Fill, height: Fill,
                flow: Overlay,

                // ── Content layer ─────────────────────────────
                content_layer = <View> {
                    width: Fill, height: Fill,
                    flow: Down,

                    // Header bar
                    header_bar = <View> {
                        width: Fill, height: 56,
                        flow: Right,
                        padding: {left: 16, right: 16},
                        align: {y: 0.5},
                        show_bg: true,
                        draw_bg: { color: #397af8 }

                        hamburger_btn = <Button> {
                            width: 40, height: 40,
                            draw_bg: { fn pixel(self) -> vec4 { return vec4(0.,0.,0.,0.); } }
                            draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 22.0 } }
                            text: "☰"
                        }

                        <View> { width: 8, height: 1 }

                        header_title = <Label> {
                            width: Fit, height: Fit,
                            draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 20.0 } }
                            text: "Element Gallery"
                        }

                        <View> { width: Fill, height: 1 }
                    }

                    // Page container
                    page_container = <View> {
                        width: Fill, height: Fill,
                        flow: Overlay,

                        // ── Welcome ───────────────────────────
                        welcome_page = <View> {
                            width: Fill, height: Fill,
                            align: {x: 0.5, y: 0.5},
                            flow: Down, spacing: 16,
                            <Image> {
                                width: 120, height: 120,
                                fit: Smallest,
                                source: dep("crate://self/resources/images/logo.png")
                            }
                            <Label> {
                                width: Fit, height: Fit,
                                draw_text: { color: #397af8, text_style: <THEME_FONT_REGULAR> { font_size: 32.0 } }
                                text: "Element Gallery"
                            }
                            <Label> {
                                width: Fit, height: Fit,
                                draw_text: { color: #8693a0, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } }
                                text: "Tap ☰ to browse components"
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 1: TEXT
                        // ══════════════════════════════════════
                        page_text = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Text" }
                            <SectionTitle> { text: "Headings" }

                            <ElementH1> { text: "Heading 1", draw_text: { color: #8F0CE8 } }
                            <ElementH2> { text: "Heading 2", draw_text: { color: #00B233 } }
                            <ElementH3> { text: "Heading 3", draw_text: { color: #FF9800 } }
                            <ElementH4> { text: "Heading 4", draw_text: { color: #397af8 } }

                            <SectionTitle> { text: "Body & Caption" }
                            <ElementBody> { text: "Body text — The quick brown fox jumps over the lazy dog." }
                            <ElementCaption> { text: "Caption text — secondary information" }
                            <ElementOverline> { text: "OVERLINE TEXT" }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 2: DIVIDER
                        // ══════════════════════════════════════
                        page_divider = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Divider" }

                            <SectionTitle> { text: "Horizontal" }
                            <ElementDivider> {}

                            <SectionTitle> { text: "Thick (primary color)" }
                            <View> {
                                width: Fill, height: 5,
                                show_bg: true,
                                draw_bg: { color: #397af8 }
                            }

                            <SectionTitle> { text: "Inset" }
                            <ElementDividerInset> {}

                            <SectionTitle> { text: "Vertical" }
                            <View> {
                                width: Fill, height: 60, flow: Right, align: {y: 0.5}, spacing: 16,
                                <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Left" }
                                <ElementDividerVertical> {}
                                <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Right" }
                            }

                            <SectionTitle> { text: "With Subheader" }
                            <SubHeader> { label = { text: "RECENT" } }
                            <ElementDivider> {}
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 3: CHIPS
                        // ══════════════════════════════════════
                        page_chips = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Chips" }

                            <SectionTitle> { text: "Solid" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementChip> { label = { text: "Primary" } }
                                <ElementChip> { label = { text: "Secondary" }, draw_bg: { color: #8F0CE8 } }
                                <ElementChip> { label = { text: "Success" }, draw_bg: { color: #00B233 } }
                            }

                            <SectionTitle> { text: "Outlined" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementChipOutline> { label = { text: "Outline" } }
                                <ElementChipOutline> { label = { text: "Info" } }
                            }

                            <SectionTitle> { text: "With Icons" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementChip> { label = { text: "Star ★" } }
                                <ElementChip> { label = { text: "Check ✓" }, draw_bg: { color: #00B233 } }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 4: BADGE
                        // ══════════════════════════════════════
                        page_badge = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Badge" }

                            <SectionTitle> { text: "Standard" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 24,
                                <ElementBadge> { label = { text: "3" }, draw_bg: { color: #00B233 } }
                                <ElementBadge> { label = { text: "99+" }, draw_bg: { color: #e53935 } }
                                <ElementBadge> { label = { text: "500" }, draw_bg: { color: #397af8 } }
                                <ElementBadge> { label = { text: "10" }, draw_bg: { color: #FF9800 } }
                            }

                            <SectionTitle> { text: "Mini (Status Dots)" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 24,
                                <View> {
                                    width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                    <ElementBadgeDot> { draw_bg: { color: #00B233 } }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "Online" }
                                }
                                <View> {
                                    width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                    <ElementBadgeDot> { draw_bg: { color: #e53935 } }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "Busy" }
                                }
                                <View> {
                                    width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                    <ElementBadgeDot> { draw_bg: { color: #397af8 } }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "Info" }
                                }
                                <View> {
                                    width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                    <ElementBadgeDot> { draw_bg: { color: #FF9800 } }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "Away" }
                                }
                            }

                            <SectionTitle> { text: "Badge on Avatar" }
                            <View> {
                                width: Fit, height: Fit, flow: Overlay,
                                <ElementAvatar> { text_view = { label = { text: "JD" }, draw_bg: { bg_color: #397af8 } } }
                                <View> {
                                    width: Fit, height: Fit,
                                    margin: {left: 28, top: 0},
                                    <ElementBadge> { label = { text: "3" }, draw_bg: { color: #e53935 } }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 5: FAB
                        // ══════════════════════════════════════
                        page_fab = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "FAB" }

                            <SectionTitle> { text: "Small" }
                            <ElementFabSmall> {}

                            <SectionTitle> { text: "Default (large, green)" }
                            <ElementFab> { draw_bg: { color: #00B233 } }

                            <SectionTitle> { text: "Extended with title" }
                            <ElementFabExtended> { text: "Navigate" }

                            <SectionTitle> { text: "Primary color" }
                            <ElementFab> { draw_bg: { color: #397af8 } }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 6: TOOLTIP
                        // ══════════════════════════════════════
                        page_tooltip = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tooltip" }

                            <SectionTitle> { text: "Default" }
                            <ElementTooltip> { label = { text: "Hello tooltip!" } }

                            <SectionTitle> { text: "Custom colors" }
                            <ElementTooltip> { label = { text: "Primary" }, draw_bg: { color: #397af8 } }
                            <ElementTooltip> { label = { text: "Secondary" }, draw_bg: { color: #8F0CE8 } }
                            <ElementTooltip> { label = { text: "Success" }, draw_bg: { color: #00B233 } }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 7: OVERLAY
                        // ══════════════════════════════════════
                        page_overlay = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Overlay" }
                            <Label> {
                                draw_text: { color: #8693a0, text_style: { font_size: 14.0 } }
                                text: "Tap the button below to trigger an overlay."
                            }

                            overlay_trigger_btn = <ElementButtonSolid> { text: "Show Overlay" }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 8: LINEAR PROGRESS
                        // ══════════════════════════════════════
                        page_linear_progress = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Linear Progress" }

                            <SectionTitle> { text: "Indeterminate" }
                            <ElementLinearProgressIndeterminate> { }

                            <SectionTitle> { text: "Indeterminate (red)" }
                            <ElementLinearProgressIndeterminate> { draw_bg: { fill_color: #e53935 } }

                            <SectionTitle> { text: "25%" }
                            <ElementLinearProgress> { draw_bg: { progress: 0.25 } }

                            <SectionTitle> { text: "50%" }
                            <ElementLinearProgress> { draw_bg: { progress: 0.50 } }

                            <SectionTitle> { text: "75%" }
                            <ElementLinearProgress> { draw_bg: { progress: 0.75 } }

                            <SectionTitle> { text: "100%" }
                            <ElementLinearProgress> { draw_bg: { progress: 1.0 } }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 9: SKELETON
                        // ══════════════════════════════════════
                        page_skeleton = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Skeleton" }

                            <SectionTitle> { text: "Circle + Lines" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSkeletonCircle> { width: 40, height: 40 }
                                <View> {
                                    width: Fill, height: Fit, flow: Down, spacing: 8,
                                    <ElementSkeletonLine> { width: Fill, height: 14 }
                                    <ElementSkeletonLine> { width: 200, height: 14 }
                                }
                            }

                            <SectionTitle> { text: "Rectangle" }
                            <ElementSkeletonRect> { width: Fill, height: 120 }

                            <SectionTitle> { text: "Card-like" }
                            <View> {
                                width: Fill, height: Fit, flow: Down, spacing: 8,
                                <ElementSkeletonRect> { width: Fill, height: 180 }
                                <ElementSkeletonLine> { width: Fill, height: 16 }
                                <ElementSkeletonLine> { width: 240, height: 16 }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 10: SOCIAL ICONS
                        // ══════════════════════════════════════
                        page_social_icons = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Social Icons" }

                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "fb", draw_bg: { color: #3b5998 } }
                                <ElementSocialIcon> { text: "tw", draw_bg: { color: #00aced } }
                                <ElementSocialIcon> { text: "gg", draw_bg: { color: #dd4b39 } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "pi", draw_bg: { color: #cb2027 } }
                                <ElementSocialIcon> { text: "in", draw_bg: { color: #007bb6 } }
                                <ElementSocialIcon> { text: "yt", draw_bg: { color: #bb0000 } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "ig", draw_bg: { color: #517fa4 } }
                                <ElementSocialIcon> { text: "gh", draw_bg: { color: #333333 } }
                                <ElementSocialIcon> { text: "dr", draw_bg: { color: #ea4c89 } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "be", draw_bg: { color: #0540ff } }
                                <ElementSocialIcon> { text: "wp", draw_bg: { color: #21759b } }
                                <ElementSocialIcon> { text: "vc", draw_bg: { color: #1ab7ea } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "sl", draw_bg: { color: #e01563 } }
                                <ElementSocialIcon> { text: "ms", draw_bg: { color: #7b0099 } }
                                <ElementSocialIcon> { text: "rd", draw_bg: { color: #ff4500 } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "tk", draw_bg: { color: #69c9d0 } }
                                <ElementSocialIcon> { text: "dc", draw_bg: { color: #7289da } }
                                <ElementSocialIcon> { text: "tg", draw_bg: { color: #0088cc } }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 11: SPEED DIAL
                        // ══════════════════════════════════════
                        page_speed_dial = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Speed Dial" }

                            <SectionTitle> { text: "Right placement" }
                            <ElementSpeedDial> {}

                            <SectionTitle> { text: "With actions" }
                            <ElementSpeedDialAction> {}
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 12: BUTTONS
                        // ══════════════════════════════════════
                        page_buttons = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Buttons" }

                            <SectionTitle> { text: "Solid" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonSolid> { text: "Primary" }
                                <ElementButtonSolid> { text: "Secondary", draw_bg: { color: #8F0CE8 } }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonSolid> { text: "Warning", draw_bg: { color: #FF9800 } }
                                <ElementButtonSolid> { text: "Error", draw_bg: { color: #e53935 } }
                            }

                            <SectionTitle> { text: "Outline" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonOutline> { text: "Outline" }
                                <ElementButtonOutline> { text: "Success", draw_text: { color: #00B233 } }
                            }

                            <SectionTitle> { text: "Clear" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonClear> { text: "Clear" }
                            }

                            <SectionTitle> { text: "Rounded" }
                            <ElementButtonSolid> {
                                text: "Rounded",
                                draw_bg: { border_radius: 30.0 }
                            }

                            <SectionTitle> { text: "Success" }
                            <ElementButtonSolid> { text: "Success", draw_bg: { color: #00B233 } }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 13: CHECKBOX
                        // ══════════════════════════════════════
                        page_checkbox = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "CheckBox" }

                            <SectionTitle> { text: "Standard" }
                            <ElementCheckBox> { text: "Click Here" }
                            <ElementCheckBox> { text: "Checked", animator: { selected = { default: on } } }

                            <SectionTitle> { text: "Switch" }
                            <ElementSwitch> {}
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 14: INPUTS
                        // ══════════════════════════════════════
                        page_inputs = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Inputs" }

                            <SectionTitle> { text: "Search Bars" }
                            <ElementSearchBar> { }

                            <SectionTitle> { text: "Standard Input" }
                            <ElementInput> { text: "", empty_text: "Enter your name" }

                            <SectionTitle> { text: "Labeled Input" }
                            <Label> {
                                draw_text: { color: #8693a0, text_style: { font_size: 13.0 } }
                                text: "EMAIL"
                            }
                            <ElementInput> { text: "", empty_text: "user@example.com" }

                            <SectionTitle> { text: "Password" }
                            <ElementInput> { text: "", empty_text: "Password" }

                            <SectionTitle> { text: "Error State" }
                            <ElementInput> { text: "invalid", empty_text: "" }
                            <Label> {
                                draw_text: { color: #e53935, text_style: { font_size: 12.0 } }
                                text: "Please enter a valid email address"
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 15: SLIDER
                        // ══════════════════════════════════════
                        page_slider = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Slider" }

                            <SectionTitle> { text: "Default (0–100)" }
                            <ElementSlider> { text: "Value", min: 0.0, max: 100.0 }

                            <SectionTitle> { text: "Stepped (step: 10)" }
                            <ElementSlider> { text: "Stepped", min: 0.0, max: 100.0, step: 10.0 }

                            <SectionTitle> { text: "Round style" }
                            <ElementSliderRound> { text: "Round", min: 0.0, max: 100.0 }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 16: RATINGS
                        // ══════════════════════════════════════
                        page_ratings = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Ratings" }

                            <SectionTitle> { text: "Star Ratings" }
                            <ElementRating0> {}
                            <ElementRating1> {}
                            <ElementRating2> {}
                            <ElementRating3> {}
                            <ElementRating4> {}
                            <ElementRating5> {}
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 17: AVATARS
                        // ══════════════════════════════════════
                        page_avatars = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Avatars" }

                            <SectionTitle> { text: "Letter Avatars" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 12,
                                <ElementAvatar> { text_view = { label = { text: "Fc" }, draw_bg: { bg_color: #3d4db7 } } }
                                <ElementAvatar> { text_view = { label = { text: "P" }, draw_bg: { bg_color: #FF6F61 } } }
                                <ElementAvatar> { text_view = { label = { text: "Rd" }, draw_bg: { bg_color: #9C27B0 } } }
                            }

                            <SectionTitle> { text: "Icon Avatars" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 12,
                                <ElementAvatar> { text_view = { label = { text: "✎" }, draw_bg: { bg_color: #6733b9 } } }
                                <ElementAvatar> { text_view = { label = { text: "♥" }, draw_bg: { bg_color: #eb1561 } } }
                                <ElementAvatar> { text_view = { label = { text: "★" }, draw_bg: { bg_color: #00a7f7 } } }
                            }

                            <SectionTitle> { text: "Different sizes" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 1.0},
                                <ElementAvatarSmall> { text_view = { label = { text: "S" } } }
                                <ElementAvatar> { text_view = { label = { text: "M" } } }
                                <ElementAvatarLarge> { text_view = { label = { text: "L" } } }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 18: CARDS
                        // ══════════════════════════════════════
                        page_cards = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Cards" }

                            <SectionTitle> { text: "Basic Card" }
                            <ElementCard> {
                                <View> {
                                    width: Fill, height: Fit, flow: Down, spacing: 8, padding: 16,
                                    <Label> {
                                        draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 18.0 } }
                                        text: "Card Title"
                                    }
                                    <Label> {
                                        width: Fill,
                                        draw_text: { color: #8693a0, text_style: { font_size: 14.0 }, wrap: Word }
                                        text: "The idea with React Native Elements is more about component structure than actual design."
                                    }
                                }
                            }

                            <SectionTitle> { text: "Card with font showcase" }
                            <ElementCard> {
                                <View> {
                                    width: Fill, height: Fit, flow: Down, spacing: 8, padding: 16,
                                    <ElementH1> { text: "h1 Heading" }
                                    <ElementH2> { text: "h2 Heading" }
                                    <ElementH3> { text: "h3 Heading" }
                                    <ElementH4> { text: "h4 Heading" }
                                    <ElementBody> { text: "Normal Text" }
                                }
                            }

                            <SectionTitle> { text: "Featured Card" }
                            <ElementCard> {
                                <Image> {
                                    width: Fill, height: 160,
                                    fit: Smallest,
                                    source: dep("crate://self/resources/images/wallpaper_2.jpg")
                                }
                                <View> {
                                    width: Fill, height: Fit, flow: Down, spacing: 8, padding: 16,
                                    <Label> {
                                        draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 18.0 } }
                                        text: "FLAVOR OF THE MONTH"
                                    }
                                    <Label> {
                                        width: Fill,
                                        draw_text: { color: #8693a0, text_style: { font_size: 14.0 }, wrap: Word }
                                        text: "Cras ut mauris et dolor porta gravida. Suspendisse quis nibh ullamcorper ante."
                                    }
                                    <ElementButtonSolid> { text: "VIEW NOW", draw_bg: { border_radius: 20.0 } }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 19: DIALOGS
                        // ══════════════════════════════════════
                        page_dialogs = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Dialogs" }

                            dialog_simple_btn = <ElementButtonSolid> { text: "Simple Dialog" }
                            dialog_multi_btn = <ElementButtonOutline> { text: "Multi-Action Dialog" }
                            dialog_loading_btn = <ElementButtonOutline> { text: "Loading Dialog" }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 20: PRICING
                        // ══════════════════════════════════════
                        page_pricing = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Pricing" }

                            <ElementPricingCard> {
                                header = { plan_name = { text: "Free", draw_text: { color: #397af8 } } price = { text: "$0" } }
                                features = {
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "1 User" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Basic Support" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "All Core Features" }
                                }
                                cta = { <ElementButtonSolid> { text: "GET STARTED" } }
                            }
                            <ElementPricingCard> {
                                header = { plan_name = { text: "Starter", draw_text: { color: #8F0CE8 } } price = { text: "$19" } }
                                features = {
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "10 Users" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Basic Support" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "All Core Features" }
                                }
                                cta = { <ElementButtonSolid> { text: "GET STARTED", draw_bg: { color: #8F0CE8 } } }
                            }
                            <ElementPricingCard> {
                                header = { plan_name = { text: "Enterprise", draw_text: { color: #00B233 } } price = { text: "$49" } }
                                features = {
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "100 Users" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "One on One Support" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "All Core Features" }
                                }
                                cta = { <ElementButtonSolid> { text: "GET STARTED", draw_bg: { color: #00B233 } } }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 21: TILES
                        // ══════════════════════════════════════
                        page_tiles = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tiles" }

                            <ElementTile> {
                                overlay = {
                                    title = { text: "A beautiful sunset" }
                                    subtitle = { text: "— Mahatma Gandhi" }
                                }
                            }

                            <ElementTile> {
                                overlay = {
                                    title = { text: "Adventure awaits" }
                                    subtitle = { text: "Explore the world" }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 22: IMAGE
                        // ══════════════════════════════════════
                        page_image = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Image" }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/wallpaper_1.jpg") }
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/wallpaper_2.jpg") }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/wallpaper_3.jpg") }
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/wallpaper_4.jpg") }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/bg_screen1.jpg") }
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/bg_screen2.jpg") }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/bg_screen3.jpg") }
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/bg_screen4.jpg") }
                            }
                            <View> {
                                width: Fill, height: Fit, flow: Right, spacing: 8,
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/water.png") }
                                <Image> { width: Fill, height: 150, fit: Smallest, source: dep("crate://self/resources/images/shirt.png") }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 23: TABS
                        // ══════════════════════════════════════
                        page_tabs = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tabs" }

                            <ElementTabBar> {
                                <ElementTabItem> { text: "Home" }
                                <ElementTabItem> { text: "Search" }
                                <ElementTabItem> { text: "Cart" }
                                <ElementTabItem> { text: "Account" }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 24: LISTS
                        // ══════════════════════════════════════
                        page_lists = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 8,

                            <ScreenTitle> { text: "Lists" }

                            // Profile card
                            <RoundedView> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 8, padding: 20,
                                align: {x: 0.5},
                                show_bg: true,
                                draw_bg: { color: #fff, border_radius: 12.0 }

                                <ElementAvatarLarge> { text_view = { label = { text: "PA" }, draw_bg: { bg_color: #8F0CE8 } } }

                                <Label> {
                                    draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 22.0 } }
                                    text: "Paul Allen"
                                }

                                <View> {
                                    width: Fit, height: Fit, flow: Right, spacing: 12,
                                    <ElementButtonOutline> { text: "View Profile" }
                                    <ElementButtonSolid> { text: "Add User", draw_bg: { color: #00B233 } }
                                }
                            }

                            <SectionTitle> { text: "User List" }

                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16, top: 12, bottom: 12},
                                    <ElementAvatarSmall> { text_view = { label = { text: "AF" }, draw_bg: { bg_color: #4A90D9 } } }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 15.0 } } text: "Amy Farha" }
                                    <ElementBadge> { label = { text: "+18" }, draw_bg: { color: #00B233 } }
                                }
                            }
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16, top: 12, bottom: 12},
                                    <ElementAvatarSmall> { text_view = { label = { text: "CJ" }, draw_bg: { bg_color: #FF6F61 } } }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 15.0 } } text: "Chris Jackson" }
                                    <ElementBadge> { label = { text: "-3" }, draw_bg: { color: #e53935 } }
                                }
                            }
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16, top: 12, bottom: 12},
                                    <ElementAvatarSmall> { text_view = { label = { text: "AS" }, draw_bg: { bg_color: #9C27B0 } } }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 15.0 } } text: "Amanda Smith" }
                                    <ElementBadge> { label = { text: "+22" }, draw_bg: { color: #00B233 } }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 25: SETTINGS
                        // ══════════════════════════════════════
                        page_settings = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 0, spacing: 0,

                            <View> { width: Fill, height: 16 }

                            <ElementSearchBar> {}

                            <SubHeader> { label = { text: "NETWORK" } }

                            // Airplane Mode
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #FF9800, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "✈" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Airplane Mode" }
                                    <ElementSwitch> {}
                                }
                            }
                            <ElementDividerInset> {}

                            // Wi-Fi
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #2196f3, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "◉" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Wi-Fi" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "Home ›" }
                                }
                            }
                            <ElementDividerInset> {}

                            // Bluetooth
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #2196f3, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "ᛒ" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Bluetooth" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "On ›" }
                                }
                            }
                            <ElementDividerInset> {}

                            // Cellular
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #4caf50, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "⦿" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Cellular" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }

                            <SubHeader> { label = { text: "NOTIFICATIONS" } }

                            // Notifications
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #e53935, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "🔔" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Notifications" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }
                            <ElementDividerInset> {}

                            // Do Not Disturb
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #9C27B0, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "🌙" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Do Not Disturb" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }

                            <SubHeader> { label = { text: "GENERAL" } }

                            // General
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #8693a0, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "⚙" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "General" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }
                            <ElementDividerInset> {}

                            // Display
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #2196f3, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "A" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Display & Brightness" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }
                            <ElementDividerInset> {}

                            // Battery
                            <ElementListItem> {
                                <View> {
                                    width: Fill, height: 48, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                    <RoundedView> {
                                        width: 28, height: 28,
                                        show_bg: true, draw_bg: { color: #4caf50, border_radius: 6.0 }
                                        align: {x: 0.5, y: 0.5},
                                        <Label> { draw_text: { color: #fff, text_style: { font_size: 14.0 } } text: "🔋" }
                                    }
                                    <Label> { width: Fill, draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Battery" }
                                    <Label> { draw_text: { color: #8693a0, text_style: { font_size: 14.0 } } text: "›" }
                                }
                            }

                            <View> { width: Fill, height: 20 }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 26: THEME
                        // ══════════════════════════════════════
                        page_theme = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 12,

                            <ScreenTitle> { text: "Theme" }
                            <Label> {
                                width: Fill,
                                draw_text: { color: #8693a0, text_style: { font_size: 14.0 }, wrap: Word }
                                text: "Choose a color below to update the primary theme color."
                            }

                            theme_color_1 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #2196f3 }, cursor: Hand }
                            theme_color_2 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #e91e63 }, cursor: Hand }
                            theme_color_3 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #3d5aff }, cursor: Hand }
                            theme_color_4 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #4615b2 }, cursor: Hand }
                            theme_color_5 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #ffd600 }, cursor: Hand }
                            theme_color_6 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #ff5722 }, cursor: Hand }
                            theme_color_7 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: #00a152 }, cursor: Hand }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 27: FONTS
                        // ══════════════════════════════════════
                        page_fonts = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 8,

                            <ScreenTitle> { text: "Fonts" }
                            <Label> {
                                draw_text: { color: #8693a0, text_style: { font_size: 14.0 } }
                                text: "System fonts displayed in default typeface"
                            }

                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "San Francisco" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Helvetica Neue" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Arial" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Georgia" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Courier New" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Times New Roman" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Verdana" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Trebuchet MS" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Futura" }
                            <ElementDivider> {}
                            <Label> { draw_text: { color: #333, text_style: { font_size: 18.0 } } text: "Menlo" }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 28: BOTTOM SHEET
                        // ══════════════════════════════════════
                        page_bottom_sheet = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Bottom Sheet" }
                            <Label> {
                                draw_text: { color: #8693a0, text_style: { font_size: 14.0 } }
                                text: "Tap the button to open a bottom sheet."
                            }

                            bottom_sheet_btn = <Button> {
                                width: Fit, height: 44,
                                padding: {left: 20, right: 20},
                                draw_bg: { color: #397af8, border_radius: 4.0 }
                                draw_text: { color: #fff, text_style: <THEME_FONT_REGULAR> { font_size: 15.0 } }
                                text: "Open Bottom Sheet"
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 29: LOGIN
                        // ══════════════════════════════════════
                        page_login = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 0, spacing: 0,

                            // Background image
                            <Image> {
                                width: Fill, height: 200,
                                fit: Smallest,
                                source: dep("crate://self/resources/images/bg_screen4.jpg")
                            }

                            // Login form
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 16,
                                padding: {left: 20, right: 20, top: 40, bottom: 40},
                                show_bg: true,
                                draw_bg: { color: #2F343B }

                                <Label> {
                                    width: Fill,
                                    draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 28.0 } }
                                    text: "Login"
                                }
                                <Label> {
                                    width: Fill,
                                    draw_text: { color: #ffffff88, text_style: { font_size: 14.0 } }
                                    text: "Please sign in to continue."
                                }

                                <ElementInput> { text: "", empty_text: "Email" }
                                <ElementInput> { text: "", empty_text: "Password" }

                                <ElementButtonSolid> {
                                    text: "SIGN IN",
                                    draw_bg: { border_radius: 30.0 }
                                }
                            }

                            // Signup form
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down, spacing: 16,
                                padding: {left: 20, right: 20, top: 40, bottom: 40},
                                show_bg: true,
                                draw_bg: { color: #2E3248 }

                                <Label> {
                                    width: Fill,
                                    draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 28.0 } }
                                    text: "Sign Up"
                                }

                                <ElementInput> { text: "", empty_text: "Full Name" }
                                <ElementInput> { text: "", empty_text: "Email" }
                                <ElementInput> { text: "", empty_text: "Password" }
                                <ElementInput> { text: "", empty_text: "Confirm Password" }

                                <ElementButtonSolid> {
                                    text: "SIGN UP",
                                    draw_bg: { border_radius: 30.0, color: #8F0CE8 }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 30: WHATSAPP
                        // ══════════════════════════════════════
                        page_whatsapp = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 0, spacing: 0,

                            // WhatsApp header
                            <View> {
                                width: Fill, height: 56,
                                flow: Right, padding: {left: 16, right: 16}, align: {y: 0.5},
                                show_bg: true, draw_bg: { color: #075E54 }

                                <Label> {
                                    width: Fill,
                                    draw_text: { color: #fff, text_style: <THEME_FONT_REGULAR> { font_size: 20.0 } }
                                    text: "WhatsApp"
                                }
                                <Label> { draw_text: { color: #fff, text_style: { font_size: 18.0 } } text: "🔍" }
                                <View> { width: 16, height: 1 }
                                <Label> { draw_text: { color: #fff, text_style: { font_size: 18.0 } } text: "⋮" }
                            }

                            // Tab bar
                            <View> {
                                width: Fill, height: 44,
                                flow: Right, show_bg: true, draw_bg: { color: #075E54 },

                                <View> {
                                    width: 44, height: Fill, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #ffffffaa, text_style: { font_size: 18.0 } } text: "📷" }
                                }

                                <View> {
                                    width: Fill, height: Fill, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #fff, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } } text: "CHATS" }
                                }
                                <View> {
                                    width: Fill, height: Fill, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #ffffffaa, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } } text: "STATUS" }
                                }
                                <View> {
                                    width: Fill, height: Fill, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #ffffffaa, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } } text: "CALLS" }
                                }
                            }

                            // Active tab indicator
                            <View> {
                                width: Fill, height: 3,
                                flow: Right,
                                <View> { width: 44, height: Fill }
                                <View> { width: Fill, height: Fill, show_bg: true, draw_bg: { color: #ffffff } }
                                <View> { width: Fill, height: Fill }
                                <View> { width: Fill, height: Fill }
                            }

                            // Chat list
                            <View> {
                                width: Fill, height: Fit, flow: Down,

                                <ElementListItem> {
                                    <View> {
                                        width: Fill, height: 72, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                        <ElementAvatar> { text_view = { label = { text: "JD" }, draw_bg: { bg_color: #4A90D9 } } }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 4,
                                            <Label> { draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "John Doe" }
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "Hey, how are you?" }
                                        }
                                        <View> {
                                            width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 1.0},
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "12:30 PM" }
                                            <ElementBadge> { label = { text: "3" }, draw_bg: { color: #25D366 } }
                                        }
                                    }
                                }

                                <ElementListItem> {
                                    <View> {
                                        width: Fill, height: 72, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                        <ElementAvatar> { text_view = { label = { text: "JS" }, draw_bg: { bg_color: #FF6F61 } } }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 4,
                                            <Label> { draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Jane Smith" }
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "See you tomorrow!" }
                                        }
                                        <View> {
                                            width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 1.0},
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "11:45 AM" }
                                            <ElementBadge> { label = { text: "5" }, draw_bg: { color: #25D366 } }
                                        }
                                    }
                                }

                                <ElementListItem> {
                                    <View> {
                                        width: Fill, height: 72, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                        <ElementAvatar> { text_view = { label = { text: "BW" }, draw_bg: { bg_color: #9C27B0 } } }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 4,
                                            <Label> { draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Bob Wilson" }
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "That sounds great!" }
                                        }
                                        <View> {
                                            width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 1.0},
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "10:20 AM" }
                                            <ElementBadge> { label = { text: "7" }, draw_bg: { color: #25D366 } }
                                        }
                                    }
                                }

                                <ElementListItem> {
                                    <View> {
                                        width: Fill, height: 72, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                        <ElementAvatar> { text_view = { label = { text: "AL" }, draw_bg: { bg_color: #00B233 } } }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 4,
                                            <Label> { draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Alice Lee" }
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "Call me when free" }
                                        }
                                        <View> {
                                            width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 1.0},
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "9:15 AM" }
                                            <ElementBadge> { label = { text: "9" }, draw_bg: { color: #25D366 } }
                                        }
                                    }
                                }

                                <ElementListItem> {
                                    <View> {
                                        width: Fill, height: 72, flow: Right, spacing: 12, align: {y: 0.5}, padding: {left: 16, right: 16},
                                        <ElementAvatar> { text_view = { label = { text: "MK" }, draw_bg: { bg_color: #FF9800 } } }
                                        <View> {
                                            width: Fill, height: Fit, flow: Down, spacing: 4,
                                            <Label> { draw_text: { color: #333, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Mike Kim" }
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "Photo attached" }
                                        }
                                        <View> {
                                            width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 1.0},
                                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 11.0 } } text: "Yesterday" }
                                            <ElementBadge> { label = { text: "2" }, draw_bg: { color: #25D366 } }
                                        }
                                    }
                                }
                            }

                            // WhatsApp FAB
                            <View> {
                                width: Fill, height: 80,
                                align: {x: 1.0, y: 1.0},
                                padding: {right: 16, bottom: 16},
                                <ElementFab> { draw_bg: { color: #25D366 } }
                            }
                        }

                        // ══════════════════════════════════════
                        //  SCREEN 31: MARKDOWN
                        // ══════════════════════════════════════
                        page_markdown = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Markdown" }

                            <ElementMarkdown> {
                                body: "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\nThis is **bold text** and *italic text* and ***bold italic***.\n\n- Item one\n- Item two\n- Item three\n\n> A blockquote with some wisdom.\n\nInline `code` and a code block:\n\n```\nfn main() {\n    println!(\"Hello Element Gallery!\");\n}\n```\n\n---\n\nA [link](https://makepad.dev) to Makepad."
                            }
                        }

                    } // end page_container
                } // end content_layer

                // ── Drawer overlay ────────────────────────────
                drawer_overlay = <View> {
                    visible: false,
                    width: Fill, height: Fill,
                    flow: Right,

                    // Drawer panel
                    drawer_panel = <ScrollYView> {
                        width: 300, height: Fill,
                        flow: Down,
                        show_bg: true,
                        draw_bg: { color: #ffffff }

                        // Logo area
                        <View> {
                            width: Fill, height: 100,
                            padding: {left: 20, top: 20},
                            flow: Down, spacing: 4,
                            show_bg: true,
                            draw_bg: { color: #397af8 }
                            align: {y: 1.0},

                            <Label> {
                                draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 24.0 } }
                                text: "Element Gallery"
                            }
                            <Label> {
                                draw_text: { color: #ffffffcc, text_style: { font_size: 12.0 } }
                                text: "makepad-element showcase"
                            }
                            <View> { width: Fill, height: 12 }
                        }

                        // Menu items
                        menu_text = <DrawerItem> { text: "Text" }
                        menu_divider = <DrawerItem> { text: "Divider" }
                        menu_chips = <DrawerItem> { text: "Chips" }
                        menu_badge = <DrawerItem> { text: "Badge" }
                        menu_fab = <DrawerItem> { text: "FAB" }
                        menu_tooltip = <DrawerItem> { text: "Tooltip" }
                        menu_overlay = <DrawerItem> { text: "Overlay" }
                        menu_linear_progress = <DrawerItem> { text: "Linear Progress" }
                        menu_skeleton = <DrawerItem> { text: "Skeleton" }
                        menu_social_icons = <DrawerItem> { text: "Social Icons" }
                        menu_speed_dial = <DrawerItem> { text: "Speed Dial" }
                        menu_buttons = <DrawerItem> { text: "Buttons" }
                        menu_checkbox = <DrawerItem> { text: "CheckBox" }
                        menu_inputs = <DrawerItem> { text: "Inputs" }
                        menu_slider = <DrawerItem> { text: "Slider" }
                        menu_ratings = <DrawerItem> { text: "Ratings" }
                        menu_avatars = <DrawerItem> { text: "Avatars" }
                        menu_cards = <DrawerItem> { text: "Cards" }
                        menu_dialogs = <DrawerItem> { text: "Dialogs" }
                        menu_pricing = <DrawerItem> { text: "Pricing" }
                        menu_tiles = <DrawerItem> { text: "Tiles" }
                        menu_image = <DrawerItem> { text: "Image" }
                        menu_tabs = <DrawerItem> { text: "Tabs" }
                        menu_lists = <DrawerItem> { text: "Lists" }
                        menu_settings = <DrawerItem> { text: "Settings" }
                        menu_theme = <DrawerItem> { text: "Theme" }
                        menu_fonts = <DrawerItem> { text: "Fonts" }
                        menu_bottom_sheet = <DrawerItem> { text: "Bottom Sheet" }
                        menu_login = <DrawerItem> { text: "Login" }
                        menu_whatsapp = <DrawerItem> { text: "WhatsApp" }
                        menu_markdown = <DrawerItem> { text: "Markdown" }

                        <View> { width: Fill, height: 20 }
                    }

                    // Backdrop
                    drawer_backdrop = <View> {
                        width: Fill, height: Fill,
                        show_bg: true,
                        draw_bg: { color: #00000066 }
                        cursor: Hand,
                    }
                }

                // ── Bottom sheet overlay ──────────────────────
                bottom_sheet_overlay = <View> {
                    visible: false,
                    width: Fill, height: Fill,
                    flow: Down,
                    align: {y: 1.0},

                    // Backdrop
                    bs_backdrop = <View> {
                        width: Fill, height: Fill,
                        show_bg: true, draw_bg: { color: #00000066 },
                        cursor: Hand,
                    }

                    // Sheet content
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        show_bg: true, draw_bg: { color: #ffffff }

                        <ElementListItem> {
                            <View> {
                                width: Fill, height: 48, align: {y: 0.5}, padding: {left: 20},
                                <Label> { draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "List Item 1" }
                            }
                        }
                        <ElementDivider> {}
                        <ElementListItem> {
                            <View> {
                                width: Fill, height: 48, align: {y: 0.5}, padding: {left: 20},
                                <Label> { draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "List Item 2" }
                            }
                        }
                        <ElementDivider> {}
                        <View> {
                            width: Fill, height: 48, align: {x: 0.5, y: 0.5},
                            show_bg: true, draw_bg: { color: #e53935 },
                            cursor: Hand,
                            bs_cancel_btn = <Label> { draw_text: { color: #fff, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Cancel" }
                        }
                    }
                }

            } // end body
        } // end Window
    } // end App
} // end live_design!


// ══════════════════════════════════════════════════════════════
//  Rust: App struct + event handling
// ══════════════════════════════════════════════════════════════

app_main!(App);

fn main() {
    app_main();
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    drawer_open: bool,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_element::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.match_event(cx, event);
    }
}

// All page ids and their corresponding menu ids
fn pages() -> Vec<(LiveId, LiveId, &'static str)> {
    vec![
        (live_id!(menu_text), live_id!(page_text), "Text"),
        (live_id!(menu_divider), live_id!(page_divider), "Divider"),
        (live_id!(menu_chips), live_id!(page_chips), "Chips"),
        (live_id!(menu_badge), live_id!(page_badge), "Badge"),
        (live_id!(menu_fab), live_id!(page_fab), "FAB"),
        (live_id!(menu_tooltip), live_id!(page_tooltip), "Tooltip"),
        (live_id!(menu_overlay), live_id!(page_overlay), "Overlay"),
        (live_id!(menu_linear_progress), live_id!(page_linear_progress), "Linear Progress"),
        (live_id!(menu_skeleton), live_id!(page_skeleton), "Skeleton"),
        (live_id!(menu_social_icons), live_id!(page_social_icons), "Social Icons"),
        (live_id!(menu_speed_dial), live_id!(page_speed_dial), "Speed Dial"),
        (live_id!(menu_buttons), live_id!(page_buttons), "Buttons"),
        (live_id!(menu_checkbox), live_id!(page_checkbox), "CheckBox"),
        (live_id!(menu_inputs), live_id!(page_inputs), "Inputs"),
        (live_id!(menu_slider), live_id!(page_slider), "Slider"),
        (live_id!(menu_ratings), live_id!(page_ratings), "Ratings"),
        (live_id!(menu_avatars), live_id!(page_avatars), "Avatars"),
        (live_id!(menu_cards), live_id!(page_cards), "Cards"),
        (live_id!(menu_dialogs), live_id!(page_dialogs), "Dialogs"),
        (live_id!(menu_pricing), live_id!(page_pricing), "Pricing"),
        (live_id!(menu_tiles), live_id!(page_tiles), "Tiles"),
        (live_id!(menu_image), live_id!(page_image), "Image"),
        (live_id!(menu_tabs), live_id!(page_tabs), "Tabs"),
        (live_id!(menu_lists), live_id!(page_lists), "Lists"),
        (live_id!(menu_settings), live_id!(page_settings), "Settings"),
        (live_id!(menu_theme), live_id!(page_theme), "Theme"),
        (live_id!(menu_fonts), live_id!(page_fonts), "Fonts"),
        (live_id!(menu_bottom_sheet), live_id!(page_bottom_sheet), "Bottom Sheet"),
        (live_id!(menu_login), live_id!(page_login), "Login"),
        (live_id!(menu_whatsapp), live_id!(page_whatsapp), "WhatsApp"),
        (live_id!(menu_markdown), live_id!(page_markdown), "Markdown"),
    ]
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Hamburger button
        if self.ui.button(ids!(hamburger_btn)).clicked(actions) {
            self.toggle_drawer(cx);
        }

        // Drawer backdrop
        if self.ui.view(ids!(drawer_backdrop)).finger_up(actions).is_some() {
            self.close_drawer(cx);
        }

        // Menu items
        for (menu_id, page_id, title) in pages() {
            if self.ui.button(&[menu_id]).clicked(actions) {
                log!("MENU CLICKED: {}", title);
                self.show_page(cx, page_id, title);
                self.close_drawer(cx);
            }
        }

        // Bottom sheet trigger
        if self.ui.button(ids!(bottom_sheet_btn)).clicked(actions) {
            self.ui.view(ids!(bottom_sheet_overlay)).set_visible(cx, true);
        }

        // Bottom sheet backdrop / cancel
        if self.ui.view(ids!(bs_backdrop)).finger_up(actions).is_some() {
            self.ui.view(ids!(bottom_sheet_overlay)).set_visible(cx, false);
        }

        // Theme color swatches
        let theme_colors: &[(LiveId, Vec4)] = &[
            (live_id!(theme_color_1), vec4(0.129, 0.588, 0.953, 1.0)),
            (live_id!(theme_color_2), vec4(0.914, 0.118, 0.388, 1.0)),
            (live_id!(theme_color_3), vec4(0.239, 0.353, 0.996, 1.0)),
            (live_id!(theme_color_4), vec4(0.275, 0.082, 0.698, 1.0)),
            (live_id!(theme_color_5), vec4(1.000, 0.839, 0.000, 1.0)),
            (live_id!(theme_color_6), vec4(1.000, 0.341, 0.133, 1.0)),
            (live_id!(theme_color_7), vec4(0.000, 0.631, 0.322, 1.0)),
        ];
        for &(swatch_id, color) in theme_colors {
            if self.ui.view(&[swatch_id]).finger_up(actions).is_some() {
                self.ui.view(ids!(header_bar)).apply_over(cx, live! {
                    draw_bg: { color: (color) }
                });
            }
        }
    }
}

impl App {
    fn toggle_drawer(&mut self, cx: &mut Cx) {
        self.drawer_open = !self.drawer_open;
        self.ui.view(ids!(drawer_overlay)).set_visible(cx, self.drawer_open);
        self.ui.redraw(cx);
    }

    fn close_drawer(&mut self, cx: &mut Cx) {
        self.drawer_open = false;
        self.ui.view(ids!(drawer_overlay)).set_visible(cx, false);
        self.ui.redraw(cx);
    }

    fn show_page(&mut self, cx: &mut Cx, page_id: LiveId, title: &str) {
        // Hide welcome + all pages
        self.ui.view(ids!(welcome_page)).set_visible(cx, false);
        for (_, pid, _) in pages() {
            self.ui.view(&[pid]).set_visible(cx, false);
        }
        // Show target page
        self.ui.view(&[page_id]).set_visible(cx, true);
        // Update header title
        self.ui.label(ids!(header_title)).set_text(cx, title);
        self.ui.redraw(cx);
    }
}
