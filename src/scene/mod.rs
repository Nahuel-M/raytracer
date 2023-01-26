pub mod camera;
use std::fmt::Display;

use crate::{hittable::Hittable, Vec3, hit::Hit};

use image::{ImageBuffer, Pixel, Rgba};

use camera::Camera;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{ray::Ray, shape::{sphere::Sphere, polygon::Polygon}};

const MAX_DEPTH: u8 = 4;

// #[derive(Debug)]

pub struct Scene<'a> {
    pub camera: Camera,
    pub objects: Vec<Hittable<'a>>,
    pub image: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>,
}

impl Display for Scene<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for object in &self.objects{
            writeln!(f, "{}", object).unwrap();
        }
        writeln!(f)
    }
}

struct Vector3Wrapper(Vec3);
impl From<Vector3Wrapper> for Rgba<u8>{
    fn from(val: Vector3Wrapper) -> Self {
        Rgba([(val.0.x*255.).min(255.) as u8, (val.0.y*255.).min(255.) as u8, (val.0.z*255.).min(255.) as u8, 255_u8])
    }
}
#[allow(dead_code)]
impl<'a> Scene<'a> {
    pub fn new(
        camera: Camera,
        image: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>
    ) -> Self {
        let objects = vec![];
        Scene{ camera, objects, image }
    }
    pub fn add_sphere<>(&mut self, sphere : Sphere){
        self.objects.push(Hittable::with_sphere(sphere));
    }
    pub fn add_polygon<>(&mut self, polygon : Polygon){
        self.objects.push(Hittable::with_polygon(polygon));
    }

    pub fn add_hittable<>(&mut self, hittable : Hittable<'a>){
        self.objects.push(hittable);
    }

    pub fn render(&mut self, super_samples: usize){
        let now = std::time::Instant::now();
        print!("Rendering... ");
        let half_width: f64 = self.image.width() as f64 / 2.;
        let half_height: f64 = self.image.height() as f64 / 2.;

        let pixels : Vec<_> = self.image.enumerate_pixels_mut().collect();
        
        let mut super_pixels : Vec<(f64, f64)> = Vec::with_capacity(super_samples*super_samples);
        for super_pixel_x in 0..super_samples{
            for super_pixel_y in 0..super_samples{
                let super_pixel = (
                    (super_pixel_x as f64 - (super_samples-1) as f64 / 2.) / super_samples as f64 * self.camera.pixel_size,
                    (super_pixel_y as f64 - (super_samples-1) as f64 / 2.) / super_samples as f64 * self.camera.pixel_size,
                );
                super_pixels.push(super_pixel);
            }
        }

        pixels.into_par_iter().for_each(|(x, y, pixel)| {
            let mut average_pixel = Vec3::new(0.,0.,0.);

            for super_pixel in &super_pixels{
                let ray = self.camera.ray_for_pixel(x as f64 + super_pixel.0 - half_width, y as f64 + super_pixel.1 - half_height);
                average_pixel += Scene::color_for_ray(&ray, &self.objects, 0);
            }
            average_pixel /= (super_samples*super_samples) as f64;
            
            *pixel = Vector3Wrapper(average_pixel).into();
        });
        println!(" Done in {} seconds", now.elapsed().as_secs());
    }
    fn color_for_ray(ray : &Ray, hittables: &Vec<Hittable>, depth : u8) -> Vec3{

        if depth > MAX_DEPTH{
            return Vec3::new(0.,0.,0.,)
        }
        let mut lowest_distance_hit = Hit::max();
        let mut hit_object : Option<&Hittable> = None;
        for hittable in hittables { 
            let potential_hit = hittable.get_potential_hit(ray);
            
            if let Some(hit) = potential_hit {
                if hit.distance < 0.0001 { continue; }
                if hit.distance < lowest_distance_hit.distance {
                    lowest_distance_hit = hit;
                    hit_object = Some(hittable);
                }
            }
        }
        if let Some(hittable) = hit_object {
            let position = lowest_distance_hit.position;
            let surface_normal = lowest_distance_hit.normal;

            let mut final_color = Vec3::new(0., 0., 0.);
            
            final_color += hittable.material.luminance;
            if hittable.material.specular > 0. {
                final_color += hittable.material.specular * {
                    let specular_ray = ray.reflect_specular(surface_normal, position);
                    Scene::color_for_ray(&specular_ray, hittables, depth + 1)
                };
            }
            if hittable.material.specular + hittable.material.refraction < 0.999 && hittable.material.color.sum() > 0. {
                let orthogonal_to_normal = lowest_distance_hit.parallel_to_surface;
                final_color += (1. - hittable.material.specular - hittable.material.refraction) * hittable.material.color * {
                    let diffuse_ray = ray.reflect_diffuse(surface_normal, orthogonal_to_normal, position);
                    Scene::color_for_ray(&diffuse_ray, hittables, depth + 1)
                };
            }
            if hittable.material.refraction > 0. {
                final_color += hittable.material.refraction * {
                    let refract_ray = ray.refract(surface_normal, position, hittable.material.ior);
                    Scene::color_for_ray(&refract_ray, hittables, depth + 1)
                };
            }
            final_color
        } else {
            Vec3::new(0.,0.,0.)
        }
    }
}
