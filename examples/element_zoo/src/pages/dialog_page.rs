use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::dialog::*;
    use makepad_element::components::button::*;

    pub DialogDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Dialog" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Alert dialog" }
        <ElementDialog> {
            title = { text: "Alert" }
            body = { text: "Something important happened. Please acknowledge." }
            actions = {
                <ElementButtonSolid> { text: "OK" }
            }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Confirm dialog" }
        <ElementDialog> {
            title = { text: "Confirm" }
            body = { text: "Are you sure you want to proceed with this action?" }
            actions = {
                <ElementButtonClear> { text: "Cancel" }
                <ElementButtonSolid> { text: "Confirm" }
            }
        }
    }
}
