use image::{ImageBuffer, Pixel, Rgba};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{algebra::vec3::Vec3, world::World, ray::Ray};

pub struct Renderer {}

impl Renderer {
    pub fn render(
        world: &World,
        image: &mut ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>,
        super_samples_sqrt: usize,
        ray_depth: usize,
    ) {
        
        let half_width: f64 = image.width() as f64 / 2.;
        let half_height: f64 = image.height() as f64 / 2.;

        let pixels: Vec<_> = image.enumerate_pixels_mut().collect();

        let mut super_pixels: Vec<(f64, f64)> = Vec::with_capacity(super_samples_sqrt * super_samples_sqrt);
        for super_pixel_x in 0..super_samples_sqrt {
            for super_pixel_y in 0..super_samples_sqrt {
                let super_pixel = (
                    (super_pixel_x as f64 - (super_samples_sqrt - 1) as f64 / 2.) / super_samples_sqrt as f64
                        * world.camera.pixel_size,
                    (super_pixel_y as f64 - (super_samples_sqrt - 1) as f64 / 2.) / super_samples_sqrt as f64
                        * world.camera.pixel_size,
                );
                super_pixels.push(super_pixel);
            }
        }

        pixels.into_par_iter().for_each(|(x, y, pixel)| {
            let mut average_pixel = Vec3::new(0., 0., 0.);

            for super_pixel in &super_pixels {
                let ray = world.camera.ray_for_pixel(
                    x as f64 + super_pixel.0 - half_width,
                    y as f64 + super_pixel.1 - half_height,
                );

                average_pixel += Renderer::advance_ray(&ray, &world, ray_depth);
            }
            average_pixel /= (super_samples_sqrt * super_samples_sqrt) as f64;

            *pixel = average_pixel.clamp_to_rgba();
        });
    }

    fn advance_ray(ray: &Ray, world: &World, remaining_depth : usize) -> Vec3{
        if remaining_depth == 0{
            return Vec3::new(0.,0.,0.,)
        }
        let potential_hit = world.get_ray_collision(ray);
        if let Some(hit) = potential_hit {

            let mut final_color = Vec3::new(0., 0., 0.);
            
            final_color += hit.material.luminance;
            if hit.material.specular > 0.001 {
                final_color += hit.material.specular * {
                    let specular_ray = ray.reflect_specular(hit.normal, hit.position);
                    Renderer::advance_ray(&specular_ray, world, remaining_depth - 1)
                };
            }
            if hit.material.specular + hit.material.refraction < 0.999 && hit.material.color.sum() > 0.001 {
                let orthogonal_to_normal = hit.parallel_to_surface;
                final_color += (1. - hit.material.specular - hit.material.refraction) * hit.material.color * {
                    let diffuse_ray = ray.reflect_diffuse(hit.normal, orthogonal_to_normal, hit.position);
                    Renderer::advance_ray(&diffuse_ray, world, remaining_depth - 1)
                };
            }
            if hit.material.refraction > 0.001 {
                final_color += hit.material.refraction * {
                    let refract_ray = ray.refract(hit.normal, hit.position, hit.material.ior);
                    Renderer::advance_ray(&refract_ray, world, remaining_depth - 1)
                };
            }
            final_color
        } else {
            world.background
        }
    }
}

