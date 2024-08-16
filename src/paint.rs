pub use crate::math::{Vec2, Rect};
pub use crate::color::Color;

use crate::text_sdf::{Font, AtlasBuilder, Align, Layout};

pub const TEXTURE_SIZE: u32 = 1024;
const TEXTURE_PADDING: u32 = 5;

pub struct Vertex
{
    pub position: Vec2,
    pub color: Color,
    pub tex_coords: Option<(f32, f32, u32)>
}

pub struct Frame<'a>
{
    pub new: bool,
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
    pub font_version: u64,
    pub font_data: &'a Vec<Vec<u8>>
}

pub struct Painter
{
    text_version: u64,
    text: Option<AtlasBuilder>,
    origin: Vec2,
    scale: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    new: bool
}

impl Painter
{
    fn atlas_builder(font: Font, scale: f32) -> AtlasBuilder
    {
        let mut builder = AtlasBuilder::new(font, 3.0 * scale, TEXTURE_SIZE, TEXTURE_PADDING);
        builder.add(&(&Font::digits() | &Font::all_letters()) | &Font::text_special_characters());
        builder.atlas_mut().default(Some('?'));
        builder
    }

    pub fn new(font: Font) -> Self
    {
        Self
        {
            text_version: 0,
            text: Some(Self::atlas_builder(font, 1.0)),
            origin: Vec2(0.0, 0.0),
            scale: 1.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            new: true
        }
    }

    pub fn set_offset(&mut self, offset: Vec2)
    {
        self.origin = offset;
    }

    pub fn add_offset(&mut self, offset: Vec2)
    {
        self.origin += offset;
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color)
    {
        let min = self.origin + rect.min;
        let max = self.origin + rect.max;
        let i0 = self.vertices.len() as u16;
        for pos in [min, Vec2(min.0, max.1), max, Vec2(max.0, min.1)] { self.vertices.push(Vertex { position: pos * self.scale, color, tex_coords: None }); }
        for i in [0, 1, 2, 2, 3, 0] { self.indices.push(i0 + i); }
    }

    pub fn draw_rhombus(&mut self, rect: Rect, color: Color)
    {
        let min = self.origin + rect.min;
        let max = self.origin + rect.max;
        let size = rect.size();
        let i0 = self.vertices.len() as u16;
        for pos in
        [
            Vec2(min.0, min.1 + size.1 / 2.0), //left
            Vec2(min.0 + size.0 / 2.0, max.1), //bottom
            Vec2(max.0, min.1 + size.1 / 2.0), //right
            Vec2(min.0 + size.0 / 2.0, min.1) //top
        ] { self.vertices.push(Vertex { position: pos * self.scale, color, tex_coords: None }); }
        for i in [0, 1, 2, 2, 3, 0] { self.indices.push(i0 + i); }
    }

    pub fn draw_text(&mut self, rect: Rect, text: &str, size: f32, align: Align, auto_wrap: bool, color: Color)
    {
        self.add_glyphs(text);
        let atlas_builder = self.text.as_mut().unwrap();
        let width = (rect.max.0 - rect.min.0) / size;
        let offset = self.origin + rect.min;
        let i0 = self.vertices.len() as u16;
        atlas_builder.atlas().text
        (
            text,
            Layout { width, align, auto_wrap },
            |index| self.indices.push(i0 + index as u16),
            |tex_coords, position| self.vertices.push(Vertex { position: (Vec2::from(position) * size + offset) * self.scale, color, tex_coords: Some(tex_coords) })
        );
    }

    pub fn clear_frame(&mut self, scale: f32)
    {
        if self.scale != scale
        {
            self.text = Some(Self::atlas_builder(self.text.take().unwrap().into_font(), scale));
            self.text_version += 1;
        }
        self.origin = Vec2(0.0, 0.0);
        self.scale = scale;
        self.vertices.clear();
        self.indices.clear();
        self.new = true;
    }

    pub fn get_frame(&mut self) -> Frame
    {
        let new = self.new;
        self.new = false;
        Frame { new, vertices: &self.vertices, indices: &self.indices, font_version: self.text_version, font_data: self.text.as_ref().unwrap().sdf() }
    }

    pub fn text_width(&mut self, text: &str, size: f32) -> f32
    {
        self.add_glyphs(text);
        self.text.as_ref().unwrap().atlas().width(text) * size
    }

    pub fn text_height(&mut self, text: &str, layout: Layout) -> u32
    {
        self.add_glyphs(text);
        self.text.as_ref().unwrap().atlas().height(text, layout)
    }

    fn add_glyphs(&mut self, text: &str)
    {
        if self.text.as_mut().unwrap().add(text.chars()) { self.text_version += 1; }
    }
}
