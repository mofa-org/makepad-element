use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::theme_colors::*;

    // ─── Base: Solid Button (rounded box with hover/down) ───
    // Overrides only the pixel shader; color fields are inherited from Button.
    pub ElementButtonBase = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
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
            color: (PRIMARY_FOREGROUND),
            text_style: { font_size: 16.0 }
        }
    }

    // ─── Base: Circle Button (for FAB, social icons, speed dial) ───
    pub ElementCircleButtonBase = <Button> {
        width: 56,
        height: 56,
        padding: 0,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
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
            color: (PRIMARY_FOREGROUND),
            text_style: { font_size: 24.0 }
        }
    }

    // ─── Base: Circle View (for avatar background, badge dot) ───
    pub ElementCircleViewBase = <View> {
        show_bg: true,
        draw_bg: {
            instance bg_color: (PRIMARY),

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Base: Badge (pill shape with white border) ───
    pub ElementBadgeBase = <View> {
        width: Fit, height: 18,
        padding: {left: 4, right: 4, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: (DANGER),
            instance border_color: (CARD),

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 0.5;
                let hw = bw * 0.5;
                let w = self.rect_size.x - bw;
                let h = self.rect_size.y - bw;
                let r = min(w, h) * 0.25;
                sdf.box(hw, hw, w, h, max(1.0, r));
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "1"
        }
    }

    // ─── Base: Header (3-section app bar) ───
    pub ElementHeaderBase = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: 56,
            flow: Right,
            padding: {left: 10, right: 10, top: 10, bottom: 10},
            align: {y: 0.5},

            show_bg: true,
            draw_bg: { color: #2089dc }

            left = <View> {
                width: Fit, height: Fit,
            }

            center = <View> {
                width: Fill, height: Fit,
                padding: {left: 15, right: 15},
                align: {x: 0.5},
                title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #ffffff,
                        text_style: { font_size: 18.0 }
                    }
                    text: "Title"
                }
            }

            right = <View> {
                width: Fit, height: Fit,
            }
        }

        <View> {
            width: Fill, height: 1,
            show_bg: true,
            draw_bg: { color: #f2f2f2 }
        }
    }

    // ─── Base: Linear Progress Bar ───
    pub ElementLinearProgressBase = <View> {
        width: Fill, height: 4,
        show_bg: true,
        draw_bg: {
            instance progress: 0.5,
            instance fill_color: #2089dc,
            instance border_radius: 2.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let track_color = vec4(self.fill_color.x, self.fill_color.y, self.fill_color.z, 0.4);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(track_color);
                let fill_width = self.rect_size.x * self.progress;
                if fill_width > 0.0 {
                    sdf.box(0., 0., fill_width, self.rect_size.y, self.border_radius);
                    sdf.fill(self.fill_color);
                }
                return sdf.result;
            }
        }
    }

    // ─── Base: Skeleton Shimmer (rectangle with animated highlight band) ───
    pub ElementSkeletonBase = <View> {
        show_bg: true,
        draw_bg: {
            instance bg_color: #bdc6cf,
            instance highlight_color: #f5f5f5,
            instance border_radius: 2.0,
            instance shimmer_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                let band_center = self.shimmer_pos * 1.4 - 0.2;
                let dist = abs(self.pos.x - band_center);
                let band = smoothstep(0.15, 0.0, dist);
                let color = mix(self.bg_color, self.highlight_color, band);
                sdf.fill(color);
                return sdf.result;
            }
        }
        animator: {
            shimmer = {
                default: on,
                on = {
                    from: {all: Loop {duration: 1.5, end: 1.0}}
                    apply: {
                        draw_bg: { shimmer_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }

    // ─── Base: Chevron Icon (right-pointing arrow drawn with Sdf2d) ───
    // Note: uses stroke_color (not color) to avoid conflict with View's DrawColor.color field.
    pub ElementChevronRight = <View> {
        width: 8, height: 14,
        show_bg: true,
        draw_bg: {
            instance stroke_color: #bdc6cf,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.move_to(0., 0.);
                sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5);
                sdf.line_to(0., self.rect_size.y);
                sdf.stroke(self.stroke_color, 1.5);
                return sdf.result;
            }
        }
    }

    // ============================================================================
    // DARK THEME COMPONENT VARIANTS
    // ============================================================================

    // ─── Dark: Solid Button ───
    pub ElementButtonBaseDark = <ElementButtonBase> {
        draw_bg: {
            color: #439ce0,
            color_hover: #61b5eb,
            color_down: #2a7ab8,
        }
        draw_text: {
            color: #f2f2f2,
        }
    }

    // ─── Dark: Circle Button (for FAB, social icons) ───
    pub ElementCircleButtonBaseDark = <ElementCircleButtonBase> {
        draw_bg: {
            color: #439ce0,
            color_hover: #61b5eb,
        }
        draw_text: {
            color: #f2f2f2,
        }
    }

    // ─── Dark: Circle View (for avatar background) ───
    pub ElementCircleViewBaseDark = <ElementCircleViewBase> {
        draw_bg: {
            bg_color: #439ce0,
        }
    }

    // ─── Dark: Badge ───
    pub ElementBadgeBaseDark = <ElementBadgeBase> {
        draw_bg: {
            bg_color: #bf2c24,
            border_color: #1e1e22,
        }
        label = {
            draw_text: {
                color: #f2f2f2,
            }
        }
    }

    // ─── Dark: Header (3-section app bar) ───
    pub ElementHeaderBaseDark = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: 56,
            flow: Right,
            padding: {left: 10, right: 10, top: 10, bottom: 10},
            align: {y: 0.5},

            show_bg: true,
            draw_bg: { color: #1e1e22 }

            left = <View> {
                width: Fit, height: Fit,
            }

            center = <View> {
                width: Fill, height: Fit,
                padding: {left: 15, right: 15},
                align: {x: 0.5},
                title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #f2f2f2,
                        text_style: { font_size: 18.0 }
                    }
                    text: "Title"
                }
            }

            right = <View> {
                width: Fit, height: Fit,
            }
        }

        <View> {
            width: Fill, height: 1,
            show_bg: true,
            draw_bg: { color: #2e2e33 }
        }
    }

    // ─── Dark: Linear Progress Bar ───
    pub ElementLinearProgressBaseDark = <ElementLinearProgressBase> {
        draw_bg: {
            fill_color: #439ce0,
        }
    }

    // ─── Dark: Skeleton Shimmer ───
    pub ElementSkeletonBaseDark = <ElementSkeletonBase> {
        draw_bg: {
            bg_color: #393e42,
            highlight_color: #5e6977,
        }
    }

    // ─── Dark: Chevron Icon ───
    pub ElementChevronRightDark = <ElementChevronRight> {
        draw_bg: {
            stroke_color: #5e6977,
        }
    }

    // ─── Dark: Card Base ───
    pub ElementCardBaseDark = <View> {
        width: Fill, height: Fit,
        padding: 16,
        show_bg: true,
        draw_bg: {
            instance bg_color: #1e1e22,
            instance border_radius: 8.0,
            instance shadow_offset: vec2(0.0, 2.0),
            instance shadow_blur: 4.0,
            instance shadow_color: vec4(0.0, 0.0, 0.0, 0.3),

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Dark: List Item Base ───
    pub ElementListItemBaseDark = <View> {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 14, bottom: 14},
        flow: Right,
        spacing: 12,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: { color: #1e1e22 }
    }

    // ─── Dark: Input Base ───
    pub ElementInputBaseDark = <View> {
        width: Fill, height: Fit,
        padding: {left: 12, right: 12, top: 10, bottom: 10},
        show_bg: true,
        draw_bg: {
            instance bg_color: #111114,
            instance border_color: #393e42,
            instance border_radius: 4.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 1.0;
                let hw = bw * 0.5;
                sdf.box(hw, hw, self.rect_size.x - bw, self.rect_size.y - bw, self.border_radius);
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }
    }

    // ─── Dark: Tooltip ───
    pub ElementTooltipBaseDark = <View> {
        width: Fit, height: Fit,
        padding: {left: 8, right: 8, top: 4, bottom: 4},
        show_bg: true,
        draw_bg: {
            instance bg_color: #393e42,
            instance border_radius: 4.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Dark: Overlay/Backdrop ───
    pub ElementOverlayBaseDark = <View> {
        width: Fill, height: Fill,
        show_bg: true,
        draw_bg: { color: #00000099 }
    }

    // ─── Dark: Divider ───
    pub ElementDividerDark = <View> {
        width: Fill, height: 1,
        show_bg: true,
        draw_bg: { color: #2e2e33 }
    }

    // ─── Dark: Switch Base ───
    pub ElementSwitchBaseDark = <View> {
        width: 51, height: 31,
        show_bg: true,
        draw_bg: {
            instance on: 0.0,
            instance track_off: #393e42,
            instance track_on: #439ce0,
            instance thumb_color: #f2f2f2,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let h = self.rect_size.y;
                let r = h * 0.5;
                let track_color = mix(self.track_off, self.track_on, self.on);
                sdf.box(0., 0., self.rect_size.x, h, r);
                sdf.fill(track_color);
                let thumb_r = r - 2.0;
                let thumb_x = mix(r, self.rect_size.x - r, self.on);
                sdf.circle(thumb_x, r, thumb_r);
                sdf.fill(self.thumb_color);
                return sdf.result;
            }
        }
    }

    // ─── Dark: Chip Base ───
    pub ElementChipBaseDark = <View> {
        width: Fit, height: 32,
        padding: {left: 12, right: 12, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},
        show_bg: true,
        draw_bg: {
            instance bg_color: #393e42,
            instance border_radius: 16.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Dark: Dialog Base ───
    pub ElementDialogBaseDark = <View> {
        width: 280, height: Fit,
        padding: 24,
        show_bg: true,
        draw_bg: {
            instance bg_color: #1e1e22,
            instance border_radius: 8.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Dark: Tab Bar ───
    pub ElementTabBarDark = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: Fit,
            flow: Right,
            show_bg: true,
            draw_bg: { color: #1e1e22 }
        }

        indicator = <View> {
            width: Fill, height: 2,
            show_bg: true,
            draw_bg: { color: #aa49eb }
        }
    }

    // ─── Dark: Search Bar ───
    pub ElementSearchBarBaseDark = <View> {
        width: Fill, height: 40,
        padding: {left: 12, right: 12, top: 0, bottom: 0},
        align: {y: 0.5},
        show_bg: true,
        draw_bg: {
            instance bg_color: #111114,
            instance border_radius: 20.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }
}
