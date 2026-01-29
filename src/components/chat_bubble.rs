use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    TORAD = 0.017453292519943295;  // PI / 180

    // Chat bubble with left pointer (incoming message)
    pub ElementChatBubbleLeft = <View> {
        width: Fit, height: Fit,
        padding: {left: 15, right: 10, top: 10, bottom: 10},
        align: {y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bubble_color: #ffffff,
            instance border_radius: 4.0,
            instance pointer_size: 8.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Main bubble body (offset to leave room for pointer)
                sdf.box(
                    self.pointer_size,
                    0.,
                    self.rect_size.x - self.pointer_size,
                    self.rect_size.y,
                    self.border_radius
                );

                // Upper part of left pointer triangle
                let center_y = self.rect_size.y * 0.5;
                let upper_start = vec2(0., center_y);
                let upper_end = vec2(0., center_y - self.pointer_size);
                sdf.translate(upper_start.x, upper_start.y);
                sdf.rotate(TORAD * -45., 0., 0.);
                sdf.rect(0., 0., length(upper_end - upper_start), self.pointer_size * 0.6);
                sdf.rotate(TORAD * 45., 0., 0.);
                sdf.translate(-upper_start.x, -upper_start.y);

                // Lower part of left pointer triangle
                let lower_start = vec2(0., center_y);
                let lower_end = vec2(0., center_y + self.pointer_size);
                sdf.translate(lower_start.x, lower_start.y);
                sdf.rotate(TORAD * 45., 0., 0.);
                sdf.rect(0., -self.pointer_size * 0.6, length(lower_end - lower_start), self.pointer_size * 0.6);
                sdf.rotate(TORAD * -45., 0., 0.);
                sdf.translate(-lower_start.x, -lower_start.y);

                return sdf.fill(self.bubble_color);
            }
        }

        content = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 14.0 }
            }
        }
    }

    // Chat bubble with right pointer (outgoing message)
    pub ElementChatBubbleRight = <View> {
        width: Fit, height: Fit,
        padding: {left: 10, right: 15, top: 10, bottom: 10},
        align: {y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bubble_color: #9e9,  // WeChat green (simplified)
            instance border_radius: 4.0,
            instance pointer_size: 8.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Main bubble body (offset to leave room for pointer)
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x - self.pointer_size,
                    self.rect_size.y,
                    self.border_radius
                );

                // Upper part of right pointer triangle
                let center_y = self.rect_size.y * 0.5;
                let right_x = self.rect_size.x;
                let upper_start = vec2(right_x, center_y);
                let upper_end = vec2(right_x, center_y - self.pointer_size);
                sdf.translate(upper_start.x, upper_start.y);
                sdf.rotate(TORAD * -225., 0., 0.);
                sdf.rect(0., 0., length(upper_end - upper_start), self.pointer_size * 0.6);
                sdf.rotate(TORAD * 225., 0., 0.);
                sdf.translate(-upper_start.x, -upper_start.y);

                // Lower part of right pointer triangle
                let lower_start = vec2(right_x, center_y);
                let lower_end = vec2(right_x, center_y + self.pointer_size);
                sdf.translate(lower_start.x, lower_start.y);
                sdf.rotate(TORAD * 225., 0., 0.);
                sdf.rect(0., -self.pointer_size * 0.6, length(lower_end - lower_start), self.pointer_size * 0.6);
                sdf.rotate(TORAD * -225., 0., 0.);
                sdf.translate(-lower_start.x, -lower_start.y);

                return sdf.fill(self.bubble_color);
            }
        }

        content = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 14.0 }
            }
        }
    }

    // Full incoming message row (avatar + bubble)
    pub ElementChatMessageIncoming = <View> {
        width: Fill, height: Fit,
        flow: Right,
        spacing: 10,
        padding: 10,
        align: {y: 0.0},

        avatar = <RoundedView> {
            width: 40, height: 40,
            draw_bg: {
                color: #e0e0e0,
                border_radius: 20.0,
            }
            align: {x: 0.5, y: 0.5},

            avatar_image = <Image> {
                width: 40, height: 40,
                fit: Stretch,
            }
        }

        bubble = <ElementChatBubbleLeft> {
            content = {
                width: Fit, height: Fit,
                draw_text: {
                    wrap: Word,
                }
            }
        }

        // Spacer to push content left
        <View> { width: Fill, height: 1 }
    }

    // Full outgoing message row (bubble + avatar)
    pub ElementChatMessageOutgoing = <View> {
        width: Fill, height: Fit,
        flow: Right,
        spacing: 10,
        padding: 10,
        align: {y: 0.0},

        // Spacer to push content right
        <View> { width: Fill, height: 1 }

        bubble = <ElementChatBubbleRight> {
            content = {
                width: Fit, height: Fit,
                draw_text: {
                    wrap: Word,
                }
            }
        }

        avatar = <RoundedView> {
            width: 40, height: 40,
            draw_bg: {
                color: #2089dc,
                border_radius: 20.0,
            }
            align: {x: 0.5, y: 0.5},

            avatar_image = <Image> {
                width: 40, height: 40,
                fit: Stretch,
            }
        }
    }

    // Simple chat bubble without pointer (for system messages, etc.)
    pub ElementChatBubbleSimple = <RoundedView> {
        width: Fit, height: Fit,
        padding: {left: 12, right: 12, top: 8, bottom: 8},
        draw_bg: {
            color: #f0f0f0,
            border_radius: 16.0,
        }

        content = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #666666,
                text_style: { font_size: 12.0 }
            }
        }
    }

    // System message (centered, gray)
    pub ElementChatSystemMessage = <View> {
        width: Fill, height: Fit,
        padding: {top: 8, bottom: 8},
        align: {x: 0.5, y: 0.5},

        <ElementChatBubbleSimple> {}
    }

    // Typing indicator
    pub ElementChatTypingIndicator = <View> {
        width: Fill, height: Fit,
        flow: Right,
        spacing: 10,
        padding: 10,
        align: {y: 0.5},

        avatar = <RoundedView> {
            width: 40, height: 40,
            draw_bg: {
                color: #e0e0e0,
                border_radius: 20.0,
            }
        }

        bubble = <ElementChatBubbleLeft> {
            draw_bg: { color: #f0f0f0 }

            <View> {
                width: 50, height: 20,
                flow: Right,
                spacing: 4,
                align: {x: 0.5, y: 0.5},

                <RoundedView> {
                    width: 8, height: 8,
                    draw_bg: { color: #999999, border_radius: 4.0 }
                }
                <RoundedView> {
                    width: 8, height: 8,
                    draw_bg: { color: #aaaaaa, border_radius: 4.0 }
                }
                <RoundedView> {
                    width: 8, height: 8,
                    draw_bg: { color: #bbbbbb, border_radius: 4.0 }
                }
            }
        }

        <View> { width: Fill, height: 1 }
    }
}
