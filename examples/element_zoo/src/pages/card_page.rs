use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::card::*;
    use makepad_element::components::divider::*;
    use makepad_element::components::button::*;

    pub CardDetailPage = <ScrollXYView> {
        width: Fill, height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Card"
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Basic card"
        }
        <ElementCard> {
            <Label> {
                draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                text: "This is a basic card with some content inside."
            }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Card with title"
        }
        <ElementCardTitled> {
            header = { title = { text: "Featured" } }
            content = {
                <Label> {
                    draw_text: { color: #424242, text_style: { font_size: 14.0 } }
                    text: "Card content with a title header and divider."
                }
            }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Card with actions"
        }
        <ElementCard> {
            <Label> {
                draw_text: { color: #333333, text_style: { font_size: 16.0 } }
                text: "Action Card"
            }
            <Label> {
                draw_text: { color: #757575, text_style: { font_size: 14.0 } }
                text: "A card with action buttons at the bottom."
            }
            <ElementDivider> {}
            <View> {
                width: Fill, height: Fit,
                flow: Right, spacing: 8,
                <ElementButtonClear> { text: "Cancel" }
                <ElementButtonSolid> { text: "Confirm" }
            }
        }
    }
}
