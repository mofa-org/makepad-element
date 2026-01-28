use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Single star (filled)
    StarFilled = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #faad14, text_style: { font_size: 20.0 } }
        text: "★"
    }

    // Single star (empty)
    StarEmpty = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #bdc6cf, text_style: { font_size: 20.0 } }
        text: "★"
    }

    // 5-star rating (0 filled)
    pub ElementRating0 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
        <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (1 filled)
    pub ElementRating1 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (2 filled)
    pub ElementRating2 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (3 filled)
    pub ElementRating3 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (4 filled)
    pub ElementRating4 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarEmpty> {}
    }

    // 5-star rating (5 filled)
    pub ElementRating5 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarFilled> {} <StarFilled> {}
    }
}
