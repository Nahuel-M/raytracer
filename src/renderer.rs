use crate::{
    ray::Ray,
    scene::camera::Camera,
    shapes::{polygon::Polygon, sphere::Sphere},
};
use image::{ImageBuffer, Rgba};
use nalgebra::Vector3;

pub trait RayRenderable {
    fn get_hit_distance(&self, ray: &Ray) -> Option<f64>;
}

pub fn render_sphere(camera: &Camera, sphere: &Sphere, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let half_width: i32 = img.width() as i32 / 2;
    let half_height: i32 = img.height() as i32 / 2;
    let light = Vector3::new(1.0, 1.0, 1.0).normalize();

    let mut ray : Ray;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        ray = camera.ray_for_pixel(x as i32 - half_width, y as i32 - half_height);

        if let Some(distance) = sphere.get_hit_distance(&ray) {
            let normals = (ray.at(distance) - sphere.location) / sphere.radius;
            let strength = -normals.dot(&light).min(0.0);
            *pixel = Rgba([
                (255.0 * strength) as u8,
                (255.0 * strength) as u8,
                (255.0 * strength) as u8,
                255,
            ])
        } else {
            *pixel = Rgba([0, 0, 0, 255])
        }
    }
}

pub fn render_polygons(
    camera: &Camera,
    polygons: &Vec<Polygon>,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    let half_width: i32 = img.width() as i32 / 2;
    let half_height: i32 = img.height() as i32 / 2;
    let light = Vector3::new(1.0, 1.0, 1.0).normalize();

    let mut ray : Ray;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        ray = camera.ray_for_pixel(x as i32 - half_width, y as i32 - half_height);
        let mut lowest_distance = f64::MAX;
        let mut lowest_distance_polygon: Option<&Polygon> = None;
        for polygon in polygons {
            if let Some(distance) = polygon.get_hit_distance(&ray) {
                if distance < 0. { continue; }
                if distance < lowest_distance {
                // *pixel = Rgba([(255.0) as u8, (255.0) as u8, (255.0) as u8, 255]);
                lowest_distance_polygon = Some(polygon);
                lowest_distance = distance;
                }
            }
        }
        if let Some(polygon) = lowest_distance_polygon {
            // println!("test");
            let strength = polygon.normal.dot(&light).abs();
            *pixel = Rgba([
                (255.0 * strength) as u8,
                (255.0 * strength) as u8,
                (255.0 * strength) as u8,
                255,
            ])
        } else {
            *pixel = Rgba([0, 0, 0, 255])
        }
    }
}

pub fn render_element<T: RayRenderable>(
    camera: &Camera,
    element: &T,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    let half_width: i32 = img.width() as i32 / 2;
    let half_height: i32 = img.height() as i32 / 2;
    let (mut min_dist, mut max_dist) = (9999f64, 0f64);

    let mut ray : Ray;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        ray = camera.ray_for_pixel(x as i32 - half_width, y as i32 - half_height);
        if let Some(distance) = element.get_hit_distance(&ray) {
            if distance < min_dist {
                min_dist = distance;
            }
            if distance > max_dist {
                max_dist = distance;
            }
            // println!("{distance}");
            *pixel = Rgba([255, 255, 255, 255])
        } else {
            *pixel = Rgba([0, 0, 0, 255]);
        }
    }
}
