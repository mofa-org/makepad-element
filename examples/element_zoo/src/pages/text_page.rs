use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::text::*;

    pub TextDetailPage = <ScrollXYView> {
        width: Fill,
        height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 8,

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: {
                color: #333333,
                text_style: { font_size: 24.0 }
            }
            text: "Text"
        }

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Typography variants for headings, body, and caption text."
        }

        // Heading variants
        <ElementH1> { text: "Heading 1 (32px)" }
        <ElementH2> { text: "Heading 2 (24px)" }
        <ElementH3> { text: "Heading 3 (20px)" }
        <ElementH4> { text: "Heading 4 (16px)" }
        <ElementBody> { text: "Body text (14px) - The quick brown fox jumps over the lazy dog." }
        <ElementCaption> { text: "Caption text (12px) - Secondary information" }
        <ElementOverline> { text: "OVERLINE TEXT (10px)" }
    }
}
