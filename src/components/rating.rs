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

    // Interactive clickable star
    ClickableStar = <View> {
        width: Fit, height: Fit,
        cursor: Hand,

        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }

        star_label = <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #bdc6cf, text_style: { font_size: 24.0 } }
            text: "★"
        }
    }

    // Interactive rating component (tap to select)
    pub ElementRating = {{ElementRating}} {
        width: Fit, height: Fit,
        flow: Right, spacing: 4,

        star_size: 24.0,
        filled_color: #faad14,
        empty_color: #bdc6cf,
        max_stars: 5,
        value: 0,
        read_only: false,

        star0 = <ClickableStar> {}
        star1 = <ClickableStar> {}
        star2 = <ClickableStar> {}
        star3 = <ClickableStar> {}
        star4 = <ClickableStar> {}
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ElementRatingAction {
    Changed(usize),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementRating {
    #[deref]
    view: View,

    #[live(24.0)]
    star_size: f64,

    #[live]
    filled_color: Vec4,

    #[live]
    empty_color: Vec4,

    #[live(5)]
    max_stars: i64,

    #[live(0)]
    value: i64,

    #[live(false)]
    read_only: bool,
}

impl Widget for ElementRating {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.read_only {
            return;
        }

        // Handle clicks on each star
        for i in 0..(self.max_stars.min(5) as usize) {
            let star_id = match i {
                0 => id!(star0),
                1 => id!(star1),
                2 => id!(star2),
                3 => id!(star3),
                4 => id!(star4),
                _ => continue,
            };

            let star = self.view.view(&[star_id]);
            if let Hit::FingerUp(fe) = event.hits(cx, star.area()) {
                if fe.is_over {
                    let new_value = (i + 1) as i64;
                    if new_value != self.value {
                        self.value = new_value;
                        self.update_stars(cx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            ElementRatingAction::Changed(self.value as usize),
                        );
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update star colors before drawing
        self.update_stars_colors(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementRating {
    fn update_stars(&mut self, cx: &mut Cx) {
        for i in 0..(self.max_stars.min(5) as usize) {
            let star_id = match i {
                0 => id!(star0),
                1 => id!(star1),
                2 => id!(star2),
                3 => id!(star3),
                4 => id!(star4),
                _ => continue,
            };

            let color = if (i as i64) < self.value {
                self.filled_color
            } else {
                self.empty_color
            };

            self.view.label(&[star_id, id!(star_label)]).apply_over(cx, live! {
                draw_text: { color: (color) }
            });
        }
        self.view.redraw(cx);
    }

    fn update_stars_colors(&mut self, cx: &mut Cx) {
        for i in 0..(self.max_stars.min(5) as usize) {
            let star_id = match i {
                0 => id!(star0),
                1 => id!(star1),
                2 => id!(star2),
                3 => id!(star3),
                4 => id!(star4),
                _ => continue,
            };

            let color = if (i as i64) < self.value {
                self.filled_color
            } else {
                self.empty_color
            };

            self.view.label(&[star_id, id!(star_label)]).apply_over(cx, live! {
                draw_text: { color: (color) }
            });
        }
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: usize) {
        self.value = (value as i64).min(self.max_stars);
        self.update_stars(cx);
    }

    pub fn get_value(&self) -> usize {
        self.value as usize
    }
}

impl ElementRatingRef {
    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ElementRatingAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }

    pub fn set_value(&self, cx: &mut Cx, value: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn get_value(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.get_value()
        } else {
            0
        }
    }
}
