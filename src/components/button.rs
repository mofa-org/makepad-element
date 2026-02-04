use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;
    use link::theme_colors::*;

    // Solid button (primary)
    pub ElementButtonSolid = <ElementButtonBase> {
        draw_bg: {
            color: (PRIMARY),
            color_hover: (PRIMARY_HOVER),
            color_down: (PRIMARY_ACTIVE),
        }
    }

    // Outline button
    pub ElementButtonOutline = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ffffff00,
            instance color_hover: #2089dc10,
            instance border_color: (PRIMARY),
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color, self.color_hover, self.hover);
                let bw = 1.0;
                let hw = bw * 0.5;
                sdf.box(hw, hw, self.rect_size.x - bw, self.rect_size.y - bw, self.border_radius);
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }
        draw_text: {
            color: (PRIMARY),
            text_style: { font_size: 16.0 }
        }
    }

    // Clear/text button
    pub ElementButtonClear = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ffffff00,
            instance color_hover: #2089dc10,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                return sdf.result;
            }
        }
        draw_text: {
            color: (PRIMARY),
            text_style: { font_size: 16.0 }
        }
    }

    // Error/danger button
    pub ElementButtonError = <ElementButtonBase> {
        draw_bg: {
            color: (DANGER),
            color_hover: (DANGER_HOVER),
        }
    }

    // Success button
    pub ElementButtonSuccess = <ElementButtonBase> {
        draw_bg: {
            color: (SUCCESS),
            color_hover: (SUCCESS_HOVER),
        }
    }

    // Secondary button
    pub ElementButtonSecondary = <ElementButtonBase> {
        draw_bg: {
            color: (SECONDARY),
            color_hover: (SECONDARY_HOVER),
        }
    }

    // Warning button
    pub ElementButtonWarning = <ElementButtonBase> {
        draw_bg: {
            color: (WARNING),
            color_hover: (WARNING_HOVER),
        }
    }

    // Disabled button
    pub ElementButtonDisabled = <ElementButtonBase> {
        enabled: false,
        draw_bg: {
            color: (DISABLED_BG),
            color_hover: (DISABLED_BG),
            color_down: (DISABLED_BG),
        }
        draw_text: {
            color: (DISABLED_TEXT),
        }
    }
}
