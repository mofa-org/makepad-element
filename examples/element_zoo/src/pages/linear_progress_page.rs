use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::linear_progress::*;

    pub LinearProgressDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "LinearProgress" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "25% progress" }
        <ElementLinearProgress> { draw_bg: { progress: 0.25 } }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "50% progress" }
        <ElementLinearProgress> { draw_bg: { progress: 0.50 } }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "75% progress (thick)" }
        <ElementLinearProgressThick> { draw_bg: { progress: 0.75 } }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "100% success color" }
        <ElementLinearProgressThick> { draw_bg: { progress: 1.0, fill_color: #30cc71 } }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Error color" }
        <ElementLinearProgressThick> { draw_bg: { progress: 0.33, fill_color: #e74c3c } }
    }
}
