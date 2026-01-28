use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::fab::*;

    pub FabDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "FAB" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Floating Action Buttons" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementFabSmall> {}
            <ElementFab> {}
            <ElementFabExtended> {}
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom colors" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementFab> { draw_bg: { color: #e74c3c, color_hover: #c0392b } }
            <ElementFab> { draw_bg: { color: #30cc71, color_hover: #28ae60 } }
        }
    }
}
