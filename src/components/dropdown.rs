use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::popup_menu::*;

    // Dropdown button that shows selected value and opens a menu on click
    pub ElementDropdown = {{ElementDropdown}} {
        width: Fit, height: Fit,
        padding: {left: 12, right: 12, top: 8, bottom: 8},
        flow: Right,
        align: {y: 0.5},
        spacing: 8,
        cursor: Hand,

        show_bg: true,
        draw_bg: {
            instance bg_color: #fff,
            instance bg_color_hover: #f5f5f5,
            instance border_color: #ddd,
            instance border_radius: 4.0,
            instance hover: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);
                let bw = 1.0;
                let hw = bw * 0.5;
                sdf.box(hw, hw, self.rect_size.x - bw, self.rect_size.y - bw, self.border_radius);
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, bw);
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

        icon_wrapper = <View> {
            width: 0, height: 0,
            icon = <Icon> {
                width: 16, height: 16,
                draw_icon: {
                    color: #666,
                }
            }
        }

        label = <Label> {
            draw_text: {
                color: #333,
                text_style: { font_size: 14.0 }
            }
            text: "Select..."
        }

        // Arrow indicator (placeholder - users should provide their own icon)
        arrow = <Label> {
            draw_text: {
                color: #999,
                text_style: { font_size: 10.0 }
            }
            text: "▼"
        }
    }

    // Icon-only dropdown (like WeChat's header menu)
    pub ElementDropdownIcon = <ElementDropdown> {
        padding: 8,

        draw_bg: {
            bg_color: #0000,
            bg_color_hover: #0001,
        }

        icon_wrapper = {
            width: Fit, height: Fit,
            icon = {
                width: 20, height: 20,
                draw_icon: {
                    color: #333,
                }
            }
        }

        label = <View> { width: 0, height: 0 }
        arrow = <View> { width: 0, height: 0 }
    }

    // Dark themed dropdown
    pub ElementDropdownDark = <ElementDropdown> {
        draw_bg: {
            bg_color: #333,
            bg_color_hover: #444,
            border_color: #444,
        }
        icon_wrapper = { icon = { draw_icon: { color: #ccc } } }
        label = { draw_text: { color: #fff } }
        arrow = { draw_text: { color: #999 } }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ElementDropdownAction {
    None,
    Clicked,
    Opened,
    Closed,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementDropdown {
    #[deref]
    view: View,

    #[rust]
    is_open: bool,
}

impl Widget for ElementDropdown {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(_) => {
                cx.set_key_focus(self.view.area());
            }
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    cx.widget_action(uid, &scope.path, ElementDropdownAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementDropdownRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let ElementDropdownAction::Clicked = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }
}
