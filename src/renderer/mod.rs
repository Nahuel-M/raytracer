pub mod shader;
pub mod tracer;
pub mod image_chunk;
pub mod threading;
mod ray_instancer;

use crossbeam_channel::{bounded};
use image::{ImageBuffer, Pixel, Rgb};

use std::thread;
use std::thread::available_parallelism;

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
    ray_instancer: RayInstancer,
}

impl<'a> Renderer<'a> {
    pub fn render(
        &mut self,
        world: &'a World,
        image: &mut ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>,
        super_samples_sqrt: usize,
        max_bounces: u8,
    ) {
        self.pre_compute(world, super_samples_sqrt);

        let start_time = std::time::Instant::now();

        let number_of_cores = available_parallelism().unwrap().get();
        println!("Started rendering. with {} virtual cores", number_of_cores    );

        let (result_sender, result_receiver) = bounded(100);
        thread::scope(|s| {
            let (task_sender, task_receiver) = bounded(100);

            for _ in 0..number_of_cores - 1{
                let task_receiver = task_receiver.clone();
                let result_sender = result_sender.clone();
                let ray_instancer = &self.ray_instancer;
                let tracer = &self.tracer;
                let shader = &self.shader;
                s.spawn(move || {
                    while let Ok(chunk) = task_receiver.recv(){
                        let result = threading::trace_chunk(chunk, ray_instancer, tracer, shader, max_bounces);
                        result_sender.send(result).unwrap();
                    }
                });
            }
            
            drop(result_sender);

            let num_x_chunks = image.width() as usize / CHUNK_WIDTH;
            let num_y_chunks = image.height() as usize / CHUNK_WIDTH;

            s.spawn(|| {
                while let Ok(result) = result_receiver.recv(){
                    for (coordinate, color) in result.iter(){
                        *image.get_pixel_mut(coordinate.x as u32, coordinate.y as u32) = color.to_srgb().clamp_to_rgb();
                    }
                }
            });

            for x_chunk in 0..num_x_chunks{
                for y_chunk in 0..num_y_chunks{
                    let chunk = ImageChunk8{ top_left: (x_chunk*CHUNK_WIDTH, y_chunk*CHUNK_WIDTH).into() };
                    task_sender.send(chunk).unwrap();
                }
            }
        });

        while let Ok(result) = result_receiver.recv(){
            for (coordinate, color) in result.iter(){
                *image.get_pixel_mut(coordinate.x as u32, coordinate.y as u32) = color.to_srgb().clamp_to_rgb();
            }
        }

        println!(
            "Rendering done in in {:.2} seconds",
            start_time.elapsed().as_secs_f32()
        );

        self.clean_up();
    }

    fn pre_compute(&mut self, world: &'a World, super_samples_sqrt: usize){
        let start_time = std::time::Instant::now();

        self.tracer.pre_compute(world);
        self.shader.pre_compute(world);
        self.ray_instancer.pre_compute(super_samples_sqrt, world.camera);

        println!(
            "Pre-compute done in in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
    }



    fn clean_up(&mut self){
        self.tracer.clear();
        self.shader.clear();
    }
}
