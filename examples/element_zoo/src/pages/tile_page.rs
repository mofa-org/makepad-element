use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::tile::*;

    pub TileDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Tile" }

        <ElementTile> {
            bg = { draw_bg: { color: #3498db } }
            overlay = {
                title = { text: "Ocean View" }
                subtitle = { text: "Beautiful seaside destination" }
            }
        }

        <ElementTile> {
            bg = { draw_bg: { color: #8e44ad } }
            overlay = {
                title = { text: "Mountain Retreat" }
                subtitle = { text: "Peaceful mountain getaway" }
            }
        }
    }
}
