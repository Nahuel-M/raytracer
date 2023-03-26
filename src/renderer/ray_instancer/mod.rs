use crate::{world::camera::Camera, algebra::ray::Ray, image::pixel_coordinate::PixelCoordinate};
#[derive(Default)]
pub struct RayInstancer{
    super_samples_width : usize,
    super_samples : Vec<(f64, f64)>,
    camera : Camera,
}

impl RayInstancer{
    pub fn rays_for_pixel(&self, pixel: PixelCoordinate) -> Vec<Ray> {
        let mut rays = Vec::<Ray>::with_capacity(self.super_samples_width.pow(2));
        for super_sample in &self.super_samples{
            rays.push(
                self.camera.ray_for_pixel(super_sample.0 + pixel.x as f64, super_sample.1 + pixel.y as f64)
            );
        }
        rays
    }

    fn get_super_samples(super_samples_width: usize) -> Vec<(f64, f64)> {
        let mut super_pixels: Vec<(f64, f64)> =
            Vec::with_capacity(super_samples_width * super_samples_width);
        for super_pixel_x in 0..super_samples_width {
            for super_pixel_y in 0..super_samples_width {
                let super_pixel = (
                    (super_pixel_x as f64 - (super_samples_width - 1) as f64 / 2.)
                        / super_samples_width as f64,
                    (super_pixel_y as f64 - (super_samples_width - 1) as f64 / 2.)
                        / super_samples_width as f64
                );
                super_pixels.push(super_pixel);
            }
        }
        super_pixels
    }

    pub(crate) fn pre_compute(&mut self, super_samples_sqrt: usize, camera: Camera) {
        self.super_samples_width = super_samples_sqrt;
        self.super_samples = Self::get_super_samples(super_samples_sqrt);
        self.camera = camera;
    }
}