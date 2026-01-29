//! Element Gallery - React Native Elements Example App replica
//!
//! Drawer-navigation showcase with 6 tier pages demonstrating every
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
    use makepad_element::components::clickable_view::*;
    use makepad_element::components::popup_menu::*;
    use makepad_element::components::dropdown::*;
    use makepad_element::components::chat_bubble::*;
    use makepad_element::components::filler::*;
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
        grab_key_focus: false,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return mix(#ffffff, #f0f0f6, self.hover);
            }
        }
        draw_text: {
            // Keep text dark in ALL states (normal, hover, down, focus, disabled)
            color: #43484d,
            color_hover: #43484d,
            color_down: #43484d,
            color_focus: #43484d,
            color_disabled: #999999,
            text_style: <THEME_FONT_REGULAR> { font_size: 15.0 }
        }
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
                flow: Right,

                // ── Drawer panel (sliding side panel) ────────
                drawer_panel = <View> {
                    visible: false,
                    width: 280, height: Fill,

                    drawer_scroll = <ScrollYView> {
                        width: Fill, height: Fill,
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

                        // Menu items - 5 Tiers
                        menu_tier1 = <DrawerItem> { text: "Tier 1: Basic" }
                        menu_tier2 = <DrawerItem> { text: "Tier 2: Form" }
                        menu_tier3 = <DrawerItem> { text: "Tier 3: Data Display" }
                        menu_tier4 = <DrawerItem> { text: "Tier 4: Feedback" }
                        menu_tier5 = <DrawerItem> { text: "Tier 5: Navigation" }

                        // WeChat-style widgets
                        <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: #e8e8e8 } }
                        <View> {
                            width: Fill, height: 32, padding: {left: 20}, align: {y: 0.5},
                            <Label> { draw_text: { color: #999, text_style: { font_size: 11.0 } } text: "EXTRAS" }
                        }
                        menu_wechat = <DrawerItem> { text: "WeChat Widgets" }

                        <View> { width: Fill, height: 20 }
                    } // end ScrollYView
                } // end drawer_panel

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
                        // TIER 1: BASIC (Text, Buttons, Icons, Divider, Image, Markdown)
                        // ══════════════════════════════════════
                        page_tier1 = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tier 1: Basic" }

                            // TEXT
                            <SubHeader> { label = { text: "TEXT" } }
                            <ElementH1> { text: "Heading 1", draw_text: { color: #8F0CE8 } }
                            <ElementH2> { text: "Heading 2", draw_text: { color: #00B233 } }
                            <ElementH3> { text: "Heading 3", draw_text: { color: #FF9800 } }
                            <ElementH4> { text: "Heading 4", draw_text: { color: #397af8 } }
                            <ElementBody> { text: "Body text — The quick brown fox jumps over the lazy dog." }
                            <ElementCaption> { text: "Caption text — secondary information" }

                            // BUTTONS
                            <SubHeader> { label = { text: "BUTTONS" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonSolid> { text: "Primary" }
                                <ElementButtonSolid> { text: "Secondary", draw_bg: { color: #8F0CE8 } }
                            }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonOutline> { text: "Outline" }
                                <ElementButtonClear> { text: "Clear" }
                            }

                            // ICONS
                            <SubHeader> { label = { text: "SOCIAL ICONS" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementSocialIcon> { text: "fb", draw_bg: { color: #3b5998 } }
                                <ElementSocialIcon> { text: "tw", draw_bg: { color: #00aced } }
                                <ElementSocialIcon> { text: "gg", draw_bg: { color: #dd4b39 } }
                                <ElementSocialIcon> { text: "gh", draw_bg: { color: #333333 } }
                            }

                            // DIVIDER
                            <SubHeader> { label = { text: "DIVIDER" } }
                            <ElementDivider> {}
                            <ElementDividerInset> {}
                            <View> { width: Fill, height: 50, flow: Right, align: {y: 0.5}, spacing: 16,
                                <Label> { draw_text: { color: #424242 } text: "Left" }
                                <ElementDividerVertical> {}
                                <Label> { draw_text: { color: #424242 } text: "Right" }
                            }

                            // IMAGE
                            <SubHeader> { label = { text: "IMAGE" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <RoundedView> { width: 80, height: 80, draw_bg: { color: #e91e63, border_radius: 8.0 } }
                                <RoundedView> { width: 80, height: 80, draw_bg: { color: #9c27b0, border_radius: 8.0 } }
                                <RoundedView> { width: 80, height: 80, draw_bg: { color: #673ab7, border_radius: 8.0 } }
                            }

                            // MARKDOWN
                            <SubHeader> { label = { text: "MARKDOWN" } }
                            <ElementMarkdown> {
                                body: "# Heading\n\n**Bold** and *italic* text.\n\n- List item 1\n- List item 2\n\n> A blockquote"
                            }
                        }

                        // ══════════════════════════════════════
                        // TIER 2: FORM (Inputs, CheckBox, Slider, Ratings)
                        // ══════════════════════════════════════
                        page_tier2 = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tier 2: Form" }

                            // INPUTS
                            <SubHeader> { label = { text: "INPUTS" } }
                            <ElementSearchBar> { }
                            <ElementInput> { text: "", empty_text: "Enter your name" }
                            <Label> { draw_text: { color: #8693a0, text_style: { font_size: 13.0 } } text: "EMAIL" }
                            <ElementInput> { text: "", empty_text: "user@example.com" }

                            // CHECKBOX & SWITCH
                            <SubHeader> { label = { text: "CHECKBOX & SWITCH" } }
                            <ElementCheckBox> { text: "Unchecked option" }
                            <ElementCheckBox> { text: "Checked option", animator: { selected = { default: on } } }
                            <ElementSwitch> {}

                            // SLIDER
                            <SubHeader> { label = { text: "SLIDER" } }
                            <ElementSlider> { text: "Volume", min: 0.0, max: 100.0 }
                            <ElementSliderRound> { text: "Brightness", min: 0.0, max: 100.0 }

                            // RATINGS
                            <SubHeader> { label = { text: "RATINGS" } }
                            <ElementRating0> {}
                            <ElementRating3> {}
                            <ElementRating5> {}
                        }

                        // ══════════════════════════════════════
                        // TIER 3: DATA DISPLAY (Avatar, Badge, Card, List, Chip, Tile, Pricing)
                        // ══════════════════════════════════════
                        page_tier3 = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tier 3: Data Display" }

                            // AVATARS
                            <SubHeader> { label = { text: "AVATARS" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                                <ElementAvatar> { text_view = { label = { text: "Fc" }, draw_bg: { bg_color: #3d4db7 } } }
                                <ElementAvatar> { text_view = { label = { text: "P" }, draw_bg: { bg_color: #FF6F61 } } }
                                <ElementAvatar> { text_view = { label = { text: "Rd" }, draw_bg: { bg_color: #9C27B0 } } }
                            }

                            // BADGES
                            <SubHeader> { label = { text: "BADGES" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 24,
                                <ElementBadge> { label = { text: "3" }, draw_bg: { color: #00B233 } }
                                <ElementBadge> { label = { text: "99+" }, draw_bg: { color: #e53935 } }
                                <ElementBadge> { label = { text: "NEW" }, draw_bg: { color: #397af8 } }
                            }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 16,
                                <ElementBadgeDot> { draw_bg: { color: #00B233 } }
                                <ElementBadgeDot> { draw_bg: { color: #e53935 } }
                                <ElementBadgeDot> { draw_bg: { color: #FF9800 } }
                            }

                            // CHIPS
                            <SubHeader> { label = { text: "CHIPS" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementChip> { label = { text: "Primary" } }
                                <ElementChip> { label = { text: "Success" }, draw_bg: { color: #00B233 } }
                                <ElementChipOutline> { label = { text: "Outline" } }
                            }

                            // CARDS
                            <SubHeader> { label = { text: "CARDS" } }
                            <ElementCard> {
                                <View> { width: Fill, height: 80, show_bg: true, draw_bg: { color: #e3f2fd } }
                                <View> { width: Fill, height: Fit, padding: 12, flow: Down, spacing: 4,
                                    <Label> { draw_text: { color: #333, text_style: { font_size: 16.0 } } text: "Card Title" }
                                    <Label> { draw_text: { color: #666, text_style: { font_size: 14.0 } } text: "Card description text here" }
                                }
                            }

                            // TILES
                            <SubHeader> { label = { text: "TILES" } }
                            <ElementTile> {
                                bg = { draw_bg: { color: #ff5722 } }
                                overlay = {
                                    title = { text: "Featured Tile" }
                                    subtitle = { text: "Tap to view more" }
                                }
                            }

                            // LIST ITEMS
                            <SubHeader> { label = { text: "LIST ITEMS" } }
                            <ElementListItem> {
                                <View> { width: Fill, height: 56, padding: {left: 16, right: 16}, align: {y: 0.5}, flow: Right,
                                    <ElementAvatar> { text_view = { label = { text: "JD" }, draw_bg: { bg_color: #397af8 } } }
                                    <View> { width: 12, height: 1 }
                                    <View> { width: Fill, height: Fit, flow: Down,
                                        <Label> { draw_text: { color: #333 } text: "John Doe" }
                                        <Label> { draw_text: { color: #999, text_style: { font_size: 12.0 } } text: "Software Engineer" }
                                    }
                                }
                            }
                            <ElementDivider> {}
                            <ElementListItem> {
                                <View> { width: Fill, height: 56, padding: {left: 16, right: 16}, align: {y: 0.5}, flow: Right,
                                    <ElementAvatar> { text_view = { label = { text: "AS" }, draw_bg: { bg_color: #e91e63 } } }
                                    <View> { width: 12, height: 1 }
                                    <View> { width: Fill, height: Fit, flow: Down,
                                        <Label> { draw_text: { color: #333 } text: "Alice Smith" }
                                        <Label> { draw_text: { color: #999, text_style: { font_size: 12.0 } } text: "Product Designer" }
                                    }
                                }
                            }
                        }

                        // ══════════════════════════════════════
                        // TIER 4: FEEDBACK (Dialog, Overlay, Tooltip, Skeleton, Progress)
                        // ══════════════════════════════════════
                        page_tier4 = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tier 4: Feedback" }

                            // TOOLTIPS
                            <SubHeader> { label = { text: "TOOLTIPS" } }
                            <ElementTooltip> { label = { text: "Default tooltip" } }
                            <ElementTooltip> { label = { text: "Primary" }, draw_bg: { color: #397af8 } }
                            <ElementTooltip> { label = { text: "Success" }, draw_bg: { color: #00B233 } }

                            // LINEAR PROGRESS
                            <SubHeader> { label = { text: "LINEAR PROGRESS" } }
                            <ElementLinearProgressIndeterminate> { }
                            <ElementLinearProgress> { draw_bg: { progress: 0.25 } }
                            <ElementLinearProgress> { draw_bg: { progress: 0.75 } }

                            // SKELETON
                            <SubHeader> { label = { text: "SKELETON" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSkeletonCircle> { width: 40, height: 40 }
                                <View> { width: Fill, height: Fit, flow: Down, spacing: 8,
                                    <ElementSkeletonLine> { width: Fill, height: 14 }
                                    <ElementSkeletonLine> { width: 200, height: 14 }
                                }
                            }
                            <ElementSkeletonRect> { width: Fill, height: 100 }

                            // OVERLAY & DIALOG
                            <SubHeader> { label = { text: "OVERLAY & DIALOG" } }
                            <Label> { draw_text: { color: #666 } text: "Dialog and Overlay components provide modal interactions." }
                            overlay_trigger_btn = <ElementButtonSolid> { text: "Show Overlay" }
                        }

                        // ══════════════════════════════════════
                        // TIER 5: NAVIGATION (Tab, FAB, Speed Dial, Bottom Sheet, etc.)
                        // ══════════════════════════════════════
                        page_tier5 = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "Tier 5: Navigation" }

                            // TABS
                            <SubHeader> { label = { text: "TABS" } }
                            <ElementTabBar> {
                                bar = {
                                    <ElementTabItem> { text: "Tab 1" }
                                    <ElementTabItem> { text: "Tab 2" }
                                    <ElementTabItem> { text: "Tab 3" }
                                }
                            }

                            // FAB
                            <SubHeader> { label = { text: "FAB" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                                <ElementFabSmall> {}
                                <ElementFab> { draw_bg: { color: #00B233 } }
                                <ElementFabExtended> { text: "Navigate" }
                            }

                            // SPEED DIAL
                            <SubHeader> { label = { text: "SPEED DIAL" } }
                            <ElementSpeedDial> {}

                            // BOTTOM SHEET
                            <SubHeader> { label = { text: "BOTTOM SHEET" } }
                            bottom_sheet_btn = <ElementButtonSolid> { text: "Open Bottom Sheet" }

                            // THEME
                            <SubHeader> { label = { text: "THEME COLORS" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                theme_color_1 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #2196F3, border_radius: 20.0 } }
                                theme_color_2 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #e91e63, border_radius: 20.0 } }
                                theme_color_3 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #3d5afe, border_radius: 20.0 } }
                                theme_color_4 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #4615b2, border_radius: 20.0 } }
                                theme_color_5 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #ffd600, border_radius: 20.0 } }
                                theme_color_6 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #ff5722, border_radius: 20.0 } }
                                theme_color_7 = <RoundedView> { width: 40, height: 40, cursor: Hand, draw_bg: { color: #00a152, border_radius: 20.0 } }
                            }
                            <Label> { draw_text: { color: #666 } text: "Tap a color to change the header theme" }
                        }

                        // ══════════════════════════════════════
                        // WECHAT WIDGETS
                        // ══════════════════════════════════════
                        page_wechat = <ScrollYView> {
                            visible: false,
                            width: Fill, height: Fill, flow: Down,
                            padding: 16, spacing: 16,

                            <ScreenTitle> { text: "WeChat Widgets" }

                            // CLICKABLE VIEW
                            <SubHeader> { label = { text: "CLICKABLE VIEW" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                                <ElementClickableView> {
                                    width: 100, height: 60, align: {x: 0.5, y: 0.5},
                                    draw_bg: { color: #397af8 }
                                    <Label> { draw_text: { color: #fff } text: "Basic" }
                                }
                                <ElementClickableViewHover> {
                                    width: 100, height: 60, align: {x: 0.5, y: 0.5},
                                    draw_bg: { bg_color: #52c41a, bg_color_hover: #389e0d }
                                    <Label> { draw_text: { color: #fff } text: "Hover" }
                                }
                                <ElementClickableRoundedView> {
                                    width: 100, height: 60, align: {x: 0.5, y: 0.5},
                                    draw_bg: { bg_color: #ff4d4f, bg_color_hover: #ff7875 }
                                    <Label> { draw_text: { color: #fff } text: "Rounded" }
                                }
                            }

                            // DROPDOWN & MENU
                            <SubHeader> { label = { text: "DROPDOWN & MENU" } }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                                <ElementDropdown> { label = { text: "Select option" } }
                                <ElementDropdownIcon> {
                                    icon_wrapper = { icon = { draw_icon: { svg_file: dep("crate://self/resources/icons/menu.svg") } } }
                                }
                            }
                            <ElementPopupMenu> {
                                <ElementMenuItem> { label = { text: "Edit" } }
                                <ElementMenuItem> { label = { text: "Delete" } }
                                <ElementMenuItem> { label = { text: "Share" } }
                            }

                            // CHAT BUBBLES
                            <SubHeader> { label = { text: "CHAT BUBBLES" } }
                            <ElementChatMessageIncoming> {
                                bubble = { content = { text: "Hello! How are you?" } }
                            }
                            <ElementChatMessageOutgoing> {
                                bubble = { content = { text: "I'm doing great, thanks!" } }
                            }
                            <ElementChatMessageIncoming> {
                                bubble = { content = { text: "That's wonderful to hear!" } }
                            }

                            // FILLER & SPACERS
                            <SubHeader> { label = { text: "FILLER & SPACERS" } }
                            <View> { width: Fill, height: 80, flow: Down, show_bg: true, draw_bg: { color: #f5f5f5 },
                                <RoundedView> { width: Fill, height: 20, draw_bg: { color: #2196F3, border_radius: 4.0 }, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #fff, text_style: { font_size: 10.0 } } text: "Top" }
                                }
                                <FillerY> {}
                                <RoundedView> { width: Fill, height: 20, draw_bg: { color: #4caf50, border_radius: 4.0 }, align: {x: 0.5, y: 0.5},
                                    <Label> { draw_text: { color: #fff, text_style: { font_size: 10.0 } } text: "Bottom (FillerY pushes)" }
                                }
                            }
                        }

                    } // end page_container
                } // end content_layer

                // ── Bottom sheet overlay ──────────────────────
                bottom_sheet_overlay = <View> {
                    visible: false,
                    width: Fill, height: Fill,
                    flow: Overlay,

                    // Backdrop (covers full screen)
                    bs_backdrop = <View> {
                        width: Fill, height: Fill,
                        show_bg: true, draw_bg: { color: #00000066 },
                        cursor: Hand,
                    }

                    // Sheet content (aligned to bottom)
                    <View> {
                        width: Fill, height: Fill,
                        flow: Down,
                        align: {y: 1.0},

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
                            bs_cancel_view = <View> {
                                width: Fill, height: 48, align: {x: 0.5, y: 0.5},
                                show_bg: true, draw_bg: { color: #e53935 },
                                cursor: Hand,
                                <Label> { draw_text: { color: #fff, text_style: <THEME_FONT_REGULAR> { font_size: 16.0 } } text: "Cancel" }
                            }
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

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Hamburger button toggles drawer
        if self.ui.button(ids!(hamburger_btn)).clicked(actions) {
            self.toggle_drawer(cx);
        }

        // Menu items - click to show page and close drawer
        if self.ui.button(ids!(menu_tier1)).clicked(actions) {
            self.show_page(cx, live_id!(page_tier1), "Tier 1: Basic");
            self.close_drawer(cx);
        }
        if self.ui.button(ids!(menu_tier2)).clicked(actions) {
            self.show_page(cx, live_id!(page_tier2), "Tier 2: Form");
            self.close_drawer(cx);
        }
        if self.ui.button(ids!(menu_tier3)).clicked(actions) {
            self.show_page(cx, live_id!(page_tier3), "Tier 3: Data Display");
            self.close_drawer(cx);
        }
        if self.ui.button(ids!(menu_tier4)).clicked(actions) {
            self.show_page(cx, live_id!(page_tier4), "Tier 4: Feedback");
            self.close_drawer(cx);
        }
        if self.ui.button(ids!(menu_tier5)).clicked(actions) {
            self.show_page(cx, live_id!(page_tier5), "Tier 5: Navigation");
            self.close_drawer(cx);
        }
        if self.ui.button(ids!(menu_wechat)).clicked(actions) {
            self.show_page(cx, live_id!(page_wechat), "WeChat Widgets");
            self.close_drawer(cx);
        }

        // Bottom sheet trigger
        if self.ui.button(ids!(bottom_sheet_btn)).clicked(actions) {
            self.ui.view(ids!(bottom_sheet_overlay)).set_visible(cx, true);
        }

        // Bottom sheet backdrop / cancel
        if self.ui.view(ids!(bs_backdrop)).finger_up(actions).is_some()
            || self.ui.view(ids!(bs_cancel_view)).finger_up(actions).is_some()
        {
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
        self.ui.view(ids!(drawer_panel)).set_visible(cx, self.drawer_open);
    }

    fn close_drawer(&mut self, cx: &mut Cx) {
        self.drawer_open = false;
        self.ui.view(ids!(drawer_panel)).set_visible(cx, false);
    }

    fn show_page(&mut self, cx: &mut Cx, page_id: LiveId, title: &str) {
        // Hide welcome + all tier pages
        self.ui.view(ids!(welcome_page)).set_visible(cx, false);
        self.ui.view(ids!(page_tier1)).set_visible(cx, false);
        self.ui.view(ids!(page_tier2)).set_visible(cx, false);
        self.ui.view(ids!(page_tier3)).set_visible(cx, false);
        self.ui.view(ids!(page_tier4)).set_visible(cx, false);
        self.ui.view(ids!(page_tier5)).set_visible(cx, false);
        self.ui.view(ids!(page_wechat)).set_visible(cx, false);
        // Show target page
        self.ui.view(&[page_id]).set_visible(cx, true);
        // Update header title
        self.ui.label(ids!(header_title)).set_text(cx, title);
    }
}
