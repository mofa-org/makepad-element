use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element themed slider with RNE primary color
    pub ElementSlider = <Slider> {
        width: Fill,
        height: Fit,
        min: 0.0,
        max: 100.0,
        draw_bg: {
            color_drag: #2089dc,
        }
    }

    // Round style slider
    pub ElementSliderRound = <SliderRound> {
        width: Fill,
        height: Fit,
        min: 0.0,
        max: 100.0,
        draw_bg: {
            color_drag: #2089dc,
        }
    }

    // Minimal style slider
    pub ElementSliderMinimal = <SliderMinimal> {
        width: Fill,
        height: Fit,
        min: 0.0,
        max: 100.0,
        draw_bg: {
            color_drag: #2089dc,
        }
    }
}
