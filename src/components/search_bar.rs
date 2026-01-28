use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Search bar (RNE: padding 8, borderTop/Bottom 1px #e1e1e1, input borderRadius 3, minHeight 30)
    // Shorter width, taller height for comfortable touch targets
    pub ElementSearchBar = <View> {
        width: Fill, height: 52,
        padding: {left: 12, right: 12, top: 8, bottom: 8},
        flow: Right,
        spacing: 8,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #ffffff,
            instance border_color: #e1e1e1,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill_keep(self.bg_color);
                // Top and bottom borders
                if self.pos.y < 1.0 / self.rect_size.y {
                    return self.border_color;
                }
                if self.pos.y > 1.0 - 1.0 / self.rect_size.y {
                    return self.border_color;
                }
                return sdf.result;
            }
        }

        // Search icon (SVG)
        search_icon = <Icon> {
            icon_walk: { width: 18, height: 18 }
            draw_icon: {
                svg_file: dep("crate://makepad-element/resources/icons/search.svg"),
                color: #8693a0,
            }
        }

        // Input container (RNE: borderRadius 3, minHeight 30, marginLeft 10)
        input_container = <RoundedView> {
            width: Fill, height: Fit,
            margin: {left: 6},
            show_bg: true,
            draw_bg: {
                color: #e1e8ee,
                border_radius: 3.0,
            }

            input = <TextInput> {
                width: Fill, height: Fit,
                empty_text: "Search..."
                draw_bg: {
                    color: #e1e8ee00,
                }
                draw_text: {
                    color: #333333,
                    text_style: { font_size: 14.0 }
                }
            }
        }
    }
}
