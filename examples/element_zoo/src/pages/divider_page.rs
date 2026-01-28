use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::divider::*;

    pub DividerDetailPage = <ScrollXYView> {
        width: Fill,
        height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Divider"
        }

        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Horizontal divider"
        }
        <ElementDivider> {}

        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Divider with inset"
        }
        <ElementDividerInset> {}

        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Vertical divider (in a row)"
        }
        <View> {
            width: Fill, height: 60,
            flow: Right,
            align: {y: 0.5},
            spacing: 16,

            <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                text: "Left"
            }
            <ElementDividerVertical> {}
            <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                text: "Right"
            }
        }
    }
}
