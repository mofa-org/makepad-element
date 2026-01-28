use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::tooltip::*;

    pub TooltipDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tooltip" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Tooltip variants" }

        <ElementTooltip> { label = { text: "This is a tooltip" } }
        <ElementTooltip> { label = { text: "Another tooltip with longer text content" } }
        <ElementTooltip> { draw_bg: { color: #349ad9 } label = { text: "Primary tooltip" } }
    }
}
