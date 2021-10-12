mod math;
mod widget;
mod draw;
mod event;

pub use math::*;
pub use widget::*;
pub use draw::*;
pub use event::*;

pub use crate::text::Font;
use crate::text::AtlasBuilder;

const TEXTURE_RESOLUTION: f32 = 100.0;
const TEXTURE_SIZE: u32 = 1024;
const TEXTURE_PADDING: u32 = 5;

#[derive(PartialEq)]
pub struct UiConfig
{
    pub size: Vec2,
    pub scale: f32
}

pub struct Ui<'a, T: 'a>
{
    config: UiConfig,
    text: Option<AtlasBuilder<'a>>,
    painter: draw::Painter,
    widgets: Vec<Box<dyn Widget<T> + 'a>>
}

impl<'a, T> Ui<'a, T>
{
    pub fn new(font: Font<'a>) -> Self
    {
        let text = AtlasBuilder::new(font, TEXTURE_RESOLUTION, TEXTURE_SIZE, TEXTURE_PADDING);
        Self
        {
            config: UiConfig { size: Vec2(0.0, 0.0), scale: 1.0 },
            text: Some(text),
            painter: draw::Painter::new(),
            widgets: Vec::new()
        }
    }

    pub fn add<W: Widget<T> + 'a>(&mut self, widget: W)
    {
        self.widgets.push(Box::new(widget));
    }

    pub fn begin_frame<'b>(&mut self, data: &mut T, events: impl Iterator<Item = &'b event::Event>, config: UiConfig)
    {
        if self.config != config
        {
            self.config = config;
            self.text = Some(AtlasBuilder::new(self.text.take().unwrap().into_font(), TEXTURE_RESOLUTION * self.config.scale, TEXTURE_SIZE, TEXTURE_PADDING));
        }
        self.painter.begin_frame(self.config.scale);
        for widget in &self.widgets
        {
            let size = widget.layout(data, self.config.size / self.config.scale);
            widget.draw(data, &mut self.painter);
            self.painter.offset(size);
        }
    }

    pub fn end_frame(&mut self) -> (&[draw::Vertex], &[u16])
    {
        self.painter.end_frame()
    }
}
