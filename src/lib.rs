pub mod theme;
pub mod components;

pub use makepad_widgets;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    theme::font::live_design(cx);
    components::live_design(cx);
}
