use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Standard input with Element styling
    pub ElementInput = <TextInput> {
        width: Fill,
        height: Fit,
        empty_text: "Enter text..."
        draw_text: {
            color: #333333,
        }
    }

    // Flat style input
    pub ElementInputFlat = <TextInputFlat> {
        width: Fill,
        height: Fit,
        empty_text: "Enter text..."
        draw_text: {
            color: #333333,
        }
    }

    // Input with label (RNE: label fontSize 16 bold, label color grey3 #8693a0, input fontSize 18)
    pub ElementInputLabeled = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4,
        padding: {left: 10, right: 10},

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #8693a0,
                text_style: { font_size: 16.0 }
            }
            text: "Label"
        }

        input = <TextInput> {
            width: Fill,
            height: Fit,
            empty_text: "Enter text..."
            draw_text: {
                color: #333333,
            }
        }
    }

    // Input with error message (RNE: error fontSize 12, error color = error)
    pub ElementInputError = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4,
        padding: {left: 10, right: 10},

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #8693a0,
                text_style: { font_size: 16.0 }
            }
            text: "Label"
        }

        input = <TextInput> {
            width: Fill,
            height: Fit,
            empty_text: "Enter text..."
            draw_text: {
                color: #333333,
            }
            draw_bg: {
                border_color: #ff190c,
            }
        }

        error = <Label> {
            width: Fit, height: Fit,
            margin: 5,
            draw_text: {
                color: #ff190c,
                text_style: { font_size: 12.0 }
            }
            text: "Error message"
        }
    }

    // Password input
    pub ElementInputPassword = <TextInput> {
        width: Fill,
        height: Fit,
        is_password: true,
        empty_text: "Password"
        draw_text: {
            color: #333333,
        }
    }
}
