use std::io::{Read, Seek, SeekFrom, Result, Error, ErrorKind};
use std::f32::consts::PI;

pub struct SliceReadSeek<'a>(&'a [u8], i64);

impl<'a> SliceReadSeek<'a>
{
    pub fn new(slice: &'a [u8]) -> Self
    {
        Self(slice, 0)
    }
}

impl Read for SliceReadSeek<'_>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        let start = self.1 as usize;
        let len = buf.len().min(self.0.len() - start);
        if len <= 0 { Ok(0) }
        else
        {
            buf[..len].copy_from_slice(&self.0[start .. (start + len)]);
            self.1 += len as i64;
            Ok(len)
        }
    }
}

impl Seek for SliceReadSeek<'_>
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>
    {
        let index = match pos
        {
            SeekFrom::Start(index) => index as i64,
            SeekFrom::End(index) => index + self.0.len() as i64,
            SeekFrom::Current(index) => index + self.1
        };
        if index < 0 { return Result::Err(Error::new(ErrorKind::Other, "out of bounds")); }
        self.1 = index;
        Ok(index as u64)
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

// Converts a HSV to a sRGB color. h is interpreted periodically with a period of 2*PI, 0 being red, s and v should be normalized in [0, 1].
pub fn hsv2srgb(h: f32, s: f32, v: f32) -> (f32, f32, f32)
{
    let mut h = h % (2.0 * PI); if h < 0.0 { h += 2.0 * PI };
    let hi = 0.max(6.min((h * 3.0 / PI).floor() as u8));
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
