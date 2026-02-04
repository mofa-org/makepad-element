pub mod theme;
pub mod components;

pub use makepad_widgets;
pub use makepad_code_editor;
pub use math_widget;

use makepad_widgets::*;

pub use theme::{ThemeMode, apply_theme, current_theme_mode};

pub fn live_design(cx: &mut Cx) {
    makepad_code_editor::live_design(cx);
    math_widget::math::live_design(cx);
    theme::live_design(cx);
    theme::font::live_design(cx);
    theme::live_theme::live_design(cx);
    components::live_design(cx);
}
