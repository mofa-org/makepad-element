use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Standard image
    pub ElementImage = <Image> {
        width: 200,
        height: 150,
        fit: Smallest,
    }

    // Rounded image
    pub ElementImageRounded = <View> {
        width: 200,
        height: 150,
        show_bg: true,
        draw_bg: {
            instance border_radius: 12.0,
            instance color: #E0E0E0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        <Image> {
            width: Fill,
            height: Fill,
            fit: Smallest,
        }
    }

    // Circle image (avatar-style)
    pub ElementImageCircle = <View> {
        width: 100,
        height: 100,
        show_bg: true,
        draw_bg: {
            instance color: #E0E0E0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        <Image> {
            width: Fill,
            height: Fill,
            fit: Smallest,
        }
    }
}
