use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Linear progress bar (RNE: height 4, track = fill color at 0.4 opacity)
    pub ElementLinearProgress = <View> {
        width: Fill, height: 4,
        show_bg: true,
        draw_bg: {
            instance progress: 0.5,
            instance fill_color: #2089dc,
            instance border_radius: 2.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Background track (fill color at 0.4 opacity)
                let track_color = vec4(self.fill_color.x, self.fill_color.y, self.fill_color.z, 0.4);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(track_color);
                // Fill
                let fill_width = self.rect_size.x * self.progress;
                if fill_width > 0.0 {
                    sdf.box(0., 0., fill_width, self.rect_size.y, self.border_radius);
                    sdf.fill(self.fill_color);
                }
                return sdf.result;
            }
        }
    }

    // Thicker progress bar
    pub ElementLinearProgressThick = <View> {
        width: Fill, height: 8,
        show_bg: true,
        draw_bg: {
            instance progress: 0.5,
            instance fill_color: #2089dc,
            instance border_radius: 4.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let track_color = vec4(self.fill_color.x, self.fill_color.y, self.fill_color.z, 0.4);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(track_color);
                let fill_width = self.rect_size.x * self.progress;
                if fill_width > 0.0 {
                    sdf.box(0., 0., fill_width, self.rect_size.y, self.border_radius);
                    sdf.fill(self.fill_color);
                }
                return sdf.result;
            }
        }
    }

    // Indeterminate progress bar (RNE: animation duration 2000ms)
    pub ElementLinearProgressIndeterminate = <View> {
        width: Fill, height: 4,
        show_bg: true,
        draw_bg: {
            instance fill_color: #2089dc,
            instance border_radius: 2.0,
            instance anim_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Background track (fill at 0.4 opacity)
                let track_color = vec4(self.fill_color.x, self.fill_color.y, self.fill_color.z, 0.4);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(track_color);
                // Sliding bar (30% width)
                let bar_w = self.rect_size.x * 0.3;
                let bar_x = (self.anim_pos * 1.6 - 0.3) * self.rect_size.x;
                sdf.box(bar_x, 0., bar_w, self.rect_size.y, self.border_radius);
                sdf.fill(self.fill_color);
                return sdf.result;
            }
        }
        animator: {
            indeterminate = {
                default: on,
                on = {
                    from: {all: Loop {duration: 2.0, end: 1.0}}
                    apply: {
                        draw_bg: { anim_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }
}
