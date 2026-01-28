use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Element theme font — Manrope
    pub ELEMENT_FONT_MANROPE = {
        font_family: {
            latin = font("crate://makepad-element/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    // Override the theme font so all widgets inherit Manrope
    pub THEME_FONT_REGULAR = <ELEMENT_FONT_MANROPE> {}
}
