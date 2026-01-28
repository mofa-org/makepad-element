use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Small avatar (34x34) - title fontSize = width/2 = 17
    pub ElementAvatarSmall = <View> {
        width: 34, height: 34,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},

        text_view = <View> {
            width: Fill, height: Fill,
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
            label = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 17.0 } }
                text: "A"
            }
        }
    }

    // Medium avatar (50x50) - default - title fontSize = width/2 = 25
    pub ElementAvatar = <View> {
        width: 50, height: 50,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},

        text_view = <View> {
            width: Fill, height: Fill,
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
            label = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 25.0 } }
                text: "A"
            }
        }
    }

    // Large avatar (75x75) - title fontSize = width/2 = 37.5
    pub ElementAvatarLarge = <View> {
        width: 75, height: 75,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},

        text_view = <View> {
            width: Fill, height: Fill,
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
            label = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 37.5 } }
                text: "A"
            }
        }
    }

    // Extra large avatar (150x150) - title fontSize = width/2 = 75
    pub ElementAvatarXLarge = <View> {
        width: 150, height: 150,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},

        text_view = <View> {
            width: Fill, height: Fill,
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
            label = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 75.0 } }
                text: "A"
            }
        }
    }
}
