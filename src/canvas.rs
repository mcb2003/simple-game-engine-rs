use std::ops::{Deref, DerefMut};

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas as SdlCanvas, RenderTarget, Texture, TextureCreator},
    surface::{Surface, SurfaceContext},
    video::{Window, WindowContext},
};
use sdl2_unifont::renderer::SurfaceRenderer as TextRenderer;

pub type SurfaceCanvas<'a> = Canvas<Surface<'a>, SurfaceContext<'a>>;
pub type WindowCanvas = Canvas<Window, WindowContext>;

pub struct Canvas<T: RenderTarget, U> {
    inner: SdlCanvas<T>,
    texture_creator: TextureCreator<U>,
    text_renderer: TextRenderer,
}

impl<'a> SurfaceCanvas<'a> {
    pub fn new(inner: SdlCanvas<Surface<'a>>) -> Self {
        let texture_creator = inner.texture_creator();
        let text_renderer = TextRenderer::new(inner.draw_color(), Color::RGBA(0, 0, 0, 0));
        Self {
            inner,
            texture_creator,
            text_renderer,
        }
    }

    pub fn draw_text_surface<P: Into<Point>>(
        &mut self,
        text: &str,
        pos: P,
    ) -> Result<Surface, String> {
        let pos = pos.into();
        let surface = self.text_renderer.draw(text)?;
        let mut rect = surface.rect();
        rect.set_x(pos.x());
        rect.set_y(pos.y());
        surface.blit(None, self.inner.surface_mut(), rect)?;
        Ok(surface)
    }
}

impl WindowCanvas {
    pub fn new(inner: SdlCanvas<Window>) -> Self {
        let texture_creator = inner.texture_creator();
        let text_renderer = TextRenderer::new(inner.draw_color(), Color::RGBA(0, 0, 0, 0));
        Self {
            inner,
            texture_creator,
            text_renderer,
        }
    }
}

impl<T: RenderTarget, U> Canvas<T, U> {
    pub fn texture_creator(&self) -> &TextureCreator<U> {
        &self.texture_creator
    }

    pub fn texture_creator_mut(&mut self) -> &mut TextureCreator<U> {
        &mut self.texture_creator
    }

    pub fn text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }

    pub fn text_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }

pub fn set_draw_color<C: Into<Color>>(&mut self, color: C) {
    let color = color.into();
    self.text_renderer.fg_color = color;
    self.inner.set_draw_color(color)
}

    pub fn draw_text<P: Into<Point>>(&mut self, text: &str, pos: P) -> Result<Texture, String> {
        let pos = pos.into();
        let surface = self.text_renderer.draw(text)?;
        let texture = surface.as_texture(&mut self.texture_creator).unwrap();
        let mut rect = surface.rect();
        rect.set_x(pos.x());
        rect.set_y(pos.y());
        self.inner.copy(&texture, None, rect)?;
        Ok(texture)
    }
}

impl<T: RenderTarget, U> Deref for Canvas<T, U> {
    type Target = SdlCanvas<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: RenderTarget, U> DerefMut for Canvas<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
