use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::button::*;

    pub ButtonDetailPage = <ScrollXYView> {
        width: Fill,
        height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Button"
        }

        // Solid variants
        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Solid buttons"
        }
        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 12,

            <ElementButtonSolid> { text: "Primary" }
            <ElementButtonSuccess> { text: "Success" }
            <ElementButtonError> { text: "Error" }
        }

        // Outline variant
        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Outline button"
        }
        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 12,

            <ElementButtonOutline> { text: "Outline" }
        }

        // Clear variant
        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Clear/text button"
        }
        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 12,

            <ElementButtonClear> { text: "Clear Button" }
        }

        // Disabled
        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Disabled button"
        }
        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 12,

            <ElementButtonSolid> {
                text: "Disabled"
                animator: { disabled = { default: on } }
            }
        }
    }
}
