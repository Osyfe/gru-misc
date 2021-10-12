use super::math::*;

pub struct Vertex
{
    pub position: Vec2
}

pub struct Painter
{
    origin: Vec2,
    scale: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl Painter
{
    pub fn new() -> Self
    {
        Self
        {
            origin: Vec2(0.0, 0.0),
            scale: 1.0,
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn offset(&mut self, offset: Vec2)
    {
        self.origin += offset;
    }

    pub fn draw_rect(&mut self, rect: Rect)
    {
        let min = self.origin + rect.min * self.scale;
        let max = self.origin + rect.max * self.scale;
        let i0 = self.vertices.len() as u16;
        self.vertices.push(Vertex { position: min });
        self.vertices.push(Vertex { position: Vec2(min.0, max.1) });
        self.vertices.push(Vertex { position: max });
        self.vertices.push(Vertex { position: Vec2(max.0, min.1) });
        self.indices.push(i0 + 0);
        self.indices.push(i0 + 1);
        self.indices.push(i0 + 2);
        self.indices.push(i0 + 2);
        self.indices.push(i0 + 3);
        self.indices.push(i0 + 0);
    }

    pub fn begin_frame(&mut self, scale: f32)
    {
        self.origin = Vec2(0.0, 0.0);
        self.scale = scale;
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn end_frame(&mut self) -> (&[Vertex], &[u16])
    {
        (&self.vertices, &self.indices)
    }
}
