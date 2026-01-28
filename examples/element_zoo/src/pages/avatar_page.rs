use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::avatar::*;

    pub AvatarDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Avatar" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Sizes" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementAvatarSmall> { text_view = { label = { text: "S" } } }
            <ElementAvatar> { text_view = { label = { text: "M" } } }
            <ElementAvatarLarge> { text_view = { label = { text: "L" } } }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Different colors" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementAvatar> { text_view = { draw_bg: { color: #e74c3c } label = { text: "R" } } }
            <ElementAvatar> { text_view = { draw_bg: { color: #30cc71 } label = { text: "G" } } }
            <ElementAvatar> { text_view = { draw_bg: { color: #f5a623 } label = { text: "Y" } } }
            <ElementAvatar> { text_view = { draw_bg: { color: #9b59b6 } label = { text: "P" } } }
        }
    }
}
