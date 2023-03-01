use crate::algebra::{vec3::Vec3, color::Color};

use super::ray_instancer::pixel_coordinate::PixelCoordinate;

pub struct ImageChunk<const SIZE: usize>{
    pub top_left : PixelCoordinate,
}

impl<const SIZE: usize> ImageChunk<SIZE>{
     pub fn pixels(&self,) -> Vec<PixelCoordinate>{
        let mut pixels = Vec::<PixelCoordinate>::with_capacity(SIZE*SIZE);

        for x in self.top_left.x .. self.top_left.x + SIZE{
            for y in self.top_left.y .. self.top_left.y + SIZE{
                pixels.push((x, y).into());
            }
        }
        pixels
    }
}

pub struct PixelColorArray<const SIZE: usize>{
    pub top_left : PixelCoordinate,
    data : [[Color; SIZE]; SIZE],
}

impl<const SIZE: usize> PixelColorArray<SIZE> {
    pub fn new(top_left: PixelCoordinate) -> Self{
        Self{ top_left, data: [[Vec3::ZEROS.into(); SIZE]; SIZE] }
    }
    pub fn set(&mut self, pixel: PixelCoordinate, color: Color){
        self.data[pixel.x - self.top_left.x][pixel.y - self.top_left.y] = color;
    }

    // pub fn get(&self, pixel: Pixel) -> Vec3{
    //     self.data[pixel.x - self.top_left.x][pixel.y - self.top_left.y]
    // }

    pub fn iter(&self) -> impl Iterator<Item = (PixelCoordinate, Color)> + '_{
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(col_index, col_data)| {
                col_data.iter()
                .enumerate()
                .map(move |(row_index, color)|{
                    (PixelCoordinate{x: col_index + self.top_left.x, y: row_index + self.top_left.y}, *color)
                })
            })
    }
}
