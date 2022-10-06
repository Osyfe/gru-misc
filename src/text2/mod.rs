mod sdf;

use ttf_parser::Face;
use ahash::{AHashSet, AHashMap};
use crate::math::{Vec2, Rect};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub struct Font<'a>
{
    face: Face<'a>
}

impl<'a> Font<'a>
{
    pub fn new(data: &'a [u8]) -> Self
    {
        let face = Face::from_slice(data, 0).unwrap();
        Self { face }
    }

    pub fn digits() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in 48..=57 { chars.insert(i.into()); }
    	chars
	}

	pub fn lowercase_letters() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in 97..=122 { chars.insert(i.into()); }
    	chars
	}

	pub fn uppercase_letters() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in 65..=90 { chars.insert(i.into()); }
    	chars
	}

	pub fn all_letters() -> AHashSet<char>
	{
		&Self::lowercase_letters() | &Self::uppercase_letters()
	}

	pub fn text_special_characters() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in 33..=34 { chars.insert(i.into()); } // !"
    	for i in 37..=41 { chars.insert(i.into()); } // %&'()
    	for i in 44..=47 { chars.insert(i.into()); } // ,-./
    	for i in 58..=59 { chars.insert(i.into()); } // :;
    	for i in 63..=64 { chars.insert(i.into()); } // ?@
    	for i in 95..=96 { chars.insert(i.into()); } // _`
    	chars
	}

	pub fn other_special_characters() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in 35..=36 { chars.insert(i.into()); } // #$
    	for i in 42..=43 { chars.insert(i.into()); } // *+
    	for i in 60..=62 { chars.insert(i.into()); } // <=>
    	for i in 91..=94 { chars.insert(i.into()); } // [\]^
    	for i in 123..=126 { chars.insert(i.into()); } // {|}~
    	chars
	}

	pub fn all_special_characters() -> AHashSet<char>
	{
		&Self::text_special_characters() | &Self::other_special_characters()
	}

	pub fn german_extra() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in &['ä', 'ö', 'ü', 'ß', 'Ä', 'Ö', 'Ü', 'ẞ'] { chars.insert(*i); }
    	chars
	}

	pub fn vocal_accents() -> AHashSet<char>
	{
		let mut chars = AHashSet::new();
    	for i in ['á', 'à', 'â', 'é', 'è', 'ê', 'í', 'ì', 'î', 'ó', 'ò', 'ô', 'ú', 'ù', 'û', 'Á', 'À', 'Â', 'É', 'È', 'Ê', 'Í', 'Ì', 'Î', 'Ó', 'Ò', 'Ô', 'Ú', 'Ù', 'Û'] { chars.insert(i); }
    	chars
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Glyph
{
	coords_min: (f32, f32),
	coords_max: (f32, f32),
	layer: u32,
	pos_min: (f32, f32),
	pos_max: (f32, f32),
	h_advance: f32
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Atlas
{
	glyphs: AHashMap<char, Glyph>,
	ascent: f32,
	space: f32,
	default_glyph: Option<Glyph>
}

impl Atlas
{
	pub fn new<I: IntoIterator<Item = char>>(font: Font, scale: f32, chars: I, texture_size: u32, padding: u32) -> (Vec<Vec<u8>>, Self)
	{
		let mut builder = AtlasBuilder::new(font, scale, texture_size, padding);
		builder.add(chars);
		let (bitmap, atlas) = builder.finish();
		(bitmap, atlas)
	}

	pub fn contains(&self, ch: char) -> bool
	{
		self.glyphs.contains_key(&ch)
	}

	pub fn text<I: FnMut(u32), V: FnMut((f32, f32, u32), (f32, f32))>(&self, text: &str, layout: Layout, mut index: I, mut vertex: V) -> TextData
	{
		struct Line<'a>
		{
			words: Vec<Vec<&'a Glyph>>,
			offset: f32,
			space: f32
		}
		let mut lines = Vec::new();
		for block in text.split('\n')
		{
			let mut line = Line { words: Vec::new(), offset: 0.0, space: self.space };
			let mut line_length = 0.0;
			let mut base_x = 0.0;
			for word in block.split(' ')
			{
				let word: Vec<_> = word.chars().map(|ch| match self.glyphs.get(&ch)
				{
					Some(glyph) => glyph,
					None => match &self.default_glyph
					{
						Some(glyph) => glyph,
						None => panic!("Atlas::text: Atlas does not contain \'{}\'.", ch)
					}
				}).collect();
				let word_length: f32 = word.iter().map(|glyph| glyph.h_advance).sum();
				if layout.auto_wrap && line.words.len() > 0 && base_x + self.space + word_length > layout.width
				{
					let margin = layout.width - base_x;
					match layout.align
					{
						Align::Left => {},
						Align::Right => line.offset = margin,
						Align::Center => line.offset = margin / 2.0,
						Align::Block => line.space = (layout.width - line_length) / (line.words.len() - 1).max(1) as f32
					};
					lines.push(line);
					line = Line { words: Vec::new(), offset: 0.0, space: self.space };
					line_length = 0.0;
					base_x = 0.0;
				}
				line_length += word_length;
				base_x += if line.words.len() == 0 { word_length } else { self.space + word_length };
				line.words.push(word);
			}
			if line.words.len() > 0
			{
				let margin = layout.width - base_x;
				match layout.align
				{
					Align::Left => {},
					Align::Right => line.offset = margin,
					Align::Center => line.offset = margin / 2.0,
					Align::Block => {}
				};
				lines.push(line);
			}
		}

		let mut base_index = 0;
		let mut base_y = self.ascent;
		for line in &lines
		{
			let mut base_x = line.offset;
			for word in &line.words
			{
				for glyph in word
				{
					index(base_index + 0);
					index(base_index + 1);
					index(base_index + 2);
					index(base_index + 2);
					index(base_index + 3);
					index(base_index + 0);
					base_index += 4;

					let layer = glyph.layer;
					let (coords_x_min, coords_y_min) = glyph.coords_min;
					let (coords_x_max, coords_y_max) = glyph.coords_max;
					let (pos_x_min, pos_y_min) = glyph.pos_min;
					let (pos_x_max, pos_y_max) = glyph.pos_max;
					vertex((coords_x_min, coords_y_min, layer), (pos_x_min + base_x, pos_y_min + base_y));
					vertex((coords_x_min, coords_y_max, layer), (pos_x_min + base_x, pos_y_max + base_y));
					vertex((coords_x_max, coords_y_max, layer), (pos_x_max + base_x, pos_y_max + base_y));
					vertex((coords_x_max, coords_y_min, layer), (pos_x_max + base_x, pos_y_min + base_y));
					base_x += glyph.h_advance;
				}
				base_x += line.space;
			}
			base_y += 1.0;
		}
		TextData
		{
			index_count: base_index / 4 * 6,
			vertex_count: base_index,
			line_count: lines.len() as u32
		}
	}

	pub fn width(&self, text: &str) -> f32
	{
		text.chars().map(|ch|
		{
			if ch == ' ' { self.space }
			else
			{
				match self.glyphs.get(&ch)
				{
					Some(glyph) => glyph.h_advance,
					None => match &self.default_glyph
					{
						Some(glyph) => glyph.h_advance,
						None => panic!("Atlas::text: Atlas does not contain \'{}\'.", ch)
					}
				}
			}
		}).sum()
	}

	pub fn default(&mut self, glyph: Option<char>)
	{
		self.default_glyph = glyph.as_ref().map(|ch| self.glyphs.get(ch).expect(&format!("Atlas::text: Atlas does not contain \'{}\'.", ch)).clone());
	}
}

pub struct AtlasBuilder<'a>
{
	face: Face<'a>,
	texture_size: u32,
	padding: u32,
	p0: (u32, u32),
	row_height: u32,
	layers: Vec<Vec<u8>>,
	coords_norm: f32,
    scale: f32,
	atlas: Atlas
}

impl<'a> AtlasBuilder<'a>
{
	fn new_layer(layers: &mut Vec<Vec<u8>>, texture_size: u32)
	{
		let mut layer = Vec::new();
		layer.resize((texture_size * texture_size) as usize, 0);
		layers.push(layer);
	}

	//scale = screen_scale_factor * text_pixel_height = pixel_height for standard display
	pub fn new(font: Font<'a>, scale: f32, texture_size: u32, padding: u32) -> Self
	{
		let mut layers = Vec::new();
		Self::new_layer(&mut layers, texture_size);
		let coords_norm = 1.0 / texture_size as f32;
        let face = font.face;
		let atlas =
		{
			//https://docs.rs/ab_glyph/0.2.11/ab_glyph/trait.Font.html#units
			let px_per_em = scale * (96.0 / 72.0);
			let height = face.height() as f32;
			let units_per_em = face.units_per_em() as f32;
			let scale = px_per_em * height / units_per_em;
			//init metrics
			let ascent = face.line_gap() as f32 / scale;
			let space = face.glyph_hor_advance(face.glyph_index(' ').unwrap()).unwrap() as f32 / scale;
			Atlas { glyphs: AHashMap::new(), ascent, space, default_glyph: None }
		};
		Self
		{
			face,
			texture_size,
			padding,
			p0: (0, 0),
			row_height: 0,
			layers,
			coords_norm,
            scale,
			atlas
		}
	}

	pub fn add<I: IntoIterator<Item = char>>(&mut self, chars: I) -> bool
	{
		let chars: ahash::AHashSet<_> = chars
			.into_iter()
			.filter(|ch| *ch != ' ' && !self.atlas.contains(*ch))
			.collect();
		if chars.len() == 0 { return false; }
		let mut chars: Vec<_> = chars
			.into_iter()
			.map(|ch|
			{
                let glyph = self.face.glyph_index(ch).expect(&format!("AtlasBuild::add: Font does not contain \'{ch}\'."));
				let bounds = self.face.glyph_bounding_box(glyph).expect(&format!("AtlasBuilder::add: \'{ch}\' has no bounding box."));
				let bounds = Rect { min: Vec2(bounds.x_min as f32, bounds.y_min as f32) * self.scale, max: Vec2(bounds.x_max as f32, bounds.y_max as f32) * self.scale };
				(ch, glyph, bounds)
			})
			.collect();
		 //improve packaging
		chars.sort_by(|(_, _, b1), (_, _, b2)| (b1.height() as u32).cmp(&(b2.height() as u32)));
		let mut i0 = 0;
		while
			i0 < chars.len() - 1
		 && (chars[i0].2.height() as u32) < self.row_height
		 && (chars[i0 + 1].2.height() as u32) < self.row_height
		{ i0 = (i0 + 1) % chars.len(); }
		chars.rotate_left(i0);

		let (x0, y0) = (&mut self.p0.0, &mut self.p0.1);
		for (ch, glyph, bounds) in chars
		{
			let width = bounds.width().ceil() as u32;
			let height = bounds.height().ceil() as u32;
			if *x0 + width >= self.texture_size
			{
				if *x0 == 0 { panic!("AtlasBuilder::add: \'ch\' is too wide."); }
				*x0 = 0;
				*y0 += self.row_height + self.padding;
				self.row_height = 0;
			}
            self.row_height = self.row_height.max(height);
			if *y0 + height >= self.texture_size
			{
				if *y0 == 0 { panic!("AtlasBuilder::add: \'{ch}\' is too high."); }
				*x0 = 0;
				*y0 = 0;
				Self::new_layer(&mut self.layers, self.texture_size);
			}
			let layer = self.layers.len() - 1;
			let buffer = &mut self.layers[layer];
			sdf::sdf(&self.face, glyph, bounds, self.scale, |x, y, d| buffer[((y + *y0) * self.texture_size + x + *x0) as usize] = d);
    		let glyph = Glyph
			{
				coords_min: ((*x0 as f32 - 0.5) * self.coords_norm, (*y0 as f32 - 0.5) * self.coords_norm),
				coords_max: ((*x0 as f32 + bounds.width() as f32 + 0.5) * self.coords_norm, (*y0 as f32 + bounds.height() as f32 + 0.5) * self.coords_norm),
				layer: layer as u32,
				pos_min: (bounds.min.0 as f32 / self.scale, bounds.min.1 as f32 / self.scale),
				pos_max: (bounds.max.0 as f32 / self.scale, bounds.max.1 as f32 / self.scale),
				h_advance: self.face.glyph_hor_advance(glyph).unwrap() as f32 / self.scale
			};
			self.atlas.glyphs.insert(ch, glyph);
			*x0 += bounds.width() as u32 + self.padding;
		}
		true
	}

	pub fn sdf(&self) -> &Vec<Vec<u8>>
	{
		&self.layers
	}

	pub fn atlas(&self) -> &Atlas
	{
		&self.atlas
	}

	pub fn atlas_mut(&mut self) -> &mut Atlas
	{
		&mut self.atlas
	}

	pub fn into_font(self) -> Font<'a>
	{
		Font { face: self.face }
	}

	pub fn finish(self) -> (Vec<Vec<u8>>, Atlas)
	{
		(self.layers, self.atlas)
	}
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Align
{
	Left,
	Right,
	Center,
	Block
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layout
{
	pub width: f32,
	pub align: Align,
	pub auto_wrap: bool
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextData
{
	pub index_count: u32,
	pub vertex_count: u32,
	pub line_count: u32
}

#[cfg(test)]
mod tests
{
	use super::*;
	use image::{GrayImage, ImageFormat};
	
	#[test]
	fn all_letters()
	{
		let (mut sdf, _) = Atlas::new(Font::new(include_bytes!("../res/futuram.ttf")), 0.1, Font::all_letters(), 1024, 10);
		assert_eq!(sdf.len(), 1);
		let image = GrayImage::from_raw(1024, 1024, sdf.pop().unwrap()).unwrap();
		image.save_with_format("all_letters.png", ImageFormat::Png).unwrap();
	}
}
