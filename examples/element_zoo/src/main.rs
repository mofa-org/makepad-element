//! Element Zoo - Showcase of all makepad-element components
//!
//! FileTree-based gallery with component tree on the left,
//! detail pages on the right.

use makepad_widgets::*;
use makepad_widgets::file_tree::*;
use std::collections::HashMap;

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
    // use makepad_element::components::curved_label::*; // BUG: widget doesn't render
    use makepad_element::components::staggered_grid::*;
    use makepad_element::theme::font::*;

    // Re-export Manrope font for use in this live_design! scope
    pub FONT_MANROPE = <ELEMENT_FONT_MANROPE> {}
    pub THEME_FONT_REGULAR = <FONT_MANROPE> {}

    pub THEME_COLOR_LABEL_INNER = #333333
    pub THEME_COLOR_LABEL_INNER_HOVER = #111111
    pub THEME_COLOR_LABEL_INNER_ACTIVE = #2089dc
    pub THEME_COLOR_TEXT = #333333

    StaggeredGridDemo = {{StaggeredGridDemo}} {
        width: Fill, height: 400,
        grid = <ElementStaggeredGrid> {
            columns_number: 2,
            column_spacing: 4.0,
            GridItem = <RoundedView> {
                width: Fill, height: Fit,
                show_bg: true,
                draw_bg: { color: #2089dc, border_radius: 8.0 }
                padding: 12,
                align: { x: 0.5, y: 0.5 }
                item_label = <Label> {
                    draw_text: { color: #fff, text_style: { font_size: 12.0 } }
                    text: ""
                }
            }
        }
    }

    ComponentTree = {{ComponentTree}} {
        file_tree: <FileTree> {
            node_height: 28.0,
            folder_node: {
                draw_bg: {
                    color_1: #f8f8fc,
                    color_2: #f0f0f6,
                }
                draw_text: {
                    color: #333333,
                    color_active: #2089dc,
                    text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                }
                draw_icon: {
                    color: #555555,
                    color_active: #2089dc,
                }
            }
            file_node: {
                draw_bg: {
                    color_1: #f8f8fc,
                    color_2: #f0f0f6,
                }
                draw_text: {
                    color: #555555,
                    color_active: #2089dc,
                    text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                }
                draw_icon: {
                    color: #777777,
                    color_active: #2089dc,
                }
            }
            filler: { draw_bg: { color: #f4f4fa } }
        }
    }

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            width: Fill,
            height: Fill,

            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #F1F1F7;
                }
            }

            body = <View> {
                width: Fill,
                height: Fill,
                flow: Down,

                // Header across the full width
                header_bar = <View> {
                    width: Fill, height: 56,
                    flow: Right,
                    padding: {left: 20, right: 20},
                    align: {y: 0.5},
                    show_bg: true,
                    draw_bg: { color: #2089dc }

                    <Label> { width: Fit, height: Fit,
                        draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 20.0 } }
                        text: "Element Zoo" }

                    <View> { width: Fill, height: Fit }

                    <Label> { width: Fit, height: Fit,
                        draw_text: { color: #ffffffcc, text_style: <THEME_FONT_REGULAR> { font_size: 12.0 } }
                        text: "makepad-element component gallery" }
                }

                // Main area: tree + content
                main_area = <View> {
                    width: Fill, height: Fill,
                    flow: Right,

                    // Left: FileTree sidebar
                    component_tree = <ComponentTree> {
                        file_tree: { width: 260, height: Fill }
                    }

                    // Right: Content panel with overlay pages
                    content_panel = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Overlay,

                        // Welcome page (shown by default)
                        welcome_page = <View> {
                            width: Fill, height: Fill,
                            align: {x: 0.5, y: 0.5},
                            flow: Down, spacing: 8,

                            <Label> { width: Fit, height: Fit,
                                draw_text: { color: #333333, text_style: <THEME_FONT_REGULAR> { font_size: 28.0 } }
                                text: "Element Zoo" }
                            <Label> { width: Fit, height: Fit,
                                draw_text: { color: #888888, text_style: <THEME_FONT_REGULAR> { font_size: 14.0 } }
                                text: "Select a component from the tree to preview" }
                        }

                    // === Detail Pages ===

                    // 1. Text
                    text_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 8,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Text" }
                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Typography variants for headings, body, and caption text." }

                        <ElementH1> { text: "Heading 1 (32px)" }
                        <ElementH2> { text: "Heading 2 (24px)" }
                        <ElementH3> { text: "Heading 3 (20px)" }
                        <ElementH4> { text: "Heading 4 (16px)" }
                        <ElementBody> { text: "Body text (14px) - The quick brown fox jumps over the lazy dog." }
                        <ElementCaption> { text: "Caption text (12px) - Secondary information" }
                        <ElementOverline> { text: "OVERLINE TEXT (10px)" }
                    }

                    // 2. Markdown (under Text)
                    markdown_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Markdown" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Markdown rendering with rich text formatting." }

                        <ElementMarkdown> {
                            body: "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\nThis is **bold text** and *italic text* and ***bold italic***.\n\n- Item one\n- Item two\n- Item three\n\n> A blockquote with some wisdom.\n\nInline `code` and a code block:\n\n```\nfn main() {\n    println!(\"Hello Makepad!\");\n}\n```\n\n---\n\n### Math (Unicode)\n\n`integral(0, inf) e^(-x^2) dx = sqrt(pi)/2`\n\n`e^(i*pi) + 1 = 0` (Euler's identity)\n\n`x = (-b +/- sqrt(b^2 - 4ac)) / 2a` (Quadratic formula)\n\n---\n\nA [link](https://makepad.dev) to Makepad."
                        }

                        <Label> { width: Fit, height: Fit, margin: {top: 16},
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Math via Unicode symbols. Full LaTeX rendering requires makepad math_widget crate (Typst backend)." }
                    }

                    // 3. Divider
                    divider_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Divider" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Horizontal divider" }
                        <ElementDivider> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Divider with inset" }
                        <ElementDividerInset> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Vertical divider (in a row)" }
                        <View> {
                            width: Fill, height: 60, flow: Right, align: {y: 0.5}, spacing: 16,
                            <Label> { width: Fit, height: Fit,
                                draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Left" }
                            <ElementDividerVertical> {}
                            <Label> { width: Fit, height: Fit,
                                draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Right" }
                        }
                    }

                    // 3. Icon
                    icon_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Icon" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Icon sizes and colors with SVG rendering." }

                        <View> {
                            width: Fill, height: Fit, flow: Right, spacing: 24, align: {y: 0.5},
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <ElementIconSmall> {}
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Small 16px" }
                            }
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <ElementIcon> {}
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Medium 24px" }
                            }
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <ElementIconLarge> {}
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Large 32px" }
                            }
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <ElementIconPrimary> {}
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Primary" }
                            }
                        }
                    }

                    // 4. Button
                    button_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Button" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Solid buttons" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementButtonSolid> { text: "Primary" }
                            <ElementButtonSecondary> { text: "Secondary" }
                            <ElementButtonSuccess> { text: "Success" }
                        }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementButtonWarning> { text: "Warning" }
                            <ElementButtonError> { text: "Error" }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Outline button" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementButtonOutline> { text: "Outline" }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Clear/text button" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementButtonClear> { text: "Clear Button" }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Disabled buttons" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementButtonDisabled> { text: "Disabled" }
                            <ElementButtonSolid> {
                                text: "Disabled (anim)"
                                animator: { disabled = { default: on } }
                            }
                        }
                    }

                    // 5. Image
                    image_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Image" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Image variants with different shapes. (No images loaded - showing placeholders.)" }

                        <View> {
                            width: Fill, height: Fit, flow: Right, spacing: 16,
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <View> { width: 150, height: 100, show_bg: true, draw_bg: { color: #E0E0E0 } }
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Standard" }
                            }
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <RoundedView> { width: 150, height: 100, show_bg: true, draw_bg: { color: #E0E0E0, border_radius: 12.0 } }
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Rounded" }
                            }
                            <View> { width: Fit, height: Fit, flow: Down, spacing: 4, align: {x: 0.5},
                                <View> { width: 100, height: 100, show_bg: true,
                                    draw_bg: {
                                        instance bg_color: #E0E0E0,
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            let c = self.rect_size * 0.5;
                                            sdf.circle(c.x, c.y, c.x);
                                            sdf.fill(self.bg_color);
                                            return sdf.result;
                                        }
                                    }
                                }
                                <Label> { width: Fit, height: Fit, draw_text: { color: #757575, text_style: { font_size: 11.0 } } text: "Circle" }
                            }
                        }
                    }

                    // 6. Switch
                    switch_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Switch" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Toggle switches" }

                        <View> { width: Fill, height: Fit, flow: Down, spacing: 12,
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSwitch> {}
                                <Label> { width: Fit, height: Fit, draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Default" }
                            }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSwitch> { animator: { selected = { default: on } } }
                                <Label> { width: Fit, height: Fit, draw_text: { color: #424242, text_style: { font_size: 14.0 } } text: "Selected" }
                            }
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSwitchLabeled> { text: "With Label" }
                            }
                        }
                    }

                    // 7. CheckBox
                    check_box_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "CheckBox" }

                        <View> { width: Fill, height: Fit, flow: Down, spacing: 12,
                            <ElementCheckBox> { text: "Unchecked" }
                            <ElementCheckBox> { text: "Checked", animator: { selected = { default: on } } }
                            <ElementCheckBox> { text: "Disabled", animator: { disabled = { default: on } } }
                            <ElementCheckBoxFlat> { text: "Flat Style" }
                        }
                    }

                    // 8. Slider
                    slider_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Slider" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Default slider" }
                        <ElementSlider> { text: "Volume" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Round slider" }
                        <ElementSliderRound> { text: "Brightness" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Minimal slider" }
                        <ElementSliderMinimal> { text: "Opacity" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Stepped slider (step: 10)" }
                        <ElementSlider> { text: "Step", step: 10.0 }
                    }

                    // 9. Input
                    input_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Input" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Standard input" }
                        <ElementInput> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Flat input" }
                        <ElementInputFlat> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Labeled input" }
                        <ElementInputLabeled> {
                            label = { text: "Email" }
                            input = { empty_text: "you@example.com" }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Input with error" }
                        <ElementInputError> {
                            label = { text: "Username" }
                            input = { empty_text: "Required" }
                            error = { text: "This field is required" }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Password input" }
                        <ElementInputPassword> {}
                    }

                    // 10. Card
                    card_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Card" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Basic card" }
                        <ElementCard> {
                            <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                                text: "This is a basic card with some content inside." }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Card with title" }
                        <ElementCardTitled> {
                            header = { title = { text: "Featured" } }
                            content = {
                                <Label> { draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                                    text: "Card content with a title header and divider." }
                            }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Card with actions" }
                        <ElementCard> {
                            <Label> { draw_text: { color: #333333, text_style: { font_size: 16.0 } } text: "Action Card" }
                            <Label> { draw_text: { color: #757575, text_style: { font_size: 14.0 } }
                                text: "A card with action buttons at the bottom." }
                            <ElementDivider> {}
                            <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                                <ElementButtonClear> { text: "Cancel" }
                                <ElementButtonSolid> { text: "Confirm" }
                            }
                        }
                    }

                    // 11. Avatar
                    avatar_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Avatar" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Sizes" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementAvatarSmall> { text_view = { label = { text: "S" } } }
                            <ElementAvatar> { text_view = { label = { text: "M" } } }
                            <ElementAvatarLarge> { text_view = { label = { text: "L" } } }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Different colors" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementAvatar> { text_view = { draw_bg: { bg_color: #ff190c } label = { text: "R" } } }
                            <ElementAvatar> { text_view = { draw_bg: { bg_color: #52c41a } label = { text: "G" } } }
                            <ElementAvatar> { text_view = { draw_bg: { bg_color: #faad14 } label = { text: "Y" } } }
                            <ElementAvatar> { text_view = { draw_bg: { bg_color: #ca71eb } label = { text: "P" } } }
                        }
                    }

                    // 12. Badge
                    badge_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Badge" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Badge variants" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementBadge> { label = { text: "3" } }
                            <ElementBadge> { label = { text: "99+" } }
                            <ElementBadgePrimary> { label = { text: "New" } }
                            <ElementBadgeSuccess> { label = { text: "OK" } }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Dot badge" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementBadgeDot> {}
                            <ElementBadgeDot> { draw_bg: { color: #52c41a } }
                            <ElementBadgeDot> { draw_bg: { color: #faad14 } }
                        }
                    }

                    // 13. Chip
                    chip_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Chip" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Solid chips" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                            <ElementChip> { label = { text: "Default" } }
                            <ElementChip> { draw_bg: { color: #52c41a } label = { text: "Success" } }
                            <ElementChip> { draw_bg: { color: #ff190c } label = { text: "Error" } }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Outline chips" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
                            <ElementChipOutline> { label = { text: "Outline" } }
                            <ElementChipOutline> { draw_bg: { border_color: #52c41a } label = { draw_text: { color: #52c41a } text: "Success" } }
                        }
                    }

                    // 14. Header
                    header_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Header" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Primary header" }
                        <ElementHeader> {
                            bar = { center = { title = { text: "My App" } } }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Transparent header" }
                        <ElementHeaderTransparent> {
                            bar = { center = { title = { text: "Settings" } } }
                        }
                    }

                    // 15. ListItem
                    list_item_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 0,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "ListItem" }

                        <ElementListItem> {
                            content = { title = { text: "Simple Item" } subtitle = { text: "Description text" } }
                        }
                        <ElementDivider> {}
                        <ElementListItem> {
                            content = { title = { text: "Another Item" } subtitle = { text: "More details here" } }
                        }
                        <ElementDivider> {}
                        <ElementListItemAvatar> {
                            content = { title = { text: "John Doe" } subtitle = { text: "john@example.com" } }
                        }
                        <ElementDivider> {}
                        <ElementListItemAvatar> {
                            avatar = { draw_bg: { color: #52c41a } <Label> { draw_text: { color: #ffffff, text_style: { font_size: 14.0 } } text: "B" } }
                            content = { title = { text: "Bob Smith" } subtitle = { text: "bob@example.com" } }
                        }
                    }

                    // 16. Tab
                    tab_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tab" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Tab bar with radio buttons" }

                        <View> {
                            width: Fill, height: Fit, flow: Right,
                            show_bg: true, draw_bg: { color: #ffffff }
                            <ElementTabItem> { text: "Tab 1", animator: { selected = { default: on } } }
                            <ElementTabItem> { text: "Tab 2" }
                            <ElementTabItem> { text: "Tab 3" }
                        }
                    }

                    // 17. SearchBar
                    search_bar_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SearchBar" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Default search bar" }
                        <ElementSearchBar> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom placeholder" }
                        <ElementSearchBar> {
                            input_container = { input = { empty_text: "Search products..." } }
                        }
                    }

                    // 18. LinearProgress
                    linear_progress_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "LinearProgress" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "25% progress" }
                        <ElementLinearProgress> { draw_bg: { progress: 0.25 } }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "50% progress" }
                        <ElementLinearProgress> { draw_bg: { progress: 0.50 } }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "75% progress (thick)" }
                        <ElementLinearProgressThick> { draw_bg: { progress: 0.75 } }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "100% success color" }
                        <ElementLinearProgressThick> { draw_bg: { progress: 1.0, fill_color: #52c41a } }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Error color" }
                        <ElementLinearProgressThick> { draw_bg: { progress: 0.33, fill_color: #ff190c } }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Indeterminate" }
                        <ElementLinearProgressIndeterminate> {}
                    }

                    // 19. Rating
                    rating_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Rating" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "0 stars" }
                        <ElementRating0> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "1 star" }
                        <ElementRating1> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "2 stars" }
                        <ElementRating2> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "3 stars" }
                        <ElementRating3> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "4 stars" }
                        <ElementRating4> {}

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "5 stars" }
                        <ElementRating5> {}
                    }

                    // 20. FAB
                    fab_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "FAB" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Floating Action Buttons" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementFabSmall> {}
                            <ElementFab> {}
                            <ElementFabExtended> {}
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom colors" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
                            <ElementFab> { draw_bg: { color: #ff190c, color_hover: #d9150a } }
                            <ElementFab> { draw_bg: { color: #52c41a, color_hover: #46a817 } }
                        }
                    }

                    // 21. Tooltip
                    tooltip_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tooltip" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Tooltip variants" }

                        <ElementTooltip> { label = { text: "This is a tooltip" } }
                        <ElementTooltip> { label = { text: "Another tooltip with longer text content" } }
                        <ElementTooltip> { draw_bg: { color: #617080 } label = { text: "Primary tooltip" } }
                    }

                    // 22. Overlay
                    overlay_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Overlay" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Overlay backdrop preview (scaled down)" }

                        <View> {
                            width: 300, height: 200, flow: Overlay, align: {x: 0.5, y: 0.5},

                            <View> { width: Fill, height: Fill, show_bg: true, draw_bg: { color: #00000044 } }
                            <RoundedView> {
                                width: 200, height: 100, padding: 16,
                                align: {x: 0.5, y: 0.5},
                                show_bg: true, draw_bg: { color: #ffffff, border_radius: 8.0 }
                                <Label> { draw_text: { color: #333333, text_style: { font_size: 14.0 } } text: "Overlay content" }
                            }
                        }
                    }

                    // 23. Dialog
                    dialog_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Dialog" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Alert dialog" }
                        <ElementDialog> {
                            title = { text: "Alert" }
                            body = { text: "Something important happened. Please acknowledge." }
                            actions = {
                                <ElementButtonSolid> { text: "OK" }
                            }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Confirm dialog" }
                        <ElementDialog> {
                            title = { text: "Confirm" }
                            body = { text: "Are you sure you want to proceed with this action?" }
                            actions = {
                                <ElementButtonClear> { text: "Cancel" }
                                <ElementButtonSolid> { text: "Confirm" }
                            }
                        }
                    }

                    // 24. Tile
                    tile_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tile" }

                        <ElementTile> {
                            bg = { draw_bg: { color: #2089dc } }
                            overlay = {
                                title = { text: "Ocean View" }
                                subtitle = { text: "Beautiful seaside destination" }
                            }
                        }

                        <ElementTile> {
                            bg = { draw_bg: { color: #8e44ad } }
                            overlay = {
                                title = { text: "Mountain Retreat" }
                                subtitle = { text: "Peaceful mountain getaway" }
                            }
                        }
                    }

                    // 25. PricingCard
                    pricing_card_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "PricingCard" }

                        <View> { width: Fill, height: Fit, flow: Right, spacing: 16,
                            <ElementPricingCard> {
                                header = {
                                    plan_name = { text: "FREE" }
                                    price = { text: "$0" }
                                    period = { text: "forever" }
                                }
                                features = {
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ 1 Project" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Basic support" }
                                    <Label> { draw_text: { color: #757575, text_style: { font_size: 13.0 } } text: "✗ Custom domain" }
                                }
                                cta = { <ElementButtonOutline> { text: "Get Started" } }
                            }

                            <ElementPricingCard> {
                                header = {
                                    plan_name = { text: "PRO" }
                                    price = { text: "$29" }
                                }
                                features = {
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Unlimited Projects" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Priority support" }
                                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Custom domain" }
                                }
                                cta = { <ElementButtonSolid> { text: "Subscribe" } }
                            }
                        }
                    }

                    // 26. Skeleton
                    skeleton_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Skeleton" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Loading placeholder shapes with shimmer animation" }

                        <RoundedView> {
                            width: Fill, height: Fit, padding: 16,
                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 8.0 }
                            flow: Down, spacing: 12,

                            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                                <ElementSkeletonCircle> {}
                                <View> { width: Fill, height: Fit, flow: Down, spacing: 8,
                                    <ElementSkeletonLine> { width: 150 }
                                    <ElementSkeletonLine> { width: 100, height: 12 }
                                }
                            }
                            <ElementSkeletonRect> {}
                            <ElementSkeletonLine> {}
                            <ElementSkeletonLine> { width: 200 }
                        }
                    }

                    // 27. SpeedDial
                    speed_dial_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SpeedDial" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Speed dial with action items (expanded view)" }

                        <ElementSpeedDial> {
                            actions = {
                                <ElementSpeedDialAction> { text: "A" }
                                <ElementSpeedDialAction> { text: "B" }
                                <ElementSpeedDialAction> { text: "C" }
                            }
                        }
                    }

                    // 28. SocialIcon
                    social_icon_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SocialIcon" }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Brand social icons" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementSocialGithub> {}
                            <ElementSocialTwitter> {}
                            <ElementSocialDiscord> {}
                            <ElementSocialFacebook> {}
                            <ElementSocialLinkedin> {}
                            <ElementSocialYoutube> {}
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom social icons" }
                        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
                            <ElementSocialIcon> { draw_bg: { color: #dd4b39 } text: "G+" }
                            <ElementSocialIcon> { draw_bg: { color: #ff4500 } text: "R" }
                        }
                    }
                    // 29. Svg
                    svg_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Svg" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Full SVG rendering via resvg. Supports gradients, shapes, strokes, filters — any valid SVG." }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Gradient circle" }
                        <ElementSvg> {
                            text: "<svg xmlns='http://www.w3.org/2000/svg' width='80' height='80'><defs><radialGradient id='g'><stop offset='0%' stop-color='#2089dc'/><stop offset='100%' stop-color='#ca71eb'/></radialGradient></defs><circle cx='40' cy='40' r='38' fill='url(#g)'/></svg>"
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Stroked star" }
                        <ElementSvg> {
                            text: "<svg xmlns='http://www.w3.org/2000/svg' width='80' height='80' viewBox='0 0 24 24'><polygon points='12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26' fill='none' stroke='#faad14' stroke-width='1.5' stroke-linejoin='round'/></svg>"
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Complex path with opacity" }
                        <ElementSvg> {
                            text: "<svg xmlns='http://www.w3.org/2000/svg' width='100' height='60'><rect x='5' y='5' width='90' height='50' rx='12' fill='#2089dc' opacity='0.15' stroke='#2089dc' stroke-width='2'/><text x='50' y='36' text-anchor='middle' font-size='14' fill='#2089dc' font-family='sans-serif'>resvg</text></svg>"
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "GitHub-style contribution graph" }
                        <ElementSvg> {
                            text: "<svg xmlns='http://www.w3.org/2000/svg' width='160' height='40'><rect x='2' y='2' width='12' height='12' rx='2' fill='#ebedf0'/><rect x='18' y='2' width='12' height='12' rx='2' fill='#9be9a8'/><rect x='34' y='2' width='12' height='12' rx='2' fill='#40c463'/><rect x='50' y='2' width='12' height='12' rx='2' fill='#30a14e'/><rect x='66' y='2' width='12' height='12' rx='2' fill='#216e39'/><rect x='2' y='18' width='12' height='12' rx='2' fill='#9be9a8'/><rect x='18' y='18' width='12' height='12' rx='2' fill='#ebedf0'/><rect x='34' y='18' width='12' height='12' rx='2' fill='#40c463'/><rect x='50' y='18' width='12' height='12' rx='2' fill='#9be9a8'/><rect x='66' y='18' width='12' height='12' rx='2' fill='#ebedf0'/></svg>"
                        }
                    }

                    // 30. FadeView
                    fade_view_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "FadeView" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Cached view with configurable opacity. From makepad_wonderous." }

                        <View> { flow: Right, spacing: 16, height: Fit,
                            <ElementFadeView> {
                                width: 100, height: 80,
                                draw_bg: { opacity: 1.0 }
                                <RoundedView> { width: Fill, height: Fill, show_bg: true,
                                    draw_bg: { color: #2089dc, border_radius: 8.0 }
                                    align: { x: 0.5, y: 0.5 }
                                    <Label> { draw_text: { color: #fff } text: "100%" }
                                }
                            }
                            <ElementFadeView> {
                                width: 100, height: 80,
                                draw_bg: { opacity: 0.6 }
                                <RoundedView> { width: Fill, height: Fill, show_bg: true,
                                    draw_bg: { color: #2089dc, border_radius: 8.0 }
                                    align: { x: 0.5, y: 0.5 }
                                    <Label> { draw_text: { color: #fff } text: "60%" }
                                }
                            }
                            <ElementFadeView> {
                                width: 100, height: 80,
                                draw_bg: { opacity: 0.3 }
                                <RoundedView> { width: Fill, height: Fill, show_bg: true,
                                    draw_bg: { color: #2089dc, border_radius: 8.0 }
                                    align: { x: 0.5, y: 0.5 }
                                    <Label> { draw_text: { color: #fff } text: "30%" }
                                }
                            }
                        }
                    }

                    // 31. BlurView
                    blur_view_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "BlurView" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Gaussian blur shader with configurable amount. From makepad_wonderous." }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Single blur stage" }
                        <ElementBlurStage> {
                            width: 200, height: 100,
                            draw_bg: { blursize: 1.0, blurx: 10.0, blury: 10.0 }
                            <RoundedView> { width: Fill, height: Fill, show_bg: true,
                                draw_bg: { color: #e53935, border_radius: 12.0 }
                                align: { x: 0.5, y: 0.5 }
                                <Label> { draw_text: { color: #fff, text_style: { font_size: 16.0 } } text: "Blurred" }
                            }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "4-stage cascade (production blur)" }
                        <ElementBlurView> {
                            width: 200, height: 100,
                            <RoundedView> { width: Fill, height: Fill, show_bg: true,
                                draw_bg: { color: #2089dc, border_radius: 12.0 }
                                align: { x: 0.5, y: 0.5 }
                                <Label> { draw_text: { color: #fff, text_style: { font_size: 16.0 } } text: "Deep Blur" }
                            }
                        }
                    }

                    // 32. FadingBar
                    fading_bar_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "FadingBar" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Gradient bars that fade from transparent to opaque. From makepad_wonderous." }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Vertical fading bars" }
                        <View> { flow: Right, spacing: 16, height: 80,
                            <ElementFadingBarVertical> { width: 16, height: Fill,
                                draw_bg: { color: #2089dc, opacity: 1.0 } }
                            <ElementFadingBarVertical> { width: 16, height: Fill,
                                draw_bg: { color: #e53935, opacity: 1.0 } }
                            <ElementFadingBarVertical> { width: 16, height: Fill,
                                draw_bg: { color: #43a047, opacity: 1.0 } }
                            <ElementFadingBarVertical> { width: 16, height: Fill,
                                draw_bg: { color: #faad14, opacity: 1.0 } }
                        }

                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Horizontal fading bars" }
                        <View> { flow: Down, spacing: 8, width: 200,
                            <ElementFadingBarHorizontal> { width: Fill, height: 12,
                                draw_bg: { color: #2089dc, opacity: 1.0 } }
                            <ElementFadingBarHorizontal> { width: Fill, height: 12,
                                draw_bg: { color: #ca71eb, opacity: 0.8 } }
                            <ElementFadingBarHorizontal> { width: Fill, height: 12,
                                draw_bg: { color: #e53935, opacity: 0.6 } }
                        }
                    }

                    // 33. CurvedLabel (BUG: draw_abs does not render inside turtle context)
                    curved_label_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "CurvedLabel (WIP)" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Text arranged on a circular arc. Currently broken — see BUG in curved_label.rs." }
                    }

                    // 34. StaggeredGrid
                    staggered_grid_detail_page = <ScrollXYView> {
                        visible: false,
                        width: Fill, height: Fill, flow: Down,
                        padding: {left: 16, right: 16, top: 16, bottom: 16}, spacing: 16,

                        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
                            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "StaggeredGrid" }
                        <Label> { width: Fit, height: Fit,
                            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
                            text: "Pinterest-style masonry grid with variable-height items. From makepad_wonderous. (Requires Rust draw_walk override for dynamic content.)" }
                        <StaggeredGridDemo> {}
                    }

                }
                } // main_area
            }
        }
    }
}

app_main!(App);

fn main() {
    app_main();
}

// Tree node data
struct TreeNode {
    name: String,
    children: Option<Vec<TreeEdge>>,
}

struct TreeEdge {
    name: String,
    id: LiveId,
}

// StaggeredGridDemo: populates an ElementStaggeredGrid with colored boxes
#[derive(Live, LiveHook, Widget)]
pub struct StaggeredGridDemo {
    #[deref] view: View,
}

use makepad_element::components::staggered_grid::ElementStaggeredGrid;

impl Widget for StaggeredGridDemo {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut grid_ref) = item.borrow_mut::<ElementStaggeredGrid>() {
                let colors: [Vec4; 6] = [
                    vec4(0.126, 0.537, 0.863, 1.0),
                    vec4(0.898, 0.224, 0.208, 1.0),
                    vec4(0.263, 0.627, 0.278, 1.0),
                    vec4(0.980, 0.678, 0.078, 1.0),
                    vec4(0.612, 0.153, 0.690, 1.0),
                    vec4(0.000, 0.588, 0.533, 1.0),
                ];
                let heights: [f64; 12] = [60.0, 90.0, 45.0, 110.0, 70.0, 85.0, 50.0, 100.0, 65.0, 80.0, 55.0, 95.0];

                grid_ref.set_item_range(cx, 0, 12);
                while let Some(index) = grid_ref.next_visible_item(cx) {
                    if let Some((item_widget, _status)) = grid_ref.item(cx, index, live_id!(GridItem)) {
                        let color = colors[index % colors.len()];
                        item_widget.apply_over(cx, live! {
                            height: (heights[index % heights.len()]),
                            draw_bg: { color: (color) }
                        });
                        item_widget.label(ids!(item_label)).set_text(cx, &format!("Item {}", index));
                        item_widget.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

// ComponentTree: wraps FileTree with pre-populated component hierarchy
#[derive(Live, LiveHook, Widget)]
pub struct ComponentTree {
    #[wrap] #[live] file_tree: FileTree,
    #[rust] file_nodes: LiveIdMap<LiveId, TreeNode>,
    #[rust] built: bool,
}

impl ComponentTree {
    fn ensure_built(&mut self) {
        if self.built { return; }
        self.built = true;

        let mut nodes = LiveIdMap::default();

        let components: &[(&str, &[&str])] = &[
            ("Text", &["H1-H4", "Body", "Caption", "Overline"]),
            ("Markdown", &["Rich Text", "Code Block"]),
            ("Divider", &["Horizontal", "Inset", "Vertical"]),
            ("Icon", &["Small", "Medium", "Large", "Primary"]),
            ("Button", &["Solid", "Secondary", "Warning", "Outline", "Clear", "Error", "Success"]),
            ("Image", &["Standard", "Rounded", "Circle"]),
            ("Switch", &["Default", "Selected", "Labeled"]),
            ("CheckBox", &["Unchecked", "Checked", "Disabled", "Flat"]),
            ("Slider", &["Default", "Round", "Minimal", "Stepped"]),
            ("Input", &["Standard", "Flat", "Labeled", "Error", "Password"]),
            ("Card", &["Basic", "Titled", "Actions"]),
            ("Avatar", &["Small", "Medium", "Large", "Colors"]),
            ("Badge", &["Count", "Primary", "Success", "Dot"]),
            ("Chip", &["Solid", "Outline"]),
            ("Header", &["Primary", "Transparent"]),
            ("ListItem", &["Basic", "WithAvatar"]),
            ("Tab", &["TabBar"]),
            ("SearchBar", &["Default", "Custom"]),
            ("LinearProgress", &["25%", "50%", "75%", "100%", "Error"]),
            ("Rating", &["0 Stars", "1 Star", "2 Stars", "3 Stars", "4 Stars", "5 Stars"]),
            ("FAB", &["Small", "Default", "Extended", "Colors"]),
            ("Tooltip", &["Default", "Primary"]),
            ("Overlay", &["Backdrop"]),
            ("Dialog", &["Alert", "Confirm"]),
            ("Tile", &["Ocean", "Mountain"]),
            ("PricingCard", &["Free", "Pro"]),
            ("Skeleton", &["Line", "Circle", "Rect"]),
            ("SpeedDial", &["Actions"]),
            ("SocialIcon", &["Brand", "Custom"]),
            ("Svg", &["Gradient", "Stroke", "Complex"]),
            ("FadeView", &["100%", "60%", "30%"]),
            ("BlurView", &["Single Stage", "4-Stage Cascade"]),
            ("FadingBar", &["Vertical", "Horizontal"]),
            // CurvedLabel removed — BUG: draw_abs does not render inside turtle context
            ("StaggeredGrid", &["2-Column Masonry"]),
        ];

        let tiers: &[(&str, &[usize])] = &[
            ("Tier 1: Basic", &[0, 1, 2, 3, 4, 5]),
            ("Tier 2: Form", &[6, 7, 8, 9, 10]),
            ("Tier 3: Composite", &[11, 12, 13, 14, 15, 16, 17, 18, 19]),
            ("Tier 4: Advanced", &[20, 21, 22, 23, 24, 25, 26, 27, 28, 29]),
            ("Tier 5: Wonderous", &[30, 31, 32, 33]),
        ];

        let mut root_edges = Vec::new();

        for (tier_name, comp_indices) in tiers {
            let tier_id = LiveId::from_str(tier_name);
            let mut tier_edges = Vec::new();

            for &ci in *comp_indices {
                let (comp_name, children) = &components[ci];
                let comp_id = LiveId::from_str(comp_name);
                let mut child_edges = Vec::new();

                for child_name in *children {
                    let child_id = LiveId::from_str(&format!("{}/{}", comp_name, child_name));
                    child_edges.push(TreeEdge {
                        name: child_name.to_string(),
                        id: child_id,
                    });
                    nodes.insert(child_id, TreeNode {
                        name: child_name.to_string(),
                        children: None,
                    });
                }

                nodes.insert(comp_id, TreeNode {
                    name: comp_name.to_string(),
                    children: Some(child_edges),
                });

                tier_edges.push(TreeEdge {
                    name: comp_name.to_string(),
                    id: comp_id,
                });
            }

            nodes.insert(tier_id, TreeNode {
                name: tier_name.to_string(),
                children: Some(tier_edges),
            });

            root_edges.push(TreeEdge {
                name: tier_name.to_string(),
                id: tier_id,
            });
        }

        nodes.insert(live_id!(root), TreeNode {
            name: "Components".to_string(),
            children: Some(root_edges),
        });

        self.file_nodes = nodes;
    }

    fn draw_node(cx: &mut Cx2d, node_id: LiveId, ft: &mut FileTree, nodes: &LiveIdMap<LiveId, TreeNode>) {
        if let Some(node) = nodes.get(&node_id) {
            match &node.children {
                Some(children) => {
                    if ft.begin_folder(cx, node_id, &node.name).is_ok() {
                        for edge in children {
                            Self::draw_node(cx, edge.id, ft, nodes);
                        }
                        ft.end_folder();
                    }
                }
                None => {
                    ft.file(cx, node_id, &node.name);
                }
            }
        }
    }
}

impl Widget for ComponentTree {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.file_tree.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.ensure_built();
        while self.file_tree.draw_walk(cx, scope, walk).is_step() {
            // Open tier folders by default
            for tier_name in &["Tier 1: Basic", "Tier 2: Form", "Tier 3: Composite", "Tier 4: Advanced"] {
                let tier_id = LiveId::from_str(tier_name);
                self.file_tree.set_folder_is_open(cx, tier_id, true, Animate::No);
            }

            // Draw nodes
            if let Some(root) = self.file_nodes.get(&live_id!(root)) {
                if let Some(children) = &root.children {
                    for edge in children {
                        Self::draw_node(cx, edge.id, &mut self.file_tree, &self.file_nodes);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

// App
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    node_to_page: HashMap<LiveId, LiveId>,
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
    fn handle_startup(&mut self, _cx: &mut Cx) {
        self.build_page_map();
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let ft = self.ui.file_tree(ids!(component_tree));

        if let Some(file_id) = ft.file_clicked(actions) {
            self.show_page_for_node(cx, file_id);
        }
        if let Some(folder_id) = ft.folder_clicked(actions) {
            self.show_page_for_node(cx, folder_id);
        }
    }
}

impl App {
    fn build_page_map(&mut self) {
        let components: &[(&str, &str)] = &[
            ("Text", "text_detail_page"),
            ("Markdown", "markdown_detail_page"),
            ("Divider", "divider_detail_page"),
            ("Icon", "icon_detail_page"),
            ("Button", "button_detail_page"),
            ("Image", "image_detail_page"),
            ("Switch", "switch_detail_page"),
            ("CheckBox", "check_box_detail_page"),
            ("Slider", "slider_detail_page"),
            ("Input", "input_detail_page"),
            ("Card", "card_detail_page"),
            ("Avatar", "avatar_detail_page"),
            ("Badge", "badge_detail_page"),
            ("Chip", "chip_detail_page"),
            ("Header", "header_detail_page"),
            ("ListItem", "list_item_detail_page"),
            ("Tab", "tab_detail_page"),
            ("SearchBar", "search_bar_detail_page"),
            ("LinearProgress", "linear_progress_detail_page"),
            ("Rating", "rating_detail_page"),
            ("FAB", "fab_detail_page"),
            ("Tooltip", "tooltip_detail_page"),
            ("Overlay", "overlay_detail_page"),
            ("Dialog", "dialog_detail_page"),
            ("Tile", "tile_detail_page"),
            ("PricingCard", "pricing_card_detail_page"),
            ("Skeleton", "skeleton_detail_page"),
            ("SpeedDial", "speed_dial_detail_page"),
            ("SocialIcon", "social_icon_detail_page"),
            ("Svg", "svg_detail_page"),
            ("FadeView", "fade_view_detail_page"),
            ("BlurView", "blur_view_detail_page"),
            ("FadingBar", "fading_bar_detail_page"),
            ("StaggeredGrid", "staggered_grid_detail_page"),
        ];

        let variations: &[(&str, &[&str])] = &[
            ("Text", &["H1-H4", "Body", "Caption", "Overline"]),
            ("Markdown", &["Rich Text", "Code Block"]),
            ("Divider", &["Horizontal", "Inset", "Vertical"]),
            ("Icon", &["Small", "Medium", "Large", "Primary"]),
            ("Button", &["Solid", "Secondary", "Warning", "Outline", "Clear", "Error", "Success"]),
            ("Image", &["Standard", "Rounded", "Circle"]),
            ("Switch", &["Default", "Selected", "Labeled"]),
            ("CheckBox", &["Unchecked", "Checked", "Disabled", "Flat"]),
            ("Slider", &["Default", "Round", "Minimal", "Stepped"]),
            ("Input", &["Standard", "Flat", "Labeled", "Error", "Password"]),
            ("Card", &["Basic", "Titled", "Actions"]),
            ("Avatar", &["Small", "Medium", "Large", "Colors"]),
            ("Badge", &["Count", "Primary", "Success", "Dot"]),
            ("Chip", &["Solid", "Outline"]),
            ("Header", &["Primary", "Transparent"]),
            ("ListItem", &["Basic", "WithAvatar"]),
            ("Tab", &["TabBar"]),
            ("SearchBar", &["Default", "Custom"]),
            ("LinearProgress", &["25%", "50%", "75%", "100%", "Error"]),
            ("Rating", &["0 Stars", "1 Star", "2 Stars", "3 Stars", "4 Stars", "5 Stars"]),
            ("FAB", &["Small", "Default", "Extended", "Colors"]),
            ("Tooltip", &["Default", "Primary"]),
            ("Overlay", &["Backdrop"]),
            ("Dialog", &["Alert", "Confirm"]),
            ("Tile", &["Ocean", "Mountain"]),
            ("PricingCard", &["Free", "Pro"]),
            ("Skeleton", &["Line", "Circle", "Rect"]),
            ("SpeedDial", &["Actions"]),
            ("SocialIcon", &["Brand", "Custom"]),
            ("Svg", &["Gradient", "Stroke", "Complex"]),
            ("FadeView", &["100%", "60%", "30%"]),
            ("BlurView", &["Single Stage", "4-Stage Cascade"]),
            ("FadingBar", &["Vertical", "Horizontal"]),
            ("StaggeredGrid", &["2-Column Masonry"]),
        ];

        for (comp_name, page_name) in components {
            let comp_id = LiveId::from_str(comp_name);
            let page_id = LiveId::from_str(page_name);
            self.node_to_page.insert(comp_id, page_id);
        }

        // Map child variation nodes to the same page as parent
        for (comp_name, children) in variations {
            let page_name = components.iter().find(|(n, _)| n == comp_name).unwrap().1;
            let page_id = LiveId::from_str(page_name);
            for child_name in *children {
                let child_id = LiveId::from_str(&format!("{}/{}", comp_name, child_name));
                self.node_to_page.insert(child_id, page_id);
            }
        }
    }

    fn all_detail_page_ids() -> Vec<LiveId> {
        [
            "text_detail_page", "divider_detail_page", "icon_detail_page",
            "button_detail_page", "image_detail_page", "switch_detail_page",
            "check_box_detail_page", "slider_detail_page", "input_detail_page",
            "card_detail_page", "avatar_detail_page", "badge_detail_page",
            "chip_detail_page", "header_detail_page", "list_item_detail_page",
            "tab_detail_page", "search_bar_detail_page", "linear_progress_detail_page",
            "rating_detail_page", "fab_detail_page", "tooltip_detail_page",
            "overlay_detail_page", "dialog_detail_page", "tile_detail_page",
            "pricing_card_detail_page", "skeleton_detail_page", "speed_dial_detail_page",
            "social_icon_detail_page", "markdown_detail_page", "svg_detail_page",
            "fade_view_detail_page", "blur_view_detail_page", "fading_bar_detail_page",
            "staggered_grid_detail_page",
        ].iter().map(|s| LiveId::from_str(s)).collect()
    }

    fn show_page_for_node(&mut self, cx: &mut Cx, node_id: LiveId) {
        // Hide welcome and all detail pages
        self.ui.view(ids!(welcome_page)).set_visible(cx, false);
        for page_id in Self::all_detail_page_ids() {
            self.ui.view(&[page_id]).set_visible(cx, false);
        }

        // Show the matching page
        if let Some(&page_id) = self.node_to_page.get(&node_id) {
            self.ui.view(&[page_id]).set_visible(cx, true);
        }
    }
}
