mod ray;
mod renderer;
mod parser;
mod shapes;
mod window;
mod transformations;
mod scene;
mod iter_functions;

use std::fs;

use show_image::{create_window, AsImageView, WindowOptions};

use image::{RgbaImage, Rgba};
use nalgebra::*;
use scene::camera::Camera;
use shapes::polygon::Polygon;

use crate::{transformations::{translate, flip_z, scale, flip_x}, renderer::RayRenderable};


// const WIDTH: u32 = 1280;
// const HEIGHT: u32 = 720;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;
#[show_image::main]
fn main() {
    let mut polygons : Vec<Polygon> = parser::parse_ascii_stl_no_normals(fs::read_to_string("models/teapot.stl").unwrap().as_str()).unwrap(); // NO WORKS
    // let mut polygons : Vec<Polygon> = parser::parse_ascii_stl(include_str!("models/teapot.stl")).unwrap(); // WORKS
    translate(&mut polygons, Vector3::<f64>::new(-80., -40., 0.));
    scale(&mut polygons, 0.01); 
    flip_z(&mut polygons);
    // let polygons : Vec<Polygon> = parser::parse_ascii_stl_no_normals(include_str!("models/test.stl")).unwrap();
    let mut camera = Camera::new(f64::frac_pi_2(), WIDTH);
    camera.location = Vector3::new(0.,0., -1.0);
    camera.rotation = Rotation3::from_euler_angles(0., 0.5, 0.);
    let test = polygons[1].get_hit_distance(&camera.ray_for_pixel(-45, -10));
    println!("{:?}", camera.ray_for_pixel(-45, -10));
    println!("{:?}", test);
    println!("{:?}", polygons);
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    let window = create_window("image", WindowOptions::new().set_size(Some([WIDTH*4, HEIGHT*4]))).unwrap();
    renderer::render_polygons(&camera, &polygons, &mut img);
    window.set_image("render", img.as_image_view().unwrap()).unwrap();
    println!("Done rendering.");
    window.wait_until_destroyed().unwrap();
}
