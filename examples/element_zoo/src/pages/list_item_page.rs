use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::list_item::*;
    use makepad_element::components::divider::*;

    pub ListItemDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 0,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "ListItem" }

        <ElementListItem> {
            content = { title = { text: "Simple Item" } subtitle = { text: "Description text" } }
        }
        <ElementDivider> {}
        <ElementListItem> {
            content = { title = { text: "Another Item" } subtitle = { text: "More details here" } }
        }
        <ElementDivider> {}
        <ElementListItemAvatar> {
            content = { title = { text: "John Doe" } subtitle = { text: "john@example.com" } }
        }
        <ElementDivider> {}
        <ElementListItemAvatar> {
            avatar = { draw_bg: { color: #30cc71 } <Label> { draw_text: { color: #ffffff, text_style: { font_size: 14.0 } } text: "B" } }
            content = { title = { text: "Bob Smith" } subtitle = { text: "bob@example.com" } }
        }
    }
}
