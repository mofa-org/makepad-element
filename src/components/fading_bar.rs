use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Vertical bar that fades from transparent (top) to opaque (bottom).
    // Extracted from makepad_wonderous VerticalFadingBar.
    pub ElementFadingBarVertical = <View> {
        width: 10.0,
        height: 50.0,
        show_bg: true,
        draw_bg: {
            color: #ffffff
            instance opacity: 0.8

            fn pixel(self) -> vec4 {
                let alpha = mix(0.0, self.opacity, self.pos.y);
                return vec4(self.color.rgb * alpha, alpha);
            }
        }
    }

    // Horizontal bar that fades from transparent (left) to opaque (right).
    pub ElementFadingBarHorizontal = <View> {
        width: 50.0,
        height: 10.0,
        show_bg: true,
        draw_bg: {
            color: #ffffff
            instance opacity: 0.8

            fn pixel(self) -> vec4 {
                let alpha = mix(0.0, self.opacity, self.pos.x);
                return vec4(self.color.rgb * alpha, alpha);
            }
        }
    }
}
