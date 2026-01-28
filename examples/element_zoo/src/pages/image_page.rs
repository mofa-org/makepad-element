use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::image::*;

    pub ImageDetailPage = <ScrollXYView> {
        width: Fill,
        height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit,
            margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "Image"
        }

        <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } }
            text: "Image variants with different shapes. (No images loaded - showing placeholders.)"
        }

        <View> {
            width: Fill, height: Fit,
            flow: Right, spacing: 16,

            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},

                <View> {
                    width: 150, height: 100,
                    show_bg: true,
                    draw_bg: { color: #E0E0E0 }
                }
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Standard"
                }
            }

            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},

                <RoundedView> {
                    width: 150, height: 100,
                    show_bg: true,
                    draw_bg: { color: #E0E0E0, border_radius: 12.0 }
                }
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Rounded"
                }
            }

            <View> {
                width: Fit, height: Fit,
                flow: Down, spacing: 4,
                align: {x: 0.5},

                <View> {
                    width: 100, height: 100,
                    show_bg: true,
                    draw_bg: {
                        instance color: #E0E0E0,
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            sdf.circle(c.x, c.y, c.x);
                            sdf.fill(self.color);
                            return sdf.result;
                        }
                    }
                }
                <Label> {
                    width: Fit, height: Fit,
                    draw_text: { color: #757575, text_style: { font_size: 11.0 } }
                    text: "Circle"
                }
            }
        }
    }
}
