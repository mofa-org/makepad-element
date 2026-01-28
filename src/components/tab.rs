use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Tab bar item
    // RNE: paddingH 16, paddingV 8, text color white (primary variant)
    pub ElementTabItem = <RadioButtonTab> {
        width: Fit, height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},
    }

    // Tab bar container (RNE: bg = primary #2089dc)
    pub ElementTabBar = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: Fit,
            flow: Right,
            show_bg: true,
            draw_bg: { color: #2089dc }
        }

        // Indicator line (RNE: 2px, secondary #ad1457)
        indicator = <View> {
            width: Fill, height: 2,
            show_bg: true,
            draw_bg: { color: #ad1457 }
        }
    }

    // Tab view with bar + content (RNE: bg primary, indicator secondary)
    pub ElementTabView = <View> {
        width: Fill, height: Fill,
        flow: Down,

        tab_bar = <View> {
            width: Fill, height: Fit,
            flow: Down,

            bar = <View> {
                width: Fill, height: Fit,
                flow: Right,
                show_bg: true,
                draw_bg: { color: #2089dc }
            }

            indicator = <View> {
                width: Fill, height: 2,
                show_bg: true,
                draw_bg: { color: #ad1457 }
            }
        }

        content = <PageFlip> {
            width: Fill, height: Fill,
        }
    }
}
