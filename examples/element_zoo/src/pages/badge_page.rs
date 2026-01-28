use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::badge::*;

    pub BadgeDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Badge" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Badge variants" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementBadge> { label = { text: "3" } }
            <ElementBadge> { label = { text: "99+" } }
            <ElementBadgePrimary> { label = { text: "New" } }
            <ElementBadgeSuccess> { label = { text: "OK" } }
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Dot badge" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 16, align: {y: 0.5},
            <ElementBadgeDot> {}
            <ElementBadgeDot> { draw_bg: { color: #30cc71 } }
            <ElementBadgeDot> { draw_bg: { color: #f5a623 } }
        }
    }
}
