use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::check_box::*;

    pub CheckBoxDetailPage = <ScrollXYView> {
        width: Fill, height: Fill,
        flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16},
        spacing: 16,

        <Label> {
            width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } }
            text: "CheckBox"
        }

        <View> { width: Fill, height: Fit, flow: Down, spacing: 12,
            <ElementCheckBox> { text: "Unchecked" }
            <ElementCheckBox> { text: "Checked", animator: { selected = { default: on } } }
            <ElementCheckBox> { text: "Disabled", animator: { disabled = { default: on } } }
            <ElementCheckBoxFlat> { text: "Flat Style" }
        }
    }
}
