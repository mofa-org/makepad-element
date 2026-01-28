use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Small icon (16x16) — default: star
    pub ElementIconSmall = <Icon> {
        icon_walk: { width: 16, height: 16 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/star.svg"),
            color: #242424,
        }
    }

    // Medium icon (24x24) - default (RNE: size 24, color = theme.black #242424)
    pub ElementIcon = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/star.svg"),
            color: #242424,
        }
    }

    // Large icon (32x32)
    pub ElementIconLarge = <Icon> {
        icon_walk: { width: 32, height: 32 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/star.svg"),
            color: #242424,
        }
    }

    // Primary colored icon
    pub ElementIconPrimary = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/heart.svg"),
            color: #2089dc,
        }
    }

    // Search icon
    pub ElementIconSearch = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/search.svg"),
            color: #242424,
        }
    }

    // Close icon
    pub ElementIconClose = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/close.svg"),
            color: #242424,
        }
    }

    // Plus icon
    pub ElementIconPlus = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/plus.svg"),
            color: #242424,
        }
    }

    // Check icon
    pub ElementIconCheck = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/check.svg"),
            color: #242424,
        }
    }

    // Arrow right icon
    pub ElementIconArrowRight = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/arrow_right.svg"),
            color: #242424,
        }
    }

    // Info icon
    pub ElementIconInfo = <Icon> {
        icon_walk: { width: 24, height: 24 }
        draw_icon: {
            svg_file: dep("crate://makepad-element/resources/icons/info.svg"),
            color: #242424,
        }
    }
}
