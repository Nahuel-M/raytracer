mod camera;
mod ray;
mod renderer;
mod parser;
mod shapes;
mod vec;
mod window;
mod transformations;

use std::io::stdin;

use image::RgbaImage;
use nalgebra::*;
use shapes::polygon::Polygon;
use window::Window;
use camera::Camera;
use shapes::sphere::Sphere;
use shapes::mandelbulb::Mandelbulb;


// const WIDTH: u32 = 1280;
// const HEIGHT: u32 = 720;
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;

fn main() {
    let polygons : Vec<Polygon> = parser::parse_ascii_stl(include_str!("test.stl")).unwrap();
    println!("{:?}", polygons);
    // let sphere = Sphere::new(0.0, 0.0, 10.0, 1.0);
    let mut camera = Camera::new(f64::frac_pi_2());
    camera.location = Vector3::new(0.,0., -2.0);

    let mut window = Window::new(WIDTH, HEIGHT);
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    while let Some(e) = window.next() {
        // renderer::render_sphere(&camera, &sphere, &mut img);
        // renderer::render_element(&camera, &polygon, &mut img);
        renderer::render_polygons(&camera, &polygons, &mut img);
        window.show_image(&img, e);
        println!("image");
        camera.fov_radians_horizontal += 0.01;
    }
    let mut s=String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
}
