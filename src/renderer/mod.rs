pub mod shader;
pub mod tracer;
mod ray_instancer;
pub mod image_chunk;
pub mod threading;

use image::{ImageBuffer, Pixel, Rgb};
use crate::{world::World, renderer::{ray_instancer::RayInstancer}};
use self::{
    shader::Shader,
    tracer::Tracer, image_chunk::ImageChunk
};

const CHUNK_WIDTH : usize = 8;
type ImageChunk8 = ImageChunk<CHUNK_WIDTH>;

#[derive(Default)]
pub struct Renderer<'a> {
    tracer: Tracer<'a>,
    shader: Shader<'a>,
}

impl<'a> Renderer<'a> {
    pub fn render(
        &mut self,
        world: &'a World,
        image: &mut ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>,
        super_samples_sqrt: usize,
        max_bounces: u8,
    ) {
        let start_time = std::time::Instant::now();
        self.tracer.pre_compute(world);
        self.shader.pre_compute(world);
        let ray_instancer = RayInstancer::new(super_samples_sqrt, world.camera);

        println!(
            "Pre-compute done in in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );

        println!("Started rendering.");

        let num_x_chunks = image.width() as usize / CHUNK_WIDTH;
        let num_y_chunks = image.height() as usize / CHUNK_WIDTH;
        
        for x_chunk in 0..num_x_chunks{
            for y_chunk in 0..num_y_chunks{
                let chunk = ImageChunk8{ top_left: (x_chunk*CHUNK_WIDTH, y_chunk*CHUNK_WIDTH).into() };
                let chunk = threading::trace_chunk(chunk, &ray_instancer , &self.tracer, &self.shader);
                for pixel in chunk.iter(){
                    *image.get_pixel_mut(pixel.0.x as u32, pixel.0.y as u32) = pixel.1.clamp_to_rgb();
                }
            }
        }

        println!(
            "Rendering done in in {:.2} seconds",
            start_time.elapsed().as_secs_f32()
        );

        self.tracer.clear();
        self.shader.clear();

    }
}
