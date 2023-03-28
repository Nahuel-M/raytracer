use std::iter::once;

use image::RgbImage;

use self::{image_chunk::ImageChunkCoordinates, pixel_coordinate::PixelCoordinate};

pub mod image_chunk;
pub mod pixel_coordinate;

pub fn get_chunks_iter(image : &RgbImage, chunk_size: usize) -> impl Iterator<Item = ImageChunkCoordinates>{
    let num_x_chunks = image.width() as usize / chunk_size;
    let num_y_chunks = image.height() as usize / chunk_size;
    let width = image.width() as usize;
    let height = image.height() as usize;

    let size = PixelCoordinate::from((chunk_size, chunk_size));

    let full_chunks = (0..num_y_chunks)
        .flat_map(move |y_chunk| (0..num_x_chunks).map(move |x_chunk| PixelCoordinate::from((x_chunk * chunk_size,y_chunk * chunk_size))))
        .map(move |top_left| ImageChunkCoordinates{ top_left, size });


    let left_column = (0..num_y_chunks)
        .map(move |y_chunk| PixelCoordinate::from((chunk_size*num_x_chunks, chunk_size * y_chunk)))
        .map(move |top_left| ImageChunkCoordinates{ top_left, size: PixelCoordinate::from((width - num_x_chunks * chunk_size, chunk_size)) });

    let bottom_row = (0..num_x_chunks)
        .map(move |x_chunk| PixelCoordinate::from((chunk_size*x_chunk, chunk_size * num_y_chunks)))
        .map(move |top_left| ImageChunkCoordinates{ top_left, size: PixelCoordinate::from((chunk_size, height - num_y_chunks * chunk_size)) });

    let bottom_left = ImageChunkCoordinates {
        top_left: (num_x_chunks * chunk_size, num_y_chunks * chunk_size).into(),
        size: (width - num_x_chunks * chunk_size, height - num_y_chunks * chunk_size).into(),
    };

   full_chunks.chain(left_column).chain(bottom_row).chain(once(bottom_left))

}