use makepad_widgets::*;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree, fontdb};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    DrawSvg = {{DrawSvg}} {
        texture tex: texture2d

        fn pixel(self) -> vec4 {
            return sample2d(self.tex, self.pos);
        }
    }

    pub ElementSvg = {{ElementSvg}} {
        width: Fit, height: Fit,
        draw_svg: {
            texture tex: texture2d
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawSvg {
    #[deref]
    draw_super: DrawQuad,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementSvg {
    #[live]
    draw_svg: DrawSvg,

    #[redraw]
    #[rust]
    area: Area,

    #[walk]
    walk: Walk,

    /// Raw SVG string content
    #[live]
    text: String,

    /// Scale factor for rasterization (default 2.0 for retina)
    #[live(2.0)]
    scale: f64,

    #[rust]
    old_text: String,

    #[rust]
    texture: Option<Texture>,
}

impl Widget for ElementSvg {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.render_svg(cx);
        if let Some(texture) = &self.texture {
            let (width, height) = texture.get_format(cx).vec_width_height().unwrap_or((0, 0));
            let scale = self.scale;
            let display_w = width as f64 / scale;
            let display_h = height as f64 / scale;

            // Use walk sizes if explicitly set, otherwise use SVG intrinsic size
            let final_walk = Walk {
                width: if matches!(walk.width, Size::Fit {..}) {
                    Size::Fixed(display_w)
                } else {
                    walk.width
                },
                height: if matches!(walk.height, Size::Fit {..}) {
                    Size::Fixed(display_h)
                } else {
                    walk.height
                },
                ..walk
            };

            self.draw_svg.draw_vars.set_texture(0, texture);
            self.draw_svg.draw_walk(cx, final_walk);
        }
        DrawStep::done()
    }
}

impl ElementSvg {
    fn render_svg(&mut self, cx: &mut Cx) {
        if self.text == self.old_text {
            return;
        }
        self.old_text = self.text.clone();

        if self.text.is_empty() {
            self.texture = None;
            return;
        }

        let mut opt = Options::default();
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        opt.fontdb = std::sync::Arc::new(db);

        match Tree::from_str(&self.text, &opt) {
            Ok(tree) => {
                let size = tree.size().to_int_size();
                let scale = self.scale.max(1.0) as u32;
                let width = scale * size.width();
                let height = scale * size.height();

                let Some(mut pixmap) = Pixmap::new(width, height) else {
                    log!("SVG: failed to create pixmap {}x{}", width, height);
                    return;
                };

                let transform = Transform::from_scale(scale as f32, scale as f32);
                resvg::render(&tree, transform, &mut pixmap.as_mut());

                let rgba_data = pixmap.data();
                let mut bgra_data = Vec::with_capacity((width * height) as usize);
                for chunk in rgba_data.chunks(4) {
                    let r = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let b = chunk[2] as u32;
                    let a = chunk[3] as u32;
                    bgra_data.push((a << 24) | (r << 16) | (g << 8) | b);
                }

                let texture = Texture::new_with_format(cx, TextureFormat::VecBGRAu8_32 {
                    data: Some(bgra_data),
                    width: width as usize,
                    height: height as usize,
                    updated: TextureUpdated::Full,
                });

                self.texture = Some(texture);
            }
            Err(e) => {
                log!("SVG parse error: {:?}", e);
            }
        }
    }
}
