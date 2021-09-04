#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use ab_glyph;
use std::collections::{HashSet, HashMap};

pub struct Font<'a>
{
	font: ab_glyph::FontRef<'a>
}

impl<'a> Font<'a>
{
	pub fn new(data: &'a [u8]) -> Self
	{
		Font { font: ab_glyph::FontRef::try_from_slice(data).unwrap() }
	}

	pub fn digits() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in 48..=57 { chars.insert(i.into()); }
    	chars
	}

	pub fn lowercase_letters() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in 97..=122 { chars.insert(i.into()); }
    	chars
	}

	pub fn uppercase_letters() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in 65..=90 { chars.insert(i.into()); }
    	chars
	}

	pub fn all_letters() -> HashSet<char>
	{
		&Self::lowercase_letters() | &Self::uppercase_letters()
	}

	pub fn text_special_characters() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in 33..=34 { chars.insert(i.into()); } // !"
    	for i in 37..=41 { chars.insert(i.into()); } // %&'()
    	for i in 44..=47 { chars.insert(i.into()); } // ,-./
    	for i in 58..=59 { chars.insert(i.into()); } // :;
    	for i in 63..=64 { chars.insert(i.into()); } // ?@
    	for i in 95..=96 { chars.insert(i.into()); } // _`
    	chars
	}

	pub fn other_special_characters() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in 35..=36 { chars.insert(i.into()); } // #$
    	for i in 42..=43 { chars.insert(i.into()); } // *+
    	for i in 60..=62 { chars.insert(i.into()); } // <=>
    	for i in 91..=94 { chars.insert(i.into()); } // [\]^
    	for i in 123..=126 { chars.insert(i.into()); } // {|}~
    	chars
	}

	pub fn all_special_characters() -> HashSet<char>
	{
		&Self::text_special_characters() | &Self::other_special_characters()
	}

	pub fn german_extra() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in &['ä', 'ö', 'ü', 'ß', 'Ä', 'Ö', 'Ü', 'ẞ'] { chars.insert(*i); }
    	chars
	}

	pub fn vocal_accents() -> HashSet<char>
	{
		let mut chars = HashSet::new();
    	for i in ['á', 'à', 'â', 'é', 'è', 'ê', 'í', 'ì', 'î', 'ó', 'ò', 'ô', 'ú', 'ù', 'û', 'Á', 'À', 'Â', 'É', 'È', 'Ê', 'Í', 'Ì', 'Î', 'Ó', 'Ò', 'Ô', 'Ú', 'Ù', 'Û'] { chars.insert(i); }
    	chars
	}
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Atlas
{
	glyphs: HashMap<char, Glyph>,
	ascent: f32,
	space: f32,
	default_glyph: Option<char>
}

impl Atlas
{
	pub fn new(font: &Font, chars: &HashSet<char>, resolution: f32, texture_size: u32, padding: u32) -> (Vec<Vec<u8>>, Self)
	{
		use ab_glyph::{Font, ScaleFont};
		let font = font.font.as_scaled(resolution);
		let mut chars: Vec<_> = chars.iter().map(|ch|
		{
			let glyph = font.outline_glyph(font.scaled_glyph(*ch)).expect(&format!("Atlas::new: Font does not contain \'{}\'.", ch));
			let bounds = glyph.px_bounds();
			(*ch, glyph, bounds)
		}).collect();
		chars.sort_by(|(_, _, b1), (_, _, b2)| (b1.height() as u32).cmp(&(b2.height() as u32))); //improves packaging

		let coords_norm = 1.0 / texture_size as f32;
		let pos_norm = 1.0 / (font.height() + font.line_gap());
		let mut glyphs = HashMap::new();
		let mut layers = Vec::new();
		fn new_layer(layers: &mut Vec<Vec<u8>>, texture_size: u32)
		{
			let mut layer = Vec::new();
			layer.resize((texture_size * texture_size) as usize, 0);
			layers.push(layer);
		}
		new_layer(&mut layers, texture_size);
		
		let mut x0 = 0;
		let mut y0 = 0;
		let mut row_height = 0;
		for (ch, glyph, bounds) in chars
		{
			let width = bounds.width() as u32;
			let height = bounds.height() as u32;
			row_height = row_height.max(height);
			if x0 + width >= texture_size
			{
				if x0 == 0 { panic!("Atlas::new: \'{}\' is too wide.", ch); }
				x0 = 0;
				y0 += row_height + padding;
				row_height = 0;
			}
			if y0 + height >= texture_size
			{
				if y0 == 0 { panic!("Atlas::new: \'{}\' is too high.", ch); }
				x0 = 0;
				y0 = 0;
				new_layer(&mut layers, texture_size);
			}
			let layer = layers.len() - 1;
			let buffer = &mut layers[layer];
    		glyph.draw(|x, y, c| buffer[((y + y0) * texture_size + x + x0) as usize] = (c * 255.0).round() as u8);
    		let glyph = Glyph
			{
				coords_min: ((x0 as f32 - 0.5) * coords_norm, (y0 as f32 - 0.5) * coords_norm),
				coords_max: ((x0 as f32 + bounds.width() + 0.5) * coords_norm, (y0 as f32 + bounds.height() + 0.5) * coords_norm),
				layer: layer as u32,
				pos_min: (bounds.min.x * pos_norm, bounds.min.y * pos_norm),
				pos_max: (bounds.max.x * pos_norm, bounds.max.y * pos_norm),
				h_advance: font.h_advance(glyph.glyph().id) * pos_norm
			};
			glyphs.insert(ch, glyph);
			x0 += bounds.width() as u32 + padding;
		}
		let ascent = font.ascent() * pos_norm;
		let space = font.h_advance(font.glyph_id(' ')) * pos_norm;
		(layers, Self { glyphs, ascent, space, default_glyph: None })
	}

	pub fn text(&self, text: &str, width: f32, align: Align, index: &mut dyn FnMut(u32), vertex: &mut dyn FnMut((f32, f32, f32), (f32, f32))) -> TextData
	{
		let default_glyph = self.default_glyph.as_ref().map(|ch| self.glyphs.get(ch).expect(&format!("Atlas::text: Atlas does not contain \'{}\'.", ch)));
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
					None => match default_glyph
					{
						Some(glyph) => glyph,
						None => panic!("Atlas::text: Atlas does not contain \'{}\'.", ch)
					}
				}).collect();
				let word_length: f32 = word.iter().map(|glyph| glyph.h_advance).sum();
				if line.words.len() > 0 && base_x + self.space + word_length > width
				{
					let margin = width - base_x;
					match align
					{
						Align::Left => {},
						Align::Right => line.offset = margin,
						Align::Center => line.offset = margin / 2.0,
						Align::Block => line.space = (width - line_length) / (line.words.len() - 1).max(1) as f32
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
				let margin = width - base_x;
				match align
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
		for line in lines
		{
			let mut base_x = line.offset;
			for word in line.words
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

					let layer = glyph.layer as f32;
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
			line_count: (base_y - self.ascent).round() as u32
		}
	}

	pub fn default(&mut self, glyph: Option<char>)
	{
		self.default_glyph = glyph;
	}
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Align
{
	Left,
	Right,
	Center,
	Block
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextData
{
	pub index_count: u32,
	pub vertex_count: u32,
	pub line_count: u32
}
