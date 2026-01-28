use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element themed toggle switch with RNE primary color
    pub ElementSwitch = <Toggle> {
        width: Fit,
        height: Fit,
        text: ""
        draw_bg: {
            color_active: #2089dc,
        }
    }

    // Switch with label
    pub ElementSwitchLabeled = <Toggle> {
        width: Fit,
        height: Fit,
        draw_bg: {
            color_active: #2089dc,
        }
    }
}
