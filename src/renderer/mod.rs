use std::f64::consts::PI;

use image::{ImageBuffer, Pixel, Rgb};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{algebra::vec3::Vec3, world::World, algebra::ray::Ray, material::map::GetValueAt};

pub struct Renderer {}

impl Renderer {
    pub fn render(
        world: &mut World,
        image: &mut ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>,
        super_samples_sqrt: usize,
        ray_depth: usize,
    ) {
        let start_time = std::time::Instant::now();
        world.pre_compute();
        println!("Pre-compute done in in {:.3} seconds", start_time.elapsed().as_secs_f32());

        println!("Started rendering.");
        let half_width: f64 = image.width() as f64 / 2.;
        let half_height: f64 = image.height() as f64 / 2.;

        let pixels: Vec<_> = image.enumerate_pixels_mut().collect();

        let super_pixels = Renderer::get_super_samples(super_samples_sqrt, world.camera.pixel_size);

        pixels.into_par_iter().for_each(|(x, y, pixel)| {
            let mut average_pixel = Vec3::new(0., 0., 0.);

            for super_pixel in &super_pixels {
                let ray = world.camera.ray_for_pixel(
                    x as f64 + super_pixel.0 - half_width,
                    y as f64 + super_pixel.1 - half_height,
                );

                average_pixel += Renderer::advance_ray(&ray, world, ray_depth);
            }
            average_pixel /= (super_samples_sqrt * super_samples_sqrt) as f64;

            *pixel = average_pixel.clamp_to_rgb();
        });
        
        println!("Rendering done in in {:.2} seconds", start_time.elapsed().as_secs_f32());
    }

    fn advance_ray(ray: &Ray, world: &World, remaining_depth : usize) -> Vec3{
        if remaining_depth == 0{
            return Vec3::new(0.,0.,0.,)
        }
        let potential_hit = world.get_ray_collision(ray);
        if let Some(hit) = potential_hit {
            let material = hit.material.unwrap();
            let material = material.read().unwrap();
            let mut final_color = Vec3::new(0., 0., 0.);

            let base_reflectance = (material.ior-1.).powi(2) / (material.ior+1.).powi(2);
            let fresnel_reflection = Renderer::schlick_fresnell_approximation(base_reflectance, &hit.normal, ray.direction_unit);

            let specular_factor = (1. - material.refraction) * material.specular + material.refraction * fresnel_reflection;
            let refraction_factor = material.refraction * (1. - fresnel_reflection);
            let diffuse_factor = (1. - material.specular) * (1. - material.refraction);

            let hitting_face_from_front = ray.direction_unit.dot(&hit.normal) < 0.;
            if hitting_face_from_front{
                // Add luminance
                final_color += material.luminance;

                // Add specular
                if specular_factor > 0.0001{
                    final_color += specular_factor* {
                        let specular_ray = ray.reflect_specular(hit.normal, hit.position);
                        Renderer::advance_ray(&specular_ray, world, remaining_depth - 1)
                    };
                }

                // Add diffuse
                if diffuse_factor > 0.0001 
                    && material.diffuse.sum() > 0.001{
                    let diffuse_color = diffuse_factor * material.diffuse * {
                        let diffuse_ray = ray.reflect_diffuse(hit.normal, hit.position);
                        Renderer::advance_ray(&diffuse_ray, world, remaining_depth - 1)
                    };
                    final_color += diffuse_color;
                }

            } else{
                // Continue ray as if nothing happened
                let mut new_ray = *ray;
                new_ray.origin = hit.position;
                final_color += Renderer::advance_ray(&new_ray, world, remaining_depth);
            }

            // Add refraction
            if refraction_factor > 0.01 {
                final_color += refraction_factor * {
                    let refract_ray = ray.refract(hit.normal, hit.position, material.ior);
                    Renderer::advance_ray(&refract_ray, world, remaining_depth - 1)
                };
            }
            final_color
        } else {
            match &world.background{
                crate::material::map::RgbMap::Color(color) => *color,
                crate::material::map::RgbMap::Texture(texture) => {
                    let u = 0.5 + f64::atan2(ray.direction_unit.z, ray.direction_unit.x)/(2.*PI);
                    let v = 0.5 - f64::asin(ray.direction_unit.y.min(1.).max(-1.)) / PI;
                    texture.get_value_at(u, v)       
                },
            }
        }
    }

    #[inline]
    /// See https://en.wikipedia.org/wiki/Schlick%27s_approximation
    fn schlick_fresnell_approximation(base_reflectance : f64, surface_normal: &Vec3, ray_direction: Vec3) -> f64{
        base_reflectance + (1. - base_reflectance)*(1. - ray_direction.dot(&-*surface_normal)).powi(5)
    }

    fn get_super_samples(super_samples_sqrt: usize, pixel_size: f64)-> Vec<(f64, f64)>{
        let mut super_pixels: Vec<(f64, f64)> = Vec::with_capacity(super_samples_sqrt * super_samples_sqrt);
        for super_pixel_x in 0..super_samples_sqrt {
            for super_pixel_y in 0..super_samples_sqrt {
                let super_pixel = (
                    (super_pixel_x as f64 - (super_samples_sqrt - 1) as f64 / 2.) / super_samples_sqrt as f64
                        * pixel_size,
                    (super_pixel_y as f64 - (super_samples_sqrt - 1) as f64 / 2.) / super_samples_sqrt as f64
                        * pixel_size,
                );
                super_pixels.push(super_pixel);
            }
        }
        super_pixels
    }
}

