#[derive(Clone, Copy)]
pub enum Channels
{
    L,
    RGBA,
    BGRA
}

impl Channels
{
    fn channels(self) -> u8
    {
        match self
        {
            Self::L => 1,
            Self::RGBA | Self::BGRA => 4
        }
    }

    fn check(self, components: u8)
    {
        match self
        {
            Self::L => assert!(components == 1),
            Self::RGBA | Self::BGRA => assert!(components == 3 || components == 4)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Config
{
    channels: Channels,
    default_alpha: u8
}

impl Config
{
    pub fn new() -> Self
    {
        Self
        {
            channels: Channels::RGBA,
            default_alpha: 255
        }
    }

    pub fn channels(mut self, channels: Channels) -> Self
    {
        self.channels = channels;
        self
    }

    pub fn default_alpha(mut self, default_alpha: u8) -> Self
    {
        self.default_alpha = default_alpha;
        self
    }
}

pub struct JPG
{
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub data: Vec<u8>
}

impl JPG
{
    pub fn decode(raw: &[u8], config: Config) -> Self
    {
        let mut decoder = zune_jpeg::JpegDecoder::new(raw);
        let mut data = decoder.decode().unwrap();
        let info = decoder.info().unwrap();
        let num_pixels = info.width as usize * info.height as usize;
        assert_eq!(num_pixels * info.components as usize, data.len());
        config.channels.check(info.components);
        if info.components == 3
        {
            let mut new_data = vec![config.default_alpha; num_pixels * 4];
            for ([r, g, b], [r_new, g_new, b_new, _]) in data.array_chunks().zip(new_data.array_chunks_mut())
            {
                *r_new = *r;
                *g_new = *g;
                *b_new = *b;
            }
            data = new_data;
        }
        if matches!(config.channels, Channels::BGRA)
        {
            for [r, _, b, _] in data.array_chunks_mut()
            {
                std::mem::swap(r, b);
            }
        }

        Self
        {
            width: info.width as u32,
            height: info.height as u32,
            channels: config.channels.channels(),
            data
        }
    }

    pub fn extract_channel(&mut self, channel: u8)
    {
        if self.channels != 4 { panic!("no 4 channels"); }
        let mut data = Vec::with_capacity((self.width * self.height) as usize);
        for pixels in self.data.array_chunks::<4>() { data.push(pixels[channel as usize]); }
        self.channels = 1;
        self.data = data;
    }
}
