use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Horizontal filler - expands to fill available horizontal space
    // Useful for pushing items to opposite sides in a Row layout
    pub FillerX = <View> {
        width: Fill,
        height: Fit,
    }

    // Vertical filler - expands to fill available vertical space
    // Useful for pushing items to top/bottom in a Column layout
    pub FillerY = <View> {
        width: Fit,
        height: Fill,
    }

    // Fixed-width spacer
    pub SpacerX = <View> {
        width: 10,
        height: Fit,
    }

    // Fixed-height spacer
    pub SpacerY = <View> {
        width: Fit,
        height: 10,
    }

    // Small spacers (4px)
    pub SpacerXS = <View> { width: 4, height: Fit }
    pub SpacerYS = <View> { width: Fit, height: 4 }

    // Medium spacers (8px)
    pub SpacerXM = <View> { width: 8, height: Fit }
    pub SpacerYM = <View> { width: Fit, height: 8 }

    // Large spacers (16px)
    pub SpacerXL = <View> { width: 16, height: Fit }
    pub SpacerYL = <View> { width: Fit, height: 16 }

    // Extra large spacers (24px)
    pub SpacerXXL = <View> { width: 24, height: Fit }
    pub SpacerYXL = <View> { width: Fit, height: 24 }

    // Flexible spacer with minimum size
    pub FlexSpacerX = <View> {
        width: Fill,
        height: Fit,
        margin: {left: 8, right: 8},
    }

    pub FlexSpacerY = <View> {
        width: Fit,
        height: Fill,
        margin: {top: 8, bottom: 8},
    }
}
