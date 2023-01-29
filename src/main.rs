mod hittable;
mod iter_functions;
mod material;
mod parser;
mod ray;
mod scene;
mod shape;
mod transformations;
mod algebra;
mod hit;

use std::f64::consts::PI;
use std::fs;

use show_image::{create_window, AsImageView, WindowOptions};

use image::RgbaImage;
use scene::camera::Camera;
use scene::Scene;
use shape::triangle::Triangle;
use shape::sphere::Sphere;

use crate::algebra::vec3::Vec3;
use crate::hittable::Hittable;
// const WIDTH: u32 = 1280;
// const HEIGHT: u32 = 720;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
#[show_image::main]
fn main() {
    let teapot_string = fs::read_to_string("models/teapot.stl").unwrap();
    let mut teapot = parser::parse_ascii_stl(teapot_string.as_str()).unwrap();
    teapot += Vec3::new(-80., -5., 0.1,);
    teapot *= 0.01;
    let mut camera = Camera::new(PI / 4., WIDTH);
    camera.position = Vec3::new(2., 0.5, 2.);
    camera.look_at(Vec3::new(0., 0.2, 0.));
    let img = RgbaImage::new(WIDTH, HEIGHT);

    let mut scene = Scene::new(camera, img);
    add_floor_to_scene(&mut scene);
    add_sky_light_to_scene(&mut scene);
    scene.world.background = Vec3::new(0.5,0.5,0.5);
    scene.add_hittable(Into::<Hittable>::into(teapot).with_color(0., 0., 0.8).with_specular(0.5));
    print!("{}", scene);
    scene.render(4);


    let window = create_window(
        "image",
        WindowOptions::new().set_size(Some([WIDTH, HEIGHT])),
    )
    .unwrap();
    // renderer::render_polygons(&camera, &polygons, &mut img);
    window
        .set_image("render", scene.image.as_image_view().unwrap())
        .unwrap();
    window.wait_until_destroyed().unwrap();
}

#[allow(unused)]
fn add_basic_elements(scene : &mut Scene){
    scene.add_hittable(
        Hittable::with_sphere(Sphere::new(0, 0, 1.2, 1))
            .with_color(1., 1., 0.)
            .with_specular(0.3)
            .with_refraction(0.7)
            .with_ior(4.),
    );
    scene.add_hittable(
        Hittable::with_polygon(Triangle::looking_at_position(
            Vec3::new(13., 2., 15.), scene.world.camera.position, 4.))
            .with_luminance(1.,1.,1.)
    );
    scene.add_hittable(Hittable::with_sphere(Sphere::new(2, 0, 0.5, 0.5)).with_color(1., 1., 1.));
    scene.add_hittable(
        Hittable::with_polygon(Triangle::new(
            (0., -1., -5.),
            (200., -1., 100.),
            (-200., -1., 100.),
        ))
        .with_color(1., 0.5, 1.),
    );
    let light = Triangle::new((-30., 3., 10.), (30., 3., 10.), (0., 3., -30.));
    scene.add_hittable(
        Hittable::with_polygon(light)
            .with_color(0., 0., 0.)
            .with_luminance(0.6, 0.6, 0.6),
    );

    // println!("{:?}", scene.objects);
}

fn add_floor_to_scene(scene : &mut Scene){
    scene.add_hittable(
        Hittable::with_polygon(Triangle::new(
            (0., 0., -300.),
            (200., 0., 200.),
            (-200., 0., 200.),
        ))
        .with_color(1., 0.5, 1.),
    );
}

fn add_sky_light_to_scene(scene : &mut Scene) {
    let light = Triangle::new((-20., 4., 20.), (20., 4., 10.), (0., 4., -20.));
    scene.add_hittable(
        Hittable::with_polygon(light)
            .with_color(0., 0., 0.)
            .with_luminance(0.9, 0.9, 0.9),
    );
}