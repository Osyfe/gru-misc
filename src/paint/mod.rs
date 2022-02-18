mod math;
pub use math::*;

use crate::text::{Font, AtlasBuilder, Align, Layout};

pub const TEXTURE_SIZE: u32 = 1024;
const TEXTURE_PADDING: u32 = 5;

#[derive(Clone, Copy)]
pub enum TextSize
{
    Small,
    Normal,
    Large
}

impl TextSize
{
    const NUM: usize = 3;
    const SIZES: [f32; Self::NUM] = [0.5, 1.0, 2.0];

    fn i(&self) -> usize
    {
        match self
        {
            Self::Small => 0,
            Self::Normal => 1,
            Self::Large => 2
        }
    }

    pub fn scale(&self) -> f32
    {
        Self::SIZES[self.i()]
    }
}

pub struct Vertex
{
    pub position: Vec2,
    pub color: (f32, f32, f32, f32),
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

pub struct Painter<'a>
{
    text_version: u64,
    text: Option<AtlasBuilder<'a, {TextSize::NUM}>>,
    origin: Vec2,
    scale: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    new: bool
}

impl<'a> Painter<'a>
{
    fn atlas_builder(font: Font<'a>, scale: f32) -> AtlasBuilder<'a, {TextSize::NUM}>
    {
        let mut builder = AtlasBuilder::new(font, TextSize::SIZES.map(|size| size * scale), TEXTURE_SIZE, TEXTURE_PADDING);
        for i in 0..TextSize::NUM
        {
            builder.add(i, std::iter::once('?'));
            builder.atlas_mut(i).default(Some('?'));
        }
        builder
    }

    pub fn new(font: Font<'a>) -> Self
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

    pub fn draw_rect(&mut self, rect: Rect, color: (f32, f32, f32, f32))
    {
        let min = self.origin + rect.min;
        let max = self.origin + rect.max;
        let i0 = self.vertices.len() as u16;
        for pos in [min, Vec2(min.0, max.1), max, Vec2(max.0, min.1)] { self.vertices.push(Vertex { position: pos * self.scale, color, tex_coords: None }); }
        for i in [0, 1, 2, 2, 3, 0] { self.indices.push(i0 + i); }
    }

    pub fn draw_text(&mut self, rect: Rect, text: &str, size: TextSize, align: Align, auto_wrap: bool, color: (f32, f32, f32, f32))
    {
        self.add_glyphs(text, size);
        let atlas_builder = self.text.as_mut().unwrap();
        let i = size.i();
        let size = TextSize::SIZES[i];
        let width = (rect.max.0 - rect.min.0) / size;
        let offset = self.origin + rect.min + Vec2(0.0, (rect.max.1 - rect.min.1 - size) / 2.0);
        let i0 = self.vertices.len() as u16;
        atlas_builder.atlas(i).text
        (
            text,
            Layout { width, align, auto_wrap },
            |index| self.indices.push(i0 + index as u16),
            |tex_coords, position| self.vertices.push(Vertex { position: (Vec2::from(position) * size + offset) * self.scale, color, tex_coords: Some(tex_coords) })
        );
    }

    pub fn clear_frame(&mut self, scale: f32)
    {
        if self.scale != scale { self.text = Some(Self::atlas_builder(self.text.take().unwrap().into_font(), scale)); }
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
        Frame { new, vertices: &self.vertices, indices: &self.indices, font_version: self.text_version, font_data: self.text.as_ref().unwrap().bitmap() }
    }

    pub fn text_width(&mut self, text: &str, size: TextSize) -> f32
    {
        self.add_glyphs(text, size);
        self.text.as_ref().unwrap().atlas(size.i()).width(text) * size.scale()
    }

    fn add_glyphs(&mut self, text: &str, size: TextSize)
    {
        if self.text.as_mut().unwrap().add(size.i(), text.chars()) { self.text_version += 1; }
    }
}
