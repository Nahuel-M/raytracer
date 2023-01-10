pub mod camera;
pub mod light;

use image::{ImageBuffer, Pixel, Rgba};
use nalgebra::Vector3;

use camera::Camera;
use light::Light;

use crate::ray::Ray;

struct Scene {
    camera: Camera,
    objects: Vec<Vec<Vector3<f64>>>,
    lights: Vec<Light>,
    image: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>,
}

impl Scene {
    fn new(
        camera: Camera,
        objects: Vec<Vec<Vector3<f64>>>,
        lights: Vec<Light>,
        image: ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>
    ) -> Self {
        Scene{ camera, objects, lights, image }
    }
    fn render(&self){
        
    }
}
