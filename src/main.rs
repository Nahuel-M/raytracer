
mod material;
mod ray;
mod world;
mod algebra;
mod hit;
mod renderer;

use std::f64::consts::PI;
use show_image::{create_window, AsImageView, WindowOptions};

use image::RgbaImage;
// use scene::camera::Camera;
// use scene::Scene;
// use shape::triangle::Triangle;
// use shape::sphere::Sphere;

use crate::algebra::vec3::Vec3;
use crate::renderer::Renderer;
use crate::world::World;
use crate::world::camera::Camera;
// const WIDTH: u32 = 1280;
// const HEIGHT: u32 = 720;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
#[show_image::main]
fn main() {
    let mut camera = Camera::new(PI / 4., WIDTH);
    camera.position = Vec3::new(-100., 80., -200.);
    camera.look_at(Vec3::new(0., 32., 20.));
    let mut image = RgbaImage::new(WIDTH, HEIGHT);

    let mut world = World::with_camera(camera);
    let mut teapot = world.insert_model_by_filename("models/teapot.obj");
    let _floor = world.insert_model_by_filename("models/floor.stl");
    world.background = Vec3::new(0.5,0.5,0.5);
    teapot.material.write().unwrap().specular = 0.;
    teapot.material.write().unwrap().color = Vec3::new(0.8, 0.6, 0.4);
    teapot += Vec3::new(0., 35., 0.,);
    println!("{world}");
    Renderer::render(&mut world, &mut image, 3, 5);

    let window = create_window(
        "image",
        WindowOptions::new().set_size(Some([WIDTH, HEIGHT])).set_default_controls(true),
    )
    .unwrap();
    // renderer::render_polygons(&camera, &polygons, &mut img);
    window
        .set_image("render", image.as_image_view().unwrap())
        .unwrap();
    window.wait_until_destroyed().unwrap();
}
