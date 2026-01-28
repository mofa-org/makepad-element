use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Tile: image background with text overlay
    pub ElementTile = <View> {
        width: Fill, height: 200,
        flow: Overlay,

        // Background placeholder
        bg = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: { color: #555555 }
        }

        // Gradient overlay + text
        overlay = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let gradient = mix(
                        vec4(0., 0., 0., 0.),
                        vec4(0., 0., 0., 0.7),
                        self.pos.y
                    );
                    return gradient;
                }
            }

            // RNE: padding top 15, bottom 5, left/right 15
            align: {x: 0.0, y: 1.0},
            padding: {left: 15, right: 15, top: 15, bottom: 5},
            flow: Down,
            spacing: 4,

            title = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
                text: "Tile Title"
            }
            subtitle = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #ffffffcc, text_style: { font_size: 13.0 } }
                text: "Subtitle text"
            }
        }
    }
}
