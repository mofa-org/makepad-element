use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element themed checkbox with RNE primary color
    pub ElementCheckBox = <CheckBox> {
        width: Fit,
        height: Fit,
        draw_bg: {
            color_active: #2089dc,
            mark_color_active: #ffffff,
            mark_color_active_hover: #ffffff,
        }
    }

    // Flat style checkbox
    pub ElementCheckBoxFlat = <CheckBoxFlat> {
        width: Fit,
        height: Fit,
        draw_bg: {
            color_active: #2089dc,
            mark_color_active: #ffffff,
            mark_color_active_hover: #ffffff,
        }
    }
}
