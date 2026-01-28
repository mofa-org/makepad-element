use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::overlay::*;

    pub OverlayDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Overlay" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Overlay backdrop preview (scaled down)" }

        <View> {
            width: 300, height: 200,
            flow: Overlay,
            align: {x: 0.5, y: 0.5},

            <View> {
                width: Fill, height: Fill,
                show_bg: true,
                draw_bg: { color: #00000044 }
            }
            <RoundedView> {
                width: 200, height: 100,
                padding: 16,
                align: {x: 0.5, y: 0.5},
                show_bg: true,
                draw_bg: { color: #ffffff, border_radius: 8.0 }
                <Label> { draw_text: { color: #333333, text_style: { font_size: 14.0 } } text: "Overlay content" }
            }
        }
    }
}
