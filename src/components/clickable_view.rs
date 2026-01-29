use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Clickable View - a tappable container that emits Click actions
    // Ported from makepad_wechat
    pub ElementClickableView = {{ElementClickableView}} {
        width: Fit, height: Fit,
        cursor: Hand,
        show_bg: true,
        draw_bg: {
            color: #fff,
        }
    }

    // Clickable View with hover effect
    pub ElementClickableViewHover = {{ElementClickableView}} {
        width: Fit, height: Fit,
        cursor: Hand,
        show_bg: true,
        draw_bg: {
            instance bg_color: #fff,
            instance bg_color_hover: #f5f5f5,
            instance hover: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 0.);
                sdf.fill(bg);
                return sdf.result;
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }
    }

    // Clickable View with rounded corners
    pub ElementClickableRoundedView = {{ElementClickableView}} {
        width: Fit, height: Fit,
        cursor: Hand,
        show_bg: true,
        draw_bg: {
            instance bg_color: #fff,
            instance bg_color_hover: #f5f5f5,
            instance border_radius: 8.0,
            instance hover: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                return sdf.result;
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementClickableView {
    #[deref]
    view: View,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ElementClickableViewAction {
    None,
    Clicked,
    Pressed,
    Released,
    LongPressed,
}

impl Widget for ElementClickableView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(_fe) => {
                cx.set_key_focus(self.view.area());
                cx.widget_action(uid, &scope.path, ElementClickableViewAction::Pressed);
            }
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    cx.widget_action(uid, &scope.path, ElementClickableViewAction::Clicked);
                }
                cx.widget_action(uid, &scope.path, ElementClickableViewAction::Released);
            }
            Hit::FingerLongPress(_) => {
                cx.widget_action(uid, &scope.path, ElementClickableViewAction::LongPressed);
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementClickableViewRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let ElementClickableViewAction::Clicked = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }

    pub fn pressed(&self, actions: &Actions) -> bool {
        if let ElementClickableViewAction::Pressed = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }

    pub fn released(&self, actions: &Actions) -> bool {
        if let ElementClickableViewAction::Released = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }

    pub fn long_pressed(&self, actions: &Actions) -> bool {
        if let ElementClickableViewAction::LongPressed = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }
}
