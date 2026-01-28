use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Pricing card
    // Pricing card (RNE: margin 15, padding 15, borderRadius 3, border 1px #e1e8ee)
    pub ElementPricingCard = <RoundedView> {
        width: 280, height: Fit,
        margin: 15,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_color: #e1e8ee,
            border_radius: 3.0,
            border_size: 1.0,
        }

        flow: Down,
        spacing: 0,
        align: {x: 0.5},

        // Header
        header = <View> {
            width: Fill, height: Fit,
            padding: 15,
            flow: Down,
            spacing: 8,
            align: {x: 0.5},

            // RNE: title fontSize 30, fontWeight 800, color = primary, textAlign center
            plan_name = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #2089dc, text_style: { font_size: 30.0 } }
                text: "PRO"
            }

            // RNE: price fontSize 40, fontWeight 700, marginTop/Bottom 10
            price = <Label> {
                width: Fit, height: Fit,
                margin: {top: 10, bottom: 10},
                draw_text: { color: #242424, text_style: { font_size: 40.0 } }
                text: "$29"
            }

            period = <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #bdc6cf, text_style: { font_size: 12.0 } }
                text: "per month"
            }
        }

        // Divider
        <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: #e1e8ee } }

        // Features
        features = <View> {
            width: Fill, height: Fit,
            padding: 15,
            flow: Down,
            spacing: 8,
        }

        // CTA
        cta = <View> {
            width: Fill, height: Fit,
            padding: 15,
            align: {x: 0.5},
        }
    }
}
