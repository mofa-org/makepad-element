use makepad_widgets::*;

live_design! {
    link theme_colors_dark;

    // Primary colors (Dark mode - brighter for visibility)
    pub PRIMARY = #439ce0
    pub PRIMARY_HOVER = #61b5eb
    pub PRIMARY_ACTIVE = #2089dc
    pub PRIMARY_LIGHT = #7ac4f0
    pub PRIMARY_FOREGROUND = #ffffff

    // Secondary colors
    pub SECONDARY = #aa49eb
    pub SECONDARY_HOVER = #bb6aef
    pub SECONDARY_ACTIVE = #9930e0
    pub SECONDARY_FOREGROUND = #ffffff

    // Danger/Error colors
    pub DANGER = #bf2c24
    pub DANGER_HOVER = #d4403a
    pub DANGER_ACTIVE = #a82520
    pub DANGER_FOREGROUND = #ffffff

    // Success colors
    pub SUCCESS = #439946
    pub SUCCESS_HOVER = #56ad5a
    pub SUCCESS_ACTIVE = #388a3b
    pub SUCCESS_FOREGROUND = #ffffff

    // Warning colors
    pub WARNING = #cfbe27
    pub WARNING_HOVER = #ddd03f
    pub WARNING_ACTIVE = #b8a820
    pub WARNING_FOREGROUND = #1a1a1a

    // Grey scale (inverted for dark mode)
    pub GREY_0 = #e1e8ee
    pub GREY_1 = #bdc6cf
    pub GREY_2 = #86939f
    pub GREY_3 = #5e6977
    pub GREY_4 = #43484d
    pub GREY_5 = #393e42

    // UI colors
    pub BACKGROUND = #111114
    pub FOREGROUND = #e1e1e1
    pub BORDER = #2e2e33
    pub INPUT = #1e1e22
    pub RING = #439ce0
    pub MUTED = #2e2e2e
    pub MUTED_FOREGROUND = #757575

    // Card colors
    pub CARD = #1e1e22
    pub CARD_FOREGROUND = #e1e1e1

    // Text colors
    pub TEXT_HEADING = #f2f2f2
    pub TEXT_PRIMARY = #e1e1e1
    pub TEXT_BODY = #cccccc
    pub TEXT_CAPTION = #9e9e9e
    pub TEXT_OVERLINE = #868686

    // Overlay
    pub BACKDROP = #00000099
    pub DIVIDER = #ffffff1f

    // Disabled
    pub DISABLED_BG = #2e2e2e
    pub DISABLED_TEXT = #757575

    // Tooltip
    pub TOOLTIP_BG = #393e42

    // Social brand colors (same as light)
    pub SOCIAL_FACEBOOK = #4267B2
    pub SOCIAL_TWITTER = #00aced
    pub SOCIAL_GITHUB = #ffffff
    pub SOCIAL_LINKEDIN = #007bb6
    pub SOCIAL_YOUTUBE = #bb0000
    pub SOCIAL_DISCORD = #5865F2

    // Transparent
    pub TRANSPARENT = #00000000
}
