use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_element::components::pricing_card::*;
    use makepad_element::components::button::*;

    pub PricingCardDetailPage = <ScrollXYView> {
        width: Fill, height: Fill, flow: Down,
        padding: {left: 16, right: 16, top: 60, bottom: 16}, spacing: 16,

        <Label> { width: Fit, height: Fit, margin: {bottom: 16},
            draw_text: { color: #333333, text_style: { font_size: 24.0 } } text: "PricingCard" }

        <View> { width: Fill, height: Fit, flow: Right, spacing: 16,
            <ElementPricingCard> {
                header = {
                    plan_name = { text: "FREE" }
                    price = { text: "$0" }
                    period = { text: "forever" }
                }
                features = {
                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ 1 Project" }
                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Basic support" }
                    <Label> { draw_text: { color: #757575, text_style: { font_size: 13.0 } } text: "✗ Custom domain" }
                }
                cta = { <ElementButtonOutline> { text: "Get Started" } }
            }

            <ElementPricingCard> {
                header = {
                    plan_name = { text: "PRO" }
                    price = { text: "$29" }
                }
                features = {
                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Unlimited Projects" }
                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Priority support" }
                    <Label> { draw_text: { color: #424242, text_style: { font_size: 13.0 } } text: "✓ Custom domain" }
                }
                cta = { <ElementButtonSolid> { text: "Subscribe" } }
            }
        }
    }
}
