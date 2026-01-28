use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::speed_dial::*;

    pub SpeedDialDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SpeedDial" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Speed dial with action items (expanded view)" }

        <ElementSpeedDial> {
            actions = {
                <ElementSpeedDialAction> { text: "A" }
                <ElementSpeedDialAction> { text: "B" }
                <ElementSpeedDialAction> { text: "C" }
            }
        }
    }
}
