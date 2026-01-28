use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Horizontal divider (RNE: rgba(0,0,0,0.12))
    pub ElementDivider = <View> {
        width: Fill,
        height: 1.0,
        show_bg: true,
        draw_bg: {
            color: #0000001f,
        }
    }

    // Vertical divider
    pub ElementDividerVertical = <View> {
        width: 1.0,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: #0000001f,
        }
    }

    // Divider with inset (left margin 72)
    pub ElementDividerInset = <View> {
        width: Fill,
        height: 1.0,
        margin: {left: 72},
        show_bg: true,
        draw_bg: {
            color: #0000001f,
        }
    }
}
