use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // A view that renders its children into a cached texture with configurable opacity.
    // Extracted from makepad_wonderous FadeView.
    pub ElementFadeView = <CachedView> {
        draw_bg: {
            instance opacity: 1.0

            fn pixel(self) -> vec4 {
                let color = sample2d_rt(self.image, self.pos * self.scale + self.shift);
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }
}
