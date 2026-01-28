use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Tooltip container (dark background)
    // RNE: height 40, width 150, backgroundColor rgba(97,112,128,1.0)
    pub ElementTooltip = <RoundedView> {
        width: 150, height: 40,
        padding: {left: 12, right: 12, top: 8, bottom: 8},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            color: #617080,
            border_radius: 4.0,
        }

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "Tooltip text"
        }
    }
}
