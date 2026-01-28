use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element Markdown — themed wrapper around Makepad's Markdown widget
    pub ElementMarkdown = <Markdown> {
        width: Fill, height: Fit,

        draw_normal: {
            color: #212121,
        }
        draw_italic: {
            color: #212121,
        }
        draw_bold: {
            color: #111111,
        }
        draw_bold_italic: {
            color: #111111,
        }
    }
}
