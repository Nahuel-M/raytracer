
mod material;
mod world;
mod algebra;
mod hit;
mod renderer;

use std::f64::consts::PI;
use image::io::Reader;
use show_image::{create_window, AsImageView, WindowOptions};

use image::RgbImage;

use crate::algebra::vec3::Vec3;
use crate::material::map::RgbMap;
use crate::renderer::Renderer;
use crate::world::World;
use crate::world::camera::Camera;
// const WIDTH: u32 = 1920;
// const HEIGHT: u32 = 1080;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
#[show_image::main]
fn main() {
    let mut camera = Camera::new(PI / 2., WIDTH);
    camera.position = Vec3::new(120., 70., 100.);
    camera.look_at(Vec3::new(0., 32., 20.));
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let mut world = World::with_camera(camera);
    let mut teapot = world.insert_model_by_filename("D:\\Git\\Rust\\raytracer\\models\\teapot.obj");
    let _floor = world.insert_model_by_filename("D:\\Git\\Rust\\raytracer\\models\\floor.stl");
    teapot.material.write().unwrap().specular = 0.;
    teapot.material.write().unwrap().refraction = 0.;
    teapot.material.write().unwrap().ior = 2.5;
    teapot.material.write().unwrap().diffuse = Vec3::new(1., 0.8, 0.6);
    teapot += Vec3::new(0., 35., 0.,);
    let clouds = Reader::open("D:\\Git\\Rust\\raytracer\\images\\above_clouds.jpg").unwrap().decode().unwrap().into_rgb32f();
    world.background = RgbMap::Texture(Box::new(clouds));
    Renderer::render(&mut world, &mut image, 15, 3);
    
    let window = create_window(
        "image",
        WindowOptions::new().set_size(Some([WIDTH, HEIGHT])).set_default_controls(true),
    )
    .unwrap();
    window
        .set_image("render", image.as_image_view().unwrap())
        .unwrap();
    window.wait_until_destroyed().unwrap();
    println!("{}", image.len());
}
