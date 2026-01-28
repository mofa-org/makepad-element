use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::social_icon::*;

    pub SocialIconDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "SocialIcon" }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Brand social icons" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
            <ElementSocialGithub> {}
            <ElementSocialTwitter> {}
            <ElementSocialDiscord> {}
        }

        <Label> { width: Fit, height: Fit,
            draw_text: { color: #888888, text_style: { font_size: 14.0 } } text: "Custom social icons" }
        <View> { width: Fill, height: Fit, flow: Right, spacing: 12,
            <ElementSocialIcon> { draw_bg: { color: #dd4b39 } text: "G+" }
            <ElementSocialIcon> { draw_bg: { color: #0077b5 } text: "in" }
            <ElementSocialIcon> { draw_bg: { color: #ff4500 } text: "R" }
        }
    }
}
