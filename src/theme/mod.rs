pub mod colors;
pub mod colors_light;
pub mod colors_dark;
pub mod typography;
pub mod spacing;
pub mod font;
pub mod live_theme;

pub use colors::*;
pub use typography::*;
pub use spacing::*;

use makepad_widgets::*;
use std::sync::atomic::{AtomicU8, Ordering};

const THEME_MODE_LIGHT: u8 = 0;
const THEME_MODE_DARK: u8 = 1;
static THEME_MODE_STATE: AtomicU8 = AtomicU8::new(THEME_MODE_LIGHT);

pub fn live_design(cx: &mut Cx) {
    crate::theme::colors_light::live_design(cx);
    crate::theme::colors_dark::live_design(cx);
    // Set default theme (or preserve the last requested mode on live reload).
    link_theme_colors(cx, ThemeMode::from_u8(THEME_MODE_STATE.load(Ordering::Relaxed)));
}

/// Theme mode enum for switching between light and dark themes
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            ThemeMode::Light => THEME_MODE_LIGHT,
            ThemeMode::Dark => THEME_MODE_DARK,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            THEME_MODE_DARK => ThemeMode::Dark,
            _ => ThemeMode::Light,
        }
    }
}

fn link_theme_colors(cx: &mut Cx, mode: ThemeMode) {
    match mode {
        ThemeMode::Light => {
            cx.link(live_id!(theme), live_id!(theme_desktop_light));
            cx.link(live_id!(theme_colors), live_id!(theme_colors_light));
        }
        ThemeMode::Dark => {
            cx.link(live_id!(theme), live_id!(theme_desktop_dark));
            cx.link(live_id!(theme_colors), live_id!(theme_colors_dark));
        }
    }
}

pub fn current_theme_mode() -> ThemeMode {
    ThemeMode::from_u8(THEME_MODE_STATE.load(Ordering::Relaxed))
}

/// Apply theme mode globally using Makepad's link system
///
/// This function switches the theme colors by relinking the theme_colors
/// live_id to either light or dark color definitions, then reloads the UI DSL.
///
/// # Example
/// ```rust
/// use makepad_element::theme::{ThemeMode, apply_theme};
///
/// // Switch to dark mode
/// apply_theme(cx, ThemeMode::Dark);
///
/// // Switch back to light mode
/// apply_theme(cx, ThemeMode::Light);
/// ```
pub fn apply_theme(cx: &mut Cx, mode: ThemeMode) {
    THEME_MODE_STATE.store(mode.as_u8(), Ordering::Relaxed);
    link_theme_colors(cx, mode);
    cx.reload_ui_dsl();
}
