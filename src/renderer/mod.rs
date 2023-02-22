pub mod shader;
pub mod tracer;

use image::{ImageBuffer, Pixel, Rgb};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{algebra::ray::Ray, algebra::vec3::Vec3, world::World};

use self::{
    shader::{shade_package::ShadePackage, Shader},
    tracer::Tracer
};
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
        println!(
            "Pre-compute done in in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );

        println!("Started rendering.");
        let half_width: f64 = image.width() as f64 / 2.;
        let half_height: f64 = image.height() as f64 / 2.;

        let pixels: Vec<_> = image.enumerate_pixels_mut().collect();

        let super_pixels = Renderer::get_super_samples(super_samples_sqrt);
        let camera = world.camera;
        pixels.into_par_iter().for_each(|(x, y, pixel)| {
            let mut average_pixel = Vec3::new(0., 0., 0.);

            for super_pixel in &super_pixels {
                let ray = camera.ray_for_pixel(
                    x as f64 + super_pixel.0 - half_width,
                    y as f64 + super_pixel.1 - half_height,
                );

                average_pixel +=
                    Renderer::advance_ray(&ray, &self.tracer, &self.shader, max_bounces);
            }
            average_pixel /= (super_samples_sqrt * super_samples_sqrt) as f64;

            *pixel = average_pixel.clamp_to_rgb();
        });

        self.tracer.clear();
        self.shader.clear();

        println!(
            "Rendering done in in {:.2} seconds",
            start_time.elapsed().as_secs_f32()
        );
    }

    fn advance_ray(ray: &Ray, tracer: &Tracer, shader: &Shader, remaining_bounces: u8) -> Vec3 {
        if remaining_bounces == 0 {
            return Vec3::new(0., 0., 0.);
        }

        let trace_result = tracer.trace_ray(ray);
        let shade_result = shader.shade_hit(&trace_result, ray);

        shade_result
        .iter()
        .map(|shade_package| match shade_package{
            ShadePackage::Ray(trace_package) => {
                trace_package.multiplier *
                Renderer::advance_ray(
                    &trace_package.ray,
                    tracer,
                    shader,
                    remaining_bounces - 1)
            },
            ShadePackage::Color(color) => *color,
        })
        .sum()
       
    }

    fn get_super_samples(super_samples_sqrt: usize) -> Vec<(f64, f64)> {
        let mut super_pixels: Vec<(f64, f64)> =
            Vec::with_capacity(super_samples_sqrt * super_samples_sqrt);
        for super_pixel_x in 0..super_samples_sqrt {
            for super_pixel_y in 0..super_samples_sqrt {
                let super_pixel = (
                    (super_pixel_x as f64 - (super_samples_sqrt - 1) as f64 / 2.)
                        / super_samples_sqrt as f64,
                    (super_pixel_y as f64 - (super_samples_sqrt - 1) as f64 / 2.)
                        / super_samples_sqrt as f64
                );
                super_pixels.push(super_pixel);
            }
        }
        super_pixels
    }
}