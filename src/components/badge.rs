use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Badge - small count indicator (RNE: size 18, borderRadius 9, fontSize 12, paddingH 4)
    // Uses pill shape with white hairline border, radius clamped to avoid diamond artifact
    pub ElementBadge = <View> {
        width: Fit, height: 18,
        padding: {left: 4, right: 4, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #ff190c,
            instance border_color: #ffffff,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 0.5;
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

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "1"
        }
    }

    // Badge dot (no text) - RNE mini size 8
    pub ElementBadgeDot = <View> {
        width: 8, height: 8,
        show_bg: true,
        draw_bg: {
            instance bg_color: #ff190c,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // Primary badge
    pub ElementBadgePrimary = <View> {
        width: Fit, height: 18,
        padding: {left: 4, right: 4, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #2089dc,
            instance border_color: #ffffff,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 0.5;
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

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "1"
        }
    }

    // Success badge
    pub ElementBadgeSuccess = <View> {
        width: Fit, height: 18,
        padding: {left: 4, right: 4, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #52c41a,
            instance border_color: #ffffff,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 0.5;
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

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "1"
        }
    }
}
