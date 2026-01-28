use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Floating action button (RNE: default color = secondary #ad1457, large 56x56 padding 16, borderRadius 28)
    pub ElementFab = <Button> {
        width: 56, height: 56,
        padding: 16,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #ad1457,
            instance color_hover: #8e1148,
            instance color_down: #6e0d38,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let color = mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down, self.down
                );
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

    // Small FAB (RNE: 40x40 padding 8)
    pub ElementFabSmall = <Button> {
        width: 40, height: 40,
        padding: 8,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #ad1457,
            instance color_hover: #8e1148,

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
            text_style: { font_size: 18.0 }
        }
        text: "+"
    }

    // Extended FAB (with text, RNE: large height 48 padding 16, borderRadius 28)
    pub ElementFabExtended = <Button> {
        width: Fit, height: 48,
        padding: {left: 16, right: 16, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #ad1457,
            instance color_hover: #8e1148,
            instance border_radius: 28.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 14.0 }
        }
        text: "+ Create"
    }
}
