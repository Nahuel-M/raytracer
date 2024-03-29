mod ray_instancer;
pub mod shader;
pub mod compute;
pub mod tracer;

use crossbeam_channel::bounded;
use image::{ImageBuffer, Pixel, Rgb};

use std::thread::{self, available_parallelism};

use self::{shader::Shader, tracer::Tracer};
use crate::{renderer::ray_instancer::RayInstancer, world::World, image::get_chunks_iter};

const CHUNK_SIZE: usize = 16;

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
        println!("Started rendering. with {}  cores.", number_of_cores);

        let (result_sender, result_receiver) = bounded(100);
        thread::scope(|s| {
            let (task_sender, task_receiver) = bounded(100);

            let ray_instancer = &self.ray_instancer;
            let tracer = &self.tracer;
            let shader = &self.shader;
            
            // Spawn processing threads
            for _ in 0..number_of_cores - 1 {
                s.spawn({
                    let task_receiver = task_receiver.clone();
                    let result_sender = result_sender.clone();
                    move || {
                        while let Ok(chunk) = task_receiver.recv() {
                            let result = compute::trace_chunk(
                                chunk,
                                ray_instancer,
                                tracer,
                                shader,
                                max_bounces,
                            );
                            result_sender.to_owned().send(result).unwrap();
                        }
                    }
                });
            }

            drop(result_sender);

            let chunks = get_chunks_iter(image, CHUNK_SIZE);

            // Spawn result processor thread
            s.spawn(|| {
                while let Ok(result) = result_receiver.recv() {
                    for (coordinate, color) in result.iter() {
                        *image.get_pixel_mut(coordinate.x as u32, coordinate.y as u32) =
                            color.to_srgb().clamp_to_rgb();
                    }
                }
            });

            // Generate chunks
            for chunk in chunks{
                task_sender.send(chunk).unwrap();
            }
        });

        println!(
            "Rendering done in in {:.2} seconds",
            start_time.elapsed().as_secs_f32()
        );

        self.clean_up();
    }

    fn pre_compute(&mut self, world: &'a World, super_samples_sqrt: usize) {
        let start_time = std::time::Instant::now();

        self.tracer.pre_compute(world);
        self.shader.pre_compute(world);
        self.ray_instancer
            .pre_compute(super_samples_sqrt, world.camera);

        println!(
            "Pre-compute done in in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
    }

    fn clean_up(&mut self) {
        self.tracer.clear();
        self.shader.clear();
    }
}
