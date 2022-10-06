use ttf_parser::{Face, GlyphId, OutlineBuilder};
use crate::math::{Vec2, Rect};

pub fn sdf(face: &Face, glyph: GlyphId, bounds: Rect, scale: f32, mut fill: impl FnMut(u32, u32, u8))
{
    let mut shape = Shape { pos: Vec2::zero(), scale, curves: Vec::new() };
    face.outline_glyph(glyph, &mut shape);
    for curve in shape.curves
    {
        let p0 = curve.p0();
        fill(p0.0 as u32, p0.1 as u32, 255);
    }
}

struct Shape
{
    pos: Vec2,
    scale: f32,
    curves: Vec<Curve>
}

impl OutlineBuilder for Shape
{
    fn move_to(&mut self, x: f32, y: f32)
    {
        self.pos = Vec2(x, y) * self.scale;
    }

    fn line_to(&mut self, x: f32, y: f32)
    {
        let new_pos = Vec2(x, y) * self.scale;
        self.curves.push(Curve::Linear { p0: self.pos, p1: new_pos });
        self.pos = new_pos;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32)
    {
        let new_pos = Vec2(x, y) * self.scale;
        let c1 = Vec2(x1, y1) * self.scale;
        self.curves.push(Curve::Quadratic { p0: self.pos, p1: new_pos, c1 });
        self.pos = new_pos;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32)
    {
        let new_pos = Vec2(x, y) * self.scale;
        let c1 = Vec2(x1, y1) * self.scale;
        let c2 = Vec2(x2, y2) * self.scale;
        self.curves.push(Curve::Cubic { p0: self.pos, p1: new_pos, c1, c2 });
        self.pos = new_pos;
    }

    fn close(&mut self)
    {
        
    }
}

enum Curve
{
    Linear
    {
        p0: Vec2,
        p1: Vec2
    },
    Quadratic
    {
        p0: Vec2,
        p1: Vec2,
        c1: Vec2
    },
    Cubic
    {
        p0: Vec2,
        p1: Vec2,
        c1: Vec2,
        c2: Vec2
    }
}

impl Curve
{
    fn p0(&self) -> Vec2
    {
        match self
        {
            Self::Linear { p0, .. } | Self::Quadratic { p0, .. } | Self::Cubic { p0, .. } => *p0
        }
    }

    fn p1(&self) -> Vec2
    {
        match self
        {
            Self::Linear { p1, .. } | Self::Quadratic { p1, .. } | Self::Cubic { p1, .. } => *p1
        }
    }

    fn sd(&self, p: Vec2) -> f32
    {
        match self
        {
            Self::Linear { p0, p1 } =>
            {
                0.0
            },
            Self::Quadratic { p0, p1, c1 } =>
            {
                0.0
            },
            Self::Cubic { p0, p1, c1, c2 } =>
            {
                0.0
            }
        }
    }
}
