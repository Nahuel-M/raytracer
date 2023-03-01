use crate::algebra::{vec3::Vec3, color::Color};

use super::ray_instancer::pixel::Pixel;

pub struct ImageChunk<const SIZE: usize>{
    pub top_left : Pixel,
}

impl<const SIZE: usize> ImageChunk<SIZE>{
     pub fn pixels(&self,) -> Vec<Pixel>{
        let mut pixels = Vec::<Pixel>::with_capacity(SIZE*SIZE);

        for x in self.top_left.x .. self.top_left.x + SIZE{
            for y in self.top_left.y .. self.top_left.y + SIZE{
                pixels.push((x, y).into());
            }
        }
        pixels
    }
}

pub struct PixelColorArray<const SIZE: usize>{
    pub top_left : Pixel,
    data : [[Color; SIZE]; SIZE],
}

impl<const SIZE: usize> PixelColorArray<SIZE> {
    pub fn new(top_left: Pixel) -> Self{
        Self{ top_left, data: [[Vec3::ZEROS.into(); SIZE]; SIZE] }
    }
    pub fn set(&mut self, pixel: Pixel, color: Color){
        self.data[pixel.x - self.top_left.x][pixel.y - self.top_left.y] = color;
    }

    // pub fn get(&self, pixel: Pixel) -> Vec3{
    //     self.data[pixel.x - self.top_left.x][pixel.y - self.top_left.y]
    // }

    pub fn iter(&self) -> impl Iterator<Item = (Pixel, Color)> + '_{
        self.data
            .iter()
            .enumerate()
            .flat_map(move |(col_index, col_data)| {
                col_data.iter()
                .enumerate()
                .map(move |(row_index, color)|{
                    (Pixel{x: col_index + self.top_left.x, y: row_index + self.top_left.y}, *color)
                })
            })
    }
}
