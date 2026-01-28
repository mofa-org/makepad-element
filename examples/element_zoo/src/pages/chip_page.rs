use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::chip::*;

    pub ChipDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Chip" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Solid chips" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
            <ElementChip> { label = { text: "Default" } }
            <ElementChip> { draw_bg: { color: #30cc71 } label = { text: "Success" } }
            <ElementChip> { draw_bg: { color: #e74c3c } label = { text: "Error" } }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Outline chips" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 8,
            <ElementChipOutline> { label = { text: "Outline" } }
            <ElementChipOutline> { draw_bg: { border_color: #30cc71 } label = { draw_text: { color: #30cc71 } text: "Success" } }
        }
    }
}
