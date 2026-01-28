use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // App bar / header (RNE: paddingH 10, paddingV 10, borderBottomColor #f2f2f2)
    pub ElementHeader = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: 56,
            flow: Right,
            padding: {left: 10, right: 10, top: 10, bottom: 10},
            align: {y: 0.5},

            show_bg: true,
            draw_bg: { color: #2089dc }

            left = <View> {
                width: Fit, height: Fit,
            }

            // RNE: flex 3, paddingH 15
            center = <View> {
                width: Fill, height: Fit,
                padding: {left: 15, right: 15},
                align: {x: 0.5},
                title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #ffffff,
                        text_style: { font_size: 18.0 }
                    }
                    text: "Title"
                }
            }

            right = <View> {
                width: Fit, height: Fit,
            }
        }

        // Bottom border line
        <View> {
            width: Fill, height: 1,
            show_bg: true,
            draw_bg: { color: #f2f2f2 }
        }
    }

    // Transparent header
    pub ElementHeaderTransparent = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: 56,
            flow: Right,
            padding: {left: 10, right: 10, top: 10, bottom: 10},
            align: {y: 0.5},

            left = <View> {
                width: Fit, height: Fit,
            }

            // RNE: flex 3, paddingH 15
            center = <View> {
                width: Fill, height: Fit,
                padding: {left: 15, right: 15},
                align: {x: 0.5},
                title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #333333,
                        text_style: { font_size: 18.0 }
                    }
                    text: "Title"
                }
            }

            right = <View> {
                width: Fit, height: Fit,
            }
        }

        // Bottom border line
        <View> {
            width: Fill, height: 1,
            show_bg: true,
            draw_bg: { color: #f2f2f2 }
        }
    }
}
