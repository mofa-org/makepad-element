use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::icon::*;

    pub IconDetailPage = <ScrollXYView> {
        width: Fill,
        height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Icon"
        }

        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Icon sizes and colors. (Icons require SVG resources to display.)"
        }

        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 24,
            align: {y: 0.5},

            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},
                <ElementIconSmall> {}
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Small 16px"
                }
            }
            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},
                <ElementIcon> {}
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Medium 24px"
                }
            }
            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},
                <ElementIconLarge> {}
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Large 32px"
                }
            }
            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},
                <ElementIconPrimary> {}
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Primary"
                }
            }
        }
    }
}
