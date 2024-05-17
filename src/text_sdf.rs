use easy_signed_distance_field as sdf;
use ahash::{AHashSet, AHashMap};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub struct Font
{
    font: sdf::Font
}

impl Font
{
    pub fn new(data: &[u8]) -> Self
    {
        let font = sdf::Font::from_bytes(data, sdf::FontSettings::default()).unwrap();
        Self { font }
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
	pub fn new<I: IntoIterator<Item = char>>(font: Font, px: f32, chars: I, texture_size: u32, padding: u32) -> (Vec<Vec<u8>>, Self)
	{
		let mut builder = AtlasBuilder::new(font, px, texture_size, padding);
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

	pub fn height(&self, text: &str, layout: Layout) -> u32
	{
		let mut lines = 0;
		for block in text.split('\n')
		{
			let mut words = 0;
			let mut base_x = 0.0;
			for word in block.split(' ')
			{
				let word_length: f32 = word.chars().map(|ch| match self.glyphs.get(&ch)
				{
					Some(glyph) => glyph,
					None => match &self.default_glyph
					{
						Some(glyph) => glyph,
						None => panic!("Atlas::text: Atlas does not contain \'{}\'.", ch)
					}
				}).map(|glyph| glyph.h_advance).sum();
				if layout.auto_wrap && words > 0 && base_x + self.space + word_length > layout.width
				{
					lines += 1;
					words = 0;
					base_x = 0.0;
				}
				base_x += if words == 0 { word_length } else { self.space + word_length };
				words += 1;
			}
			if words > 0 { lines += 1; }
		}
		lines
	}

	pub fn default(&mut self, glyph: Option<char>)
	{
		self.default_glyph = glyph.as_ref().map(|ch| self.glyphs.get(ch).expect(&format!("Atlas::text: Atlas does not contain \'{}\'.", ch)).clone());
	}
}

pub struct AtlasBuilder
{
	font: sdf::Font,
	texture_size: u32,
	padding: u32,
	p0: (u32, u32),
	row_height: u32,
	layers: Vec<Vec<u8>>,
	coords_norm: f32,
    px: f32,
	height: f32,
	atlas: Atlas
}

impl AtlasBuilder
{
	fn new_layer(layers: &mut Vec<Vec<u8>>, texture_size: u32)
	{
		let mut layer = Vec::new();
		layer.resize((texture_size * texture_size) as usize, 0);
		layers.push(layer);
	}

	pub fn new(font: Font, px: f32, texture_size: u32, padding: u32) -> Self
	{
		let mut layers = Vec::new();
		Self::new_layer(&mut layers, texture_size);
		let coords_norm = 1.0 / texture_size as f32;
        let font = font.font;
		let (atlas, height) =
		{
			let metrics = font.horizontal_line_metrics(px);
			let height = metrics.new_line_size;
			let ascent = metrics.ascent / height;
			let space = font.metrics(' ', px).unwrap().advance_width / height;
			(Atlas { glyphs: AHashMap::new(), ascent, space, default_glyph: None }, height)
		};
		Self
		{
			font,
			texture_size,
			padding,
			p0: (0, 0),
			row_height: 0,
			layers,
			coords_norm,
            px,
			height,
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
				let metrics = self.font.metrics(ch, self.px).expect(&format!("AtlasBuild::add: Font does not contain \'{ch}\'."));
				(ch, metrics)
			})
			.collect();
		 //improve packaging
		chars.sort_by(|(_, m1), (_, m2)| m1.height.cmp(&m2.height));
		let mut i0 = 0;
		while
			i0 < chars.len() - 1
		 && (chars[i0].1.height as u32) < self.row_height
		 && (chars[i0 + 1].1.height as u32) < self.row_height
		{ i0 = (i0 + 1) % chars.len(); }
		chars.rotate_left(i0);

		let (x0, y0) = (&mut self.p0.0, &mut self.p0.1);
		for (ch, metrics) in chars
		{
			let (sdf_width, sdf_height) = (metrics.width as u32, metrics.height as u32);
			let sdf_padding = (sdf_width / 4).min(sdf_height / 4).max(1); //padding = size / 4 gives offset = size / 6 (weird stuff) 
			let (ch_xoff, ch_yoff, ch_width, ch_height) =
			{
				let (width, height, padding) = (sdf_width as f32, sdf_height as f32, sdf_padding as f32);
				let (xoff, yoff) = ((width * padding) / (width + 2.0 * padding), (height * padding) / (height + 2.0 * padding));
				let (width, height) = (width - 2.0 * xoff, height - 2.0 * yoff);
				(xoff, yoff, width, height)
			};
			let (xmin, ymax) = (metrics.bounds.xmin, -metrics.bounds.ymin);
			let (xmax, ymin) = (xmin + metrics.bounds.width, ymax - metrics.bounds.height);
			if *x0 + sdf_width >= self.texture_size
			{
				if sdf_width >= self.texture_size { panic!("AtlasBuilder::add: \'{ch}\' is too wide."); }
				*x0 = 0;
				*y0 += self.row_height + self.padding;
				self.row_height = 0;
			}
            self.row_height = self.row_height.max(sdf_height);
			if *y0 + sdf_height >= self.texture_size
			{
				if sdf_height >= self.texture_size { panic!("AtlasBuilder::add: \'{ch}\' is too high."); }
				*x0 = 0;
				*y0 = 0;
				Self::new_layer(&mut self.layers, self.texture_size);
			}
			let layer = self.layers.len() - 1;
			let buffer = &mut self.layers[layer];
			let sdf = self.font.sdf_generate(self.px, sdf_padding as i32, 2.0, ch).unwrap().1;
			for y in 0..sdf.height
			{
				for x in 0..sdf.width
				{
					buffer[((y + *y0) * self.texture_size + x + *x0) as usize] = (sdf.buffer[(y * sdf.width + x) as usize] * 255.0) as u8;
				}
			}
    		let glyph = Glyph
			{
				coords_min: ((*x0 as f32 + ch_xoff - 0.5) * self.coords_norm, (*y0 as f32 + ch_yoff - 0.5) * self.coords_norm),
				coords_max: ((*x0 as f32 + ch_yoff + ch_width + 0.5) * self.coords_norm, (*y0 as f32 + ch_yoff + ch_height + 0.5) * self.coords_norm),
				layer: layer as u32,
				pos_min: (xmin / self.height, ymin / self.height),
				pos_max: (xmax / self.height, ymax / self.height),
				h_advance: metrics.advance_width / self.height
			};
			self.atlas.glyphs.insert(ch, glyph);
			*x0 += sdf_width + self.padding;
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

	pub fn into_font(self) -> Font
	{
		Font { font: self.font }
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
		let (mut sdf, _) = Atlas::new(Font::new(include_bytes!("../res/Latinia.ttf")), 64.0, &(&Font::digits() | &Font::all_letters()) | &Font::text_special_characters(), 1024, 5);
		assert_eq!(sdf.len(), 1);
		let image = GrayImage::from_raw(1024, 1024, sdf.pop().unwrap()).unwrap();
		image.save_with_format("all_letters.png", ImageFormat::Png).unwrap();
	}
}
