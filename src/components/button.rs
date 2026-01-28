use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Solid button (primary)
    pub ElementButtonSolid = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #2089dc,
            instance color_hover: #1975be,
            instance color_down: #1461a0,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                );
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
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
            instance border_color: #2089dc,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color, self.color_hover, self.hover);
                // Inset box by half stroke width so the border renders fully inside bounds
                let bw = 1.0;
                let hw = bw * 0.5;
                sdf.box(hw, hw, self.rect_size.x - bw, self.rect_size.y - bw, self.border_radius);
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }
        draw_text: {
            color: #2089dc,
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
            color: #2089dc,
            text_style: { font_size: 16.0 }
        }
    }

    // Error/danger button
    pub ElementButtonError = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ff190c,
            instance color_hover: #d9150a,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
        }
    }

    // Success button
    pub ElementButtonSuccess = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #52c41a,
            instance color_hover: #46a817,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
        }
    }

    // Secondary button
    pub ElementButtonSecondary = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ad1457,
            instance color_hover: #8e1148,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
        }
    }

    // Warning button
    pub ElementButtonWarning = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #faad14,
            instance color_hover: #e09a00,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
        }
    }

    // Disabled button
    pub ElementButtonDisabled = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #e5e5e5,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #9e9e9e,
            text_style: { font_size: 16.0 }
        }
    }
}
