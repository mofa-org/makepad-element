// BUG: CurvedLabel does not render.
//
// Root cause: DrawText::draw_abs() does not produce visible output when called
// inside a turtle context (begin_turtle / end_turtle_with_area). The text glyphs
// are either clipped by the turtle's draw_clip rect, or the draw calls are batched
// into a single GPU draw call where per-character uniforms (rotation angle) are
// shared across all glyphs (only the last value takes effect).
//
// Attempted approaches that all failed:
//   1. Custom DrawRotatedText shader with per-character `uniform rotation` —
//      uniforms are per-draw-call, not per-instance. Consecutive draw_abs calls
//      get merged by append_to_draw_call() so all chars share one rotation.
//   2. Plain DrawText::draw_abs per character without rotation — draw_abs inside
//      begin_turtle produces no visible output (suspected draw_clip issue).
//   3. DrawText::draw_walk — also produces no output.
//   4. View deref + draw_abs after view.draw_walk — no output.
//
// To fix this properly, one of these approaches is needed:
//   A. Use `instance` variables instead of `uniform` for rotation, with a custom
//      DrawText subclass that adds per-glyph instance data. This requires modifying
//      draw_rasterized_glyph to write extra instance slots.
//   B. Use a CachedView (optimize: Texture) approach where each character is drawn
//      into a separate texture-backed view that can be rotated as a whole.
//   C. Wait for Makepad to expose a draw_abs variant that bypasses turtle clipping.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Text arranged in a circular arc.
    // BUG: Currently does not render. See comments above.
    pub ElementCurvedLabel = {{ElementCurvedLabel}} {
        width: Fit, height: Fit,

        text: "CURVED TEXT",

        draw_text: {
            color: #212121,
            text_style: { font_size: 12.0 }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementCurvedLabel {
    #[deref]
    view: View,

    #[live]
    text: String,

    #[live(0.0)]
    rotation: f64,

    #[live(100.0)]
    radius: f64,

    #[live(std::f64::consts::PI * 0.5)]
    total_angle: f64,

    #[live]
    draw_text: DrawText,

    #[redraw]
    #[rust]
    area: Area,
}

impl Widget for ElementCurvedLabel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, Layout::default());

        let turtle_rect = cx.turtle().rect();
        let abs_pos = turtle_rect.pos;
        let width = turtle_rect.size.x;
        let radius = self.radius;

        let char_count = self.text.chars().count();
        if char_count > 0 {
            let text_clone = self.text.clone();
            for (index, ch) in text_clone.chars().enumerate() {
                let slice_angle = self.total_angle / (char_count as f64);
                let angle = -(index as f64 - (char_count as f64 - 1.0) / 2.0) * slice_angle + self.rotation;

                let center_x = if width <= 0.0 || width.is_nan() { radius } else { width / 2.0 };

                let char_x = abs_pos.x + center_x - radius * angle.sin();
                let char_y = abs_pos.y - radius * angle.cos() + radius;

                self.draw_text.draw_abs(
                    cx,
                    dvec2(char_x, char_y),
                    &ch.to_string(),
                );
            }
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}
