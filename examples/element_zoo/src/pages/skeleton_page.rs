use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::skeleton::*;

    pub SkeletonDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "Skeleton" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Loading placeholder shapes" }

        // Card skeleton
        <RoundedView> {
            width: Fill, height: Fit, padding: 16,
            show_bg: true, draw_bg: { color: #ffffff, border_radius: 8.0 }
            flow: Down, spacing: 12,

            <View> { width: Fill, height: Fit, flow: Right, spacing: 12, align: {y: 0.5},
                <ElementSkeletonCircle> {}
                <View> { width: Fill, height: Fit, flow: Down, spacing: 8,
                    <ElementSkeletonLine> { width: 150 }
                    <ElementSkeletonLine> { width: 100, height: 12 }
                }
            }
            <ElementSkeletonRect> {}
            <ElementSkeletonLine> {}
            <ElementSkeletonLine> { width: 200 }
        }
    }
}
