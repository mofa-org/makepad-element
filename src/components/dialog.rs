use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Dialog card (used inside an overlay)
    pub ElementDialog = <RoundedView> {
        width: 320, height: Fit,
        padding: 20,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 3.0,
        }

        flow: Down,
        spacing: 16,

        title = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 18.0 }
            }
            text: "Dialog Title"
        }

        body = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #8693a0,
                text_style: { font_size: 14.0 }
                wrap: Word,
            }
            text: "Dialog body text goes here."
        }

        // RNE: marginTop 10, flexDirection row-reverse, justifyContent flex-start
        actions = <View> {
            width: Fill, height: Fit,
            margin: {top: 10},
            flow: Right,
            spacing: 8,
            align: {x: 1.0},
        }
    }
}
