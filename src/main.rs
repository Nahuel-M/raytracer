mod algebra;
mod hit;
mod material;
mod renderer;
mod world;

use image::io::Reader;
use show_image::{create_window, AsImageView, WindowOptions, event};
use std::f64::consts::PI;

use image::RgbImage;

use crate::algebra::vec3::Vec3;
use crate::material::map::RgbMap;
use crate::renderer::Renderer;
use crate::world::camera::Camera;
use crate::world::World;
// const WIDTH: u32 = 1920;
// const HEIGHT: u32 = 1080;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
#[show_image::main]
fn main() {
    let mut camera = Camera::new(PI / 4., WIDTH);
    camera.position = Vec3::new(10., 8., 10.);
    camera.look_at(Vec3::new(0., 0., 0.));
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let mut world = World::with_camera(camera);
    world.import_3d_file("D:\\Git\\Rust\\raytracer\\models\\medieval_house.obj")
        .unwrap();
    world.import_material_file("models\\medieval_house.mtl")
        .unwrap();

    println!("{:?}", world.materials);
    // println!("{:?}", world.models);
    let clouds = Reader::open("D:\\Git\\Rust\\raytracer\\images\\above_clouds.jpg")
        .unwrap().decode().unwrap().into_rgb32f();

    world.background = RgbMap::Texture(clouds);

    let mut renderer = Renderer::default();

    let window = create_window(
        "image",
        WindowOptions::new()
            .set_size(Some([WIDTH, HEIGHT]))
            .set_default_controls(true),
    )
    .unwrap();

    renderer.render(&world, &mut image, 1, 3);

    window
        .set_image("render", image.as_image_view().unwrap())
        .unwrap();

    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::R)
            && event.input.state.is_pressed()
            {
                renderer.render(&world, &mut image, 1, 3);
                window
                .set_image("render", image.as_image_view().unwrap())
                .unwrap();
            }
        }
    }
}
