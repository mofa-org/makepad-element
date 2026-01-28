use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Solid chip — pill shape via clamped border radius
    pub ElementChip = <View> {
        width: Fit, height: Fit,
        padding: {left: 12, right: 12, top: 6, bottom: 6},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #2089dc,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = min(self.rect_size.x, self.rect_size.y) * 0.25;
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, max(1.0, r));
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }

        flow: Right,
        spacing: 6,

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 14.0 }
            }
            text: "Chip"
        }
    }

    // Outline chip — pill shape with smooth anti-aliased border
    pub ElementChipOutline = <View> {
        width: Fit, height: Fit,
        padding: {left: 12, right: 12, top: 6, bottom: 6},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #ffffff,
            instance border_color: #2089dc,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 1.5;
                let hw = bw * 0.5;
                let w = self.rect_size.x - bw;
                let h = self.rect_size.y - bw;
                let r = min(w, h) * 0.25;
                sdf.box(hw, hw, w, h, max(1.0, r));
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }

        flow: Right,
        spacing: 6,

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #2089dc,
                text_style: { font_size: 14.0 }
            }
            text: "Chip"
        }
    }
}
