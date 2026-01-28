use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Full-screen overlay backdrop (RNE: backdrop rgba(0,0,0,0.4), content padding 10, borderRadius 3)
    pub ElementOverlay = <View> {
        width: Fill, height: Fill,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},

        // Backdrop
        backdrop = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: { color: #00000066 }
        }

        // Content slot with RNE overlay card styling
        content = <RoundedView> {
            width: Fit, height: Fit,
            padding: 10,
            show_bg: true,
            draw_bg: {
                color: #ffffff,
                border_radius: 3.0,
            }
        }
    }
}
