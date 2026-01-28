use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Basic list item (RNE: padding 14/16, bg = white, hairline border)
    pub ElementListItem = <View> {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 14, bottom: 14},
        flow: Right,
        spacing: 12,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: { color: #ffffff }

        content = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 2,

            title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #242424,
                    text_style: { font_size: 14.0 }
                }
                text: "List Item"
            }

            subtitle = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #8693a0,
                    text_style: { font_size: 12.0 }
                }
                text: "Subtitle"
            }
        }

        // Chevron
        chevron = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #bdc6cf,
                text_style: { font_size: 16.0 }
            }
            text: "›"
        }
    }

    // List item with left avatar area
    pub ElementListItemAvatar = <View> {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 14, bottom: 14},
        flow: Right,
        spacing: 12,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: { color: #ffffff }

        avatar = <View> {
            width: 40, height: 40,
            align: {x: 0.5, y: 0.5},
            show_bg: true,
            draw_bg: {
                instance bg_color: #2089dc,
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, c.x);
                    sdf.fill(self.bg_color);
                    return sdf.result;
                }
            }
            <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 14.0 } }
                text: "A"
            }
        }

        content = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 2,

            title = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #242424, text_style: { font_size: 14.0 } }
                text: "List Item"
            }

            subtitle = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #8693a0, text_style: { font_size: 12.0 } }
                text: "Subtitle"
            }
        }

        chevron = <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #bdc6cf, text_style: { font_size: 16.0 } }
            text: "›"
        }
    }
}
