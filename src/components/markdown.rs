use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element Markdown — themed wrapper around Makepad's Markdown widget
    pub ElementMarkdown = <Markdown> {
        width: Fill, height: Fit,

        // Base font color for the entire markdown
        font_color: #212121,

        draw_normal: {
            color: #212121,
        }
        draw_italic: {
            color: #333333,
        }
        draw_bold: {
            color: #111111,
        }
        draw_bold_italic: {
            color: #111111,
        }
        draw_fixed: {
            color: #212121,
        }
        draw_block: {
            line_color: #333333,
            sep_color: #cccccc,
            quote_bg_color: #f5f5f5,
            quote_fg_color: #397af8,
            code_color: #f0f0f0,
        }
    }
}
