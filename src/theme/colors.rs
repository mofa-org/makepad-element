use makepad_widgets::*;

// Primary palette (React Native Elements)
pub const ELEMENT_PRIMARY: Vec4 = vec4(0.125, 0.537, 0.863, 1.0);       // #2089dc
pub const ELEMENT_PRIMARY_DARK: Vec4 = vec4(0.098, 0.459, 0.745, 1.0);  // #1975be
pub const ELEMENT_PRIMARY_DARKER: Vec4 = vec4(0.078, 0.380, 0.627, 1.0);// #1461a0
pub const ELEMENT_PRIMARY_LIGHT: Vec4 = vec4(0.380, 0.749, 0.902, 1.0); // #61BFE6
pub const ELEMENT_SECONDARY: Vec4 = vec4(0.678, 0.078, 0.341, 1.0);     // #ad1457
pub const ELEMENT_ERROR: Vec4 = vec4(1.0, 0.098, 0.047, 1.0);           // #ff190c
pub const ELEMENT_WARNING: Vec4 = vec4(0.980, 0.678, 0.078, 1.0);       // #faad14
pub const ELEMENT_SUCCESS: Vec4 = vec4(0.322, 0.769, 0.102, 1.0);       // #52c41a

// Grey palette (RNE)
pub const ELEMENT_GREY_0: Vec4 = vec4(0.224, 0.243, 0.259, 1.0);       // #393e42
pub const ELEMENT_GREY_1: Vec4 = vec4(0.263, 0.282, 0.302, 1.0);       // #43484d
pub const ELEMENT_GREY_2: Vec4 = vec4(0.369, 0.412, 0.467, 1.0);       // #5e6977
pub const ELEMENT_GREY_3: Vec4 = vec4(0.525, 0.576, 0.620, 1.0);       // #86939e
pub const ELEMENT_GREY_4: Vec4 = vec4(0.741, 0.776, 0.812, 1.0);       // #bdc6cf
pub const ELEMENT_GREY_5: Vec4 = vec4(0.882, 0.910, 0.933, 1.0);       // #e1e8ee
pub const ELEMENT_WHITE: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);              // #FFFFFF
pub const ELEMENT_BLACK: Vec4 = vec4(0.141, 0.141, 0.141, 1.0);        // #242424

// Functional
pub const ELEMENT_DIVIDER: Vec4 = vec4(0.0, 0.0, 0.0, 0.12);            // rgba(0,0,0,0.12)
pub const ELEMENT_DISABLED_BG: Vec4 = vec4(0.898, 0.898, 0.898, 1.0);  // #e5e5e5
pub const ELEMENT_DISABLED_TEXT: Vec4 = vec4(0.620, 0.620, 0.620, 1.0); // #9e9e9e
pub const ELEMENT_TOOLTIP_BG: Vec4 = vec4(0.380, 0.439, 0.502, 1.0);   // #617080

// Social brand colors
pub const ELEMENT_SOCIAL_FACEBOOK: Vec4 = vec4(0.259, 0.404, 0.698, 1.0);  // #4267B2
pub const ELEMENT_SOCIAL_TWITTER: Vec4 = vec4(0.0, 0.675, 0.929, 1.0);     // #00aced
pub const ELEMENT_SOCIAL_GITHUB: Vec4 = vec4(0.0, 0.0, 0.0, 1.0);         // #000000
pub const ELEMENT_SOCIAL_LINKEDIN: Vec4 = vec4(0.0, 0.482, 0.714, 1.0);   // #007bb6
pub const ELEMENT_SOCIAL_YOUTUBE: Vec4 = vec4(0.733, 0.0, 0.0, 1.0);      // #bb0000
pub const ELEMENT_SOCIAL_DISCORD: Vec4 = vec4(0.345, 0.396, 0.949, 1.0);  // #5865F2

// Text hierarchy
pub const ELEMENT_TEXT_HEADING: Vec4 = vec4(0.129, 0.129, 0.129, 1.0);  // #212121
pub const ELEMENT_TEXT_PRIMARY: Vec4 = vec4(0.200, 0.200, 0.200, 1.0);  // #333333
pub const ELEMENT_TEXT_BODY: Vec4 = vec4(0.259, 0.259, 0.259, 1.0);     // #424242
pub const ELEMENT_TEXT_CAPTION: Vec4 = vec4(0.459, 0.459, 0.459, 1.0);  // #757575
pub const ELEMENT_TEXT_OVERLINE: Vec4 = vec4(0.620, 0.620, 0.620, 1.0); // #9e9e9e

// Overlay / backdrop
pub const ELEMENT_BACKDROP: Vec4 = vec4(0.0, 0.0, 0.0, 0.4);           // #00000066

// Border
pub const ELEMENT_BORDER: Vec4 = vec4(0.949, 0.949, 0.949, 1.0);       // #f2f2f2

// Background
pub const ELEMENT_BG_PAGE: Vec4 = vec4(0.945, 0.945, 0.969, 1.0);      // #F1F1F7
pub const ELEMENT_BG_CARD: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);            // #FFFFFF

// ============================================================================
// DARK THEME COLORS (React Native Elements darkColors)
// ============================================================================

// Primary palette (Dark mode)
pub const ELEMENT_DARK_PRIMARY: Vec4 = vec4(0.263, 0.612, 0.878, 1.0);      // #439ce0
pub const ELEMENT_DARK_PRIMARY_LIGHT: Vec4 = vec4(0.380, 0.710, 0.922, 1.0);// #61b5eb
pub const ELEMENT_DARK_SECONDARY: Vec4 = vec4(0.667, 0.286, 0.922, 1.0);    // #aa49eb
pub const ELEMENT_DARK_ERROR: Vec4 = vec4(0.749, 0.173, 0.141, 1.0);        // #bf2c24
pub const ELEMENT_DARK_WARNING: Vec4 = vec4(0.812, 0.745, 0.153, 1.0);      // #cfbe27
pub const ELEMENT_DARK_SUCCESS: Vec4 = vec4(0.263, 0.600, 0.275, 1.0);      // #439946

// Grey palette (Dark mode - inverted from light)
pub const ELEMENT_DARK_GREY_0: Vec4 = vec4(0.882, 0.910, 0.933, 1.0);       // #e1e8ee
pub const ELEMENT_DARK_GREY_1: Vec4 = vec4(0.741, 0.776, 0.812, 1.0);       // #bdc6cf
pub const ELEMENT_DARK_GREY_2: Vec4 = vec4(0.525, 0.576, 0.620, 1.0);       // #86939e
pub const ELEMENT_DARK_GREY_3: Vec4 = vec4(0.369, 0.412, 0.467, 1.0);       // #5e6977
pub const ELEMENT_DARK_GREY_4: Vec4 = vec4(0.263, 0.282, 0.302, 1.0);       // #43484d
pub const ELEMENT_DARK_GREY_5: Vec4 = vec4(0.224, 0.243, 0.259, 1.0);       // #393e42
pub const ELEMENT_DARK_WHITE: Vec4 = vec4(0.031, 0.031, 0.031, 1.0);        // #080808 (inverted)
pub const ELEMENT_DARK_BLACK: Vec4 = vec4(0.949, 0.949, 0.949, 1.0);        // #f2f2f2 (inverted)

// Functional (Dark mode)
pub const ELEMENT_DARK_DIVIDER: Vec4 = vec4(1.0, 1.0, 1.0, 0.12);           // rgba(255,255,255,0.12)
pub const ELEMENT_DARK_DISABLED_BG: Vec4 = vec4(0.180, 0.180, 0.180, 1.0);  // #2e2e2e
pub const ELEMENT_DARK_DISABLED_TEXT: Vec4 = vec4(0.459, 0.459, 0.459, 1.0);// #757575
pub const ELEMENT_DARK_TOOLTIP_BG: Vec4 = vec4(0.224, 0.243, 0.259, 1.0);   // #393e42

// Text hierarchy (Dark mode)
pub const ELEMENT_DARK_TEXT_HEADING: Vec4 = vec4(0.949, 0.949, 0.949, 1.0); // #f2f2f2
pub const ELEMENT_DARK_TEXT_PRIMARY: Vec4 = vec4(0.882, 0.882, 0.882, 1.0); // #e1e1e1
pub const ELEMENT_DARK_TEXT_BODY: Vec4 = vec4(0.800, 0.800, 0.800, 1.0);    // #cccccc
pub const ELEMENT_DARK_TEXT_CAPTION: Vec4 = vec4(0.620, 0.620, 0.620, 1.0); // #9e9e9e
pub const ELEMENT_DARK_TEXT_OVERLINE: Vec4 = vec4(0.525, 0.525, 0.525, 1.0);// #868686

// Overlay / backdrop (Dark mode)
pub const ELEMENT_DARK_BACKDROP: Vec4 = vec4(0.0, 0.0, 0.0, 0.6);           // #00000099

// Border (Dark mode)
pub const ELEMENT_DARK_BORDER: Vec4 = vec4(0.180, 0.180, 0.200, 1.0);       // #2e2e33

// Background (Dark mode)
pub const ELEMENT_DARK_BG_PAGE: Vec4 = vec4(0.067, 0.067, 0.078, 1.0);      // #111114
pub const ELEMENT_DARK_BG_CARD: Vec4 = vec4(0.118, 0.118, 0.133, 1.0);      // #1e1e22
pub const ELEMENT_DARK_BG_ELEVATED: Vec4 = vec4(0.157, 0.157, 0.176, 1.0);  // #28282d
