//! Provides the [`Canvas`] struct, which allows the screen to be manipulated, such as by drawing
//! points, lines, rectangles, text, or textures to it.

use std::ops::{Deref, DerefMut};

use sdl2::{
    rect::Point,
    render::{Canvas as SdlCanvas, RenderTarget, TextureCreator},
    surface::{Surface, SurfaceContext},
    video::{Window, WindowContext},
};
#[cfg(feature = "unifont")]
use sdl2::{
    pixels::Color,
    render::Texture,
};
#[cfg(feature = "unifont")]
use sdl2_unifont::renderer::SurfaceRenderer as TextRenderer;

/// A [`Canvas`] that internally renders to a [`Surface`][sdl2::surface::Surface].
pub type SurfaceCanvas<'a> = Canvas<Surface<'a>, SurfaceContext<'a>>;
/// A [`Canvas`] that renders to a [`Window`][Window] on the screen.
pub type WindowCanvas = Canvas<Window, WindowContext>;

/// This struct allows you to draw to the screen in various ways. It is a wrapper around:
/// * An [sdl2 `Canvas`][SdlCanvas], which allows you to draw points, lines, rectangles, etc, and to "blit"
///   textures and surfaces onto the screen.
/// * An [sdl2 `TextureCreator`][TextureCreator], which is linked to the sdl2 `Canvas`, for creating textures.
/// * An [sdl2-unifont `SurfaceRenderer`][TextRenderer] for rendering text to a surface.
/// This struct implements [`Deref`][std::ops::Deref] and [`DerefMut`][std::ops::DerefMut] for the sdl2 `Canvas`, so you can call any of the
/// normal drawing routines via deref coersion.
pub struct Canvas<T: RenderTarget, U> {
    inner: SdlCanvas<T>,
    texture_creator: TextureCreator<U>,
#[cfg(feature = "unifont")]
    text_renderer: TextRenderer,
#[cfg(feature = "unifont")]
    synced_colors: bool,
}

impl WindowCanvas {
    /// Create a new `Canvas` from the specified sdl2 `WindowCanvas` that draws to a window on the
    /// screen
    pub fn new(inner: SdlCanvas<Window>) -> Self {
        let texture_creator = inner.texture_creator();
#[cfg(feature = "unifont")]
        let text_renderer = TextRenderer::new(inner.draw_color(), Color::RGBA(0, 0, 0, 0));
        Self {
            inner,
            texture_creator,
#[cfg(feature = "unifont")]
            text_renderer,
#[cfg(feature = "unifont")]
            synced_colors: true,
        }
    }
}

impl<'a> SurfaceCanvas<'a> {
    /// Create a new `Canvas` from an sdl2 `SurfaceCanvas`, that draws internally to an sdl2
    /// `Surface`.
    pub fn new(inner: SdlCanvas<Surface<'a>>) -> Self {
        let texture_creator = inner.texture_creator();
#[cfg(feature = "unifont")]
        let text_renderer = TextRenderer::new(inner.draw_color(), Color::RGBA(0, 0, 0, 0));
        Self {
            inner,
            texture_creator,
#[cfg(feature = "unifont")]
            text_renderer,
#[cfg(feature = "unifont")]
            synced_colors: true,
        }
    }

    /// Draw the specified text to a point on the screen. Returns a
    /// [`Surface`][Surface] representing the
    /// rendered text, or a `String` indicating an error from sdl.
    ///
    /// This is a `Canvas<Surface>` specific alternative to [`draw_text`][Self::draw_text], which internally creates
    /// a texture for the rendered text.
#[cfg(feature = "unifont")]
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

impl<T: RenderTarget, U> Canvas<T, U> {
    /// Returns an immutable reference to the [`TextureCreator`][TextureCreator] associated with this canvas.
    pub fn texture_creator(&self) -> &TextureCreator<U> {
        &self.texture_creator
    }

    /// Returns a mutable reference to the [`TextureCreator`][TextureCreator] associated with this canvas.
    pub fn texture_creator_mut(&mut self) -> &mut TextureCreator<U> {
        &mut self.texture_creator
    }

    /// Returns an immutable reference to the [sdl2-unifont `SurfaceRenderer`][TextRenderer] for text rendering, associated with this canvas.
#[cfg(feature = "unifont")]
    pub fn text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }

    /// Returns a mutable reference to the [sdl2-unifont `SurfaceRenderer`][TextRenderer] for text rendering, associated with this canvas.
#[cfg(feature = "unifont")]
    pub fn text_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }

    /// Set the draw color for the standard sdl2 `canvas` drawing routines. If colors are
    /// synchronised (I.E. `canvas.set_text_color(None)`), Also changes the default text color.
#[cfg(feature = "unifont")]
pub fn set_draw_color<C: Into<Color>>(&mut self, color: C) {
    let color = color.into();
    if self.synced_colors {
        self.text_renderer.fg_color = color;
    }
    self.inner.set_draw_color(color)
}
        
/// If called with `Some(color)`, set the color used when rendering text. If called with `None`,
/// resynchronises the drawing and text colors.
#[cfg(feature = "unifont")]
        pub fn set_text_color<C>(&mut self, color: C)
        where C: Into<Option<Color>> {
            self.synced_colors = if let Some(color) = color.into() {
                self.text_renderer.fg_color = color;
                false
            } else {
                self.text_renderer.fg_color = self.inner.draw_color();
                true
            };
        }

    /// Draw the specified text to a point on the screen. Returns a [`Texture`] representing the
    /// rendered text, or a `String` indicating an error from sdl.
#[cfg(feature = "unifont")]
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

fn draw_circle_points(&mut self, center: Point, point: Point) -> Result<(), String> {
    let points = &[
                            Point::new(center.x() + point.x(), center.y() + point.y()),
	Point::new(center.x() - point.x(), center.y() + point.y()),
	Point::new(center.x() + point.x(), center.y() - point.y()),
	Point::new(center.x() - point.x(), center.y() - point.y()),
	Point::new(center.x() + point.y(), center.y() + point.x()),
	Point::new(center.x() - point.y(), center.y() + point.x()),
	Point::new(center.x() + point.y(), center.y() - point.x()),
	Point::new(center.x() - point.y(), center.y() - point.x()),
    ];
    self.inner.draw_points(points.as_ref())
}

fn fill_circle_lines(&mut self, center: Point, point: Point) -> Result<(), String> {
    let points = &[
                            Point::new(center.x() + point.x(), center.y() + point.y()),
	Point::new(center.x() - point.x(), center.y() + point.y()),
	Point::new(center.x() + point.x(), center.y() - point.y()),
	Point::new(center.x() - point.x(), center.y() - point.y()),
	Point::new(center.x() + point.y(), center.y() + point.x()),
	Point::new(center.x() - point.y(), center.y() + point.x()),
	Point::new(center.x() + point.y(), center.y() - point.x()),
	Point::new(center.x() - point.y(), center.y() - point.x()),
    ];
    self.inner.draw_lines(points.as_ref())
}

/// Draws a circle outline using Bresenham's algorithm, with the given center and radius.
pub fn draw_circle<P>(&mut self, center: P, radius: i32) -> Result<(), String>
where
P: Into<Point>{
    let center = center.into();
	let mut current = Point::new(0, radius);
	let mut d = 3 - 2 * radius;
	self.draw_circle_points(center, current)?;
	while current.y() >= current.x() {
		// for each pixel we will draw all eight pixels
		current = current.offset(1, 0);

		// check for decision parameter and correspondingly update d, x, y
		if d > 0 {
			current = current.offset(0, -1);
			d += 4 * (current.x() - current.y()) + 10;
		} else {
			d += 4 * current.x() + 6;
		self.draw_circle_points(center, current)?;
	}
}
Ok(())
}

/// Draws a filled circle using Bresenham's algorithm, with the given center and radius.
pub fn fill_circle<P>(&mut self, center: P, radius: i32) -> Result<(), String>
where
P: Into<Point>{
    let center = center.into();
	let mut current = Point::new(0, radius);
	let mut d = 3 - 2 * radius;
	self.fill_circle_lines(center, current)?;
	while current.y() >= current.x() {
		// for each pixel we will draw all eight pixels
		current = current.offset(1, 0);

		// check for decision parameter and correspondingly update d, x, y
		if d > 0 {
			current = current.offset(0, -1);
			d += 4 * (current.x() - current.y()) + 10;
		} else {
			d += 4 * current.x() + 6;
		self.fill_circle_lines(center, current)?;
	}
}
Ok(())
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
