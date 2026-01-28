use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::input::*;

    pub InputDetailPage = <ScrollXYView> {
        width: Fill, height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Input"
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Standard input"
        }
        <ElementInput> {}

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Flat input"
        }
        <ElementInputFlat> {}

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Labeled input"
        }
        <ElementInputLabeled> {
            label = { text: "Email" }
            input = { empty_text: "you@example.com" }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Input with error"
        }
        <ElementInputError> {
            label = { text: "Username" }
            input = { empty_text: "Required" }
            error = { text: "This field is required" }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Password input"
        }
        <ElementInputPassword> {}
    }
}
