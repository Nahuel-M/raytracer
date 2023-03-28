use crate::algebra::color::Color;

use super::pixel_coordinate::PixelCoordinate;

pub struct ImageChunkCoordinates{
    pub top_left : PixelCoordinate,
    pub size : PixelCoordinate,
}

impl ImageChunkCoordinates{
     pub fn pixels(&self,) -> Vec<PixelCoordinate>{
        let mut pixels = Vec::<PixelCoordinate>::with_capacity(self.size.x*self.size.y);

        for x in self.top_left.x .. self.top_left.x + self.size.x{
            for y in self.top_left.y .. self.top_left.y + self.size.y{
                pixels.push((x, y).into());
            }
        }
        pixels
    }

    pub fn instantiate_chunk(&self) -> ImageChunk{
        ImageChunk::new(self.top_left, self.size)
    }
}

pub struct ImageChunk{
    pub top_left : PixelCoordinate,
    pub size : PixelCoordinate,
    data : Vec<Color>,
}

impl ImageChunk{
    pub fn new(top_left: PixelCoordinate, size: PixelCoordinate) -> Self{
        Self{ top_left, size, data: vec![Color::BLACK;size.x*size.y] }
    }
    pub fn set(&mut self, pixel: PixelCoordinate, color: Color){
        self.data[(pixel.x - self.top_left.x) + (pixel.y - self.top_left.y) * self.size.x] = color;
    }

    pub fn iter(&self) -> impl Iterator<Item = (PixelCoordinate, Color)> + '_{
        self.data
            .iter()
            .enumerate()
            .map(|(index, &color)| {
                ((self.top_left.x + index % self.size.x, self.top_left.y + index / self.size.x).into(), color)
            })
    }
}
