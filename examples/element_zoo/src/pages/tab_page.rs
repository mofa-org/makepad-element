use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::tab::*;

    pub TabDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tab" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Tab bar with radio buttons" }

        <View> {
            width: Fill, height: Fit,
            flow: Right,
            show_bg: true,
            draw_bg: { color: #ffffff }

            <ElementTabItem> { text: "Tab 1", radio_group: demo_tabs, animator: { selected = { default: on } } }
            <ElementTabItem> { text: "Tab 2", radio_group: demo_tabs }
            <ElementTabItem> { text: "Tab 3", radio_group: demo_tabs }
        }
    }
}
