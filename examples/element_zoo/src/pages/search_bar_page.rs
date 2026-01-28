use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::search_bar::*;

    pub SearchBarDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SearchBar" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Default search bar" }
        <ElementSearchBar> {}

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom placeholder" }
        <ElementSearchBar> {
            input = { empty_text: "Search products..." }
        }
    }
}
