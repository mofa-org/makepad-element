use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Skeleton line placeholder with shimmer
    // RNE: height 12, borderRadius 2, bg = grey4 (#bdc6cf)
    pub ElementSkeletonLine = <View> {
        width: Fill, height: 12,
        show_bg: true,
        draw_bg: {
            instance bg_color: #bdc6cf,
            instance highlight_color: #f5f5f5,
            instance border_radius: 2.0,
            instance shimmer_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                // Shimmer band
                let band_center = self.shimmer_pos * 1.4 - 0.2;
                let dist = abs(self.pos.x - band_center);
                let band = smoothstep(0.15, 0.0, dist);
                let color = mix(self.bg_color, self.highlight_color, band);
                sdf.fill(color);
                return sdf.result;
            }
        }
        animator: {
            shimmer = {
                default: on,
                on = {
                    from: {all: Loop {duration: 1.5, end: 1.0}}
                    apply: {
                        draw_bg: { shimmer_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }

    // Skeleton circle (avatar placeholder) with shimmer
    pub ElementSkeletonCircle = <View> {
        width: 48, height: 48,
        show_bg: true,
        draw_bg: {
            instance bg_color: #bdc6cf,
            instance highlight_color: #f5f5f5,
            instance shimmer_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x);
                let band_center = self.shimmer_pos * 1.4 - 0.2;
                let dist = abs(self.pos.x - band_center);
                let band = smoothstep(0.15, 0.0, dist);
                let color = mix(self.bg_color, self.highlight_color, band);
                sdf.fill(color);
                return sdf.result;
            }
        }
        animator: {
            shimmer = {
                default: on,
                on = {
                    from: {all: Loop {duration: 1.5, end: 1.0}}
                    apply: {
                        draw_bg: { shimmer_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }

    // Skeleton rectangle (image placeholder) with shimmer
    pub ElementSkeletonRect = <View> {
        width: Fill, height: 120,
        show_bg: true,
        draw_bg: {
            instance bg_color: #bdc6cf,
            instance highlight_color: #f5f5f5,
            instance border_radius: 2.0,
            instance shimmer_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                let band_center = self.shimmer_pos * 1.4 - 0.2;
                let dist = abs(self.pos.x - band_center);
                let band = smoothstep(0.15, 0.0, dist);
                let color = mix(self.bg_color, self.highlight_color, band);
                sdf.fill(color);
                return sdf.result;
            }
        }
        animator: {
            shimmer = {
                default: on,
                on = {
                    from: {all: Loop {duration: 1.5, end: 1.0}}
                    apply: {
                        draw_bg: { shimmer_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }
}
