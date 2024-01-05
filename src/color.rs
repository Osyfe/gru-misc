#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color(f32, f32, f32, f32); //normalized linear rgb + alpha

impl Color
{
    pub const fn from_normalized_linear(r: f32, g: f32, b: f32, a: f32) -> Self
    {
        Self(r, g, b, a)
    }

    pub fn from_discrete_linear(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        Self(normalize(r), normalize(g), normalize(b), normalize(a))
    }

    pub fn from_normalized_srgb(r: f32, g: f32, b: f32, a: f32) -> Self
    {
        Self(srgb2rgb(r), srgb2rgb(g), srgb2rgb(b), a)
    }

    pub fn from_discrete_srgb(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        Self(srgb2rgb(normalize(r)), srgb2rgb(normalize(g)), srgb2rgb(normalize(b)), normalize(a))
    }

    pub fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> Self
    {
        let (r, g, b) = hsv2srgb(h, s, v);
        Self::from_normalized_srgb(r, g, b, a)
    }

    pub fn to_normalized_linear(self) -> (f32, f32, f32, f32)
    {
        (self.0, self.1, self.2, self.3)
    }

    pub fn to_discrete_linear(self) -> (u8, u8, u8, u8)
    {
        (discretize(self.0), discretize(self.1), discretize(self.2), discretize(self.3))
    }

    pub fn to_normalized_srgb(self) -> (f32, f32, f32, f32)
    {
        (rgb2srgb(self.0), rgb2srgb(self.1), rgb2srgb(self.2), self.3)
    }

    pub fn to_discrete_srgb(self) -> (u8, u8, u8, u8)
    {
        (discretize(rgb2srgb(self.0)), discretize(rgb2srgb(self.1)), discretize(rgb2srgb(self.2)), discretize(self.3))
    }
}

pub fn rgb2srgb(rgb: f32) -> f32
{
    if rgb <= 0.0031308 { 12.92 * rgb } else { 1.055 * rgb.powf(1.0 / 2.4) - 0.055 }
}

pub fn srgb2rgb(srgb: f32) -> f32
{
    if srgb <= 0.04045 { srgb / 12.92 } else { ((srgb + 0.055) / 1.055).powf(2.4) }
}

pub fn normalize(v: u8) -> f32
{
    v as f32 / 255.0
}

pub fn discretize(v: f32) -> u8
{
    (v * 255.0).round() as u8
}

// Converts a HSV to a sRGB color. h is interpreted periodically with a period of 2*PI, 0 being red, and s and v should be normalized in [0, 1].
pub fn hsv2srgb(mut h: f32, s: f32, v: f32) -> (f32, f32, f32)
{
	while h < 0.0 { h += 2.0 * PI };
    h %= 2.0 * PI;
    let hi = ((h * 3.0 / PI).floor() as u8).max(0).min(6);
    let f = h * 3.0 / PI - hi as f32;
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
    match hi
    {
        0 | 6 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => unreachable!()
    }
}
