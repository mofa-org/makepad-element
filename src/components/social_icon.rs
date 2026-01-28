use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Generic social icon button (RNE: paddingV 14, paddingH 21, margin 7, borderRadius 30)
    pub ElementSocialIcon = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #333333,
            instance color_hover: #555555,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 18.0 }
        }
    }

    // GitHub social icon
    pub ElementSocialGithub = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #000000,
            instance color_hover: #333333,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "GH"
    }

    // Twitter/X social icon
    pub ElementSocialTwitter = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #00aced,
            instance color_hover: #0099d4,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "X"
    }

    // Discord social icon
    pub ElementSocialDiscord = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #5865f2,
            instance color_hover: #4752c4,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "DC"
    }

    // Facebook social icon
    pub ElementSocialFacebook = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #4267B2,
            instance color_hover: #365899,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "FB"
    }

    // LinkedIn social icon
    pub ElementSocialLinkedin = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #007bb6,
            instance color_hover: #006699,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "IN"
    }

    // YouTube social icon
    pub ElementSocialYoutube = <Button> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color: #bb0000,
            instance color_hover: #990000,
            instance border_radius: 30.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: { color: #ffffff, text_style: { font_size: 18.0 } }
        text: "YT"
    }
}
