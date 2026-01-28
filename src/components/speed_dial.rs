use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Speed dial action item (small FAB)
    pub ElementSpeedDialAction = <Button> {
        width: 40, height: 40,
        padding: 0,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #8693a0,
            instance color_hover: #5e6977,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.circle(c.x, c.y, c.x);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 14.0 }
        }
    }

    // Speed dial container (vertical stack of actions + main FAB)
    pub ElementSpeedDial = <View> {
        width: Fit, height: Fit,
        flow: Down,
        spacing: 12,
        align: {x: 0.5},

        // RNE: action marginBottom 16
        actions = <View> {
            width: Fit, height: Fit,
            flow: Down,
            spacing: 16,
            align: {x: 0.5},
        }

        // RNE: margin 16, marginTop 0
        main_button = <Button> {
            width: 56, height: 56,
            padding: 0,
            margin: {left: 16, right: 16, bottom: 16},
            align: {x: 0.5, y: 0.5},

            draw_bg: {
                instance color: #2089dc,
                instance color_hover: #1975be,

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let color = mix(self.color, self.color_hover, self.hover);
                    sdf.circle(c.x, c.y, c.x);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 24.0 }
            }
            text: "+"
        }
    }
}
