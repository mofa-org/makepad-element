use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Menu Item - individual item in a popup menu
    pub ElementMenuItem = {{ElementMenuItem}} {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 12, bottom: 12},
        spacing: 12,
        flow: Right,
        align: {y: 0.5},
        cursor: Hand,

        show_bg: true,
        draw_bg: {
            instance bg_color: #ffffff,
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
                    from: {all: Snap}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Snap}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }

        icon = <Icon> {
            width: 20, height: 20,
            draw_icon: {
                color: #666,
            }
        }

        label = <Label> {
            draw_text: {
                color: #333,
                text_style: { font_size: 14.0 }
            }
        }
    }

    // Dark themed menu item
    pub ElementMenuItemDark = <ElementMenuItem> {
        draw_bg: {
            bg_color: #333,
            bg_color_hover: #444,
        }
        icon = { draw_icon: { color: #ccc } }
        label = { draw_text: { color: #ccc } }
    }

    // Popup Menu container
    pub ElementPopupMenu = <RoundedView> {
        width: Fit, height: Fit,
        flow: Down,
        padding: 4,

        draw_bg: {
            color: #fff,
            border_radius: 8.0,
        }
    }

    // Dark themed popup menu
    pub ElementPopupMenuDark = <RoundedView> {
        width: Fit, height: Fit,
        flow: Down,
        padding: 4,

        draw_bg: {
            color: #333,
            border_radius: 8.0,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct MenuItemId(pub LiveId);

#[derive(Clone, DefaultNone, Debug)]
pub enum ElementMenuItemAction {
    None,
    Hovered,
    Selected,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementMenuItem {
    #[deref]
    view: View,
}

impl Widget for ElementMenuItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(_) => {
            }
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    cx.widget_action(uid, &scope.path, ElementMenuItemAction::Selected);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.widget_action(uid, &scope.path, ElementMenuItemAction::Hovered);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementMenuItemRef {
    pub fn selected(&self, actions: &Actions) -> bool {
        if let ElementMenuItemAction::Selected = actions.find_widget_action(self.widget_uid()).cast() {
            return true;
        }
        false
    }
}
