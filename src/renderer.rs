use crate::{
    camera::Camera,
    ray::Ray,
    shapes::{
        polygon::Polygon,
        sphere::{Sphere},
    },
};
use image::{ImageBuffer, Rgba};
use nalgebra::Vector3;

pub trait RayRenderable {
    fn get_hit_distance(&self, ray: &Ray) -> Option<f64>;
}

pub fn render_sphere(camera: &Camera, sphere: &Sphere, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let pixel_size = (camera.fov_radians_horizontal / 2.0).tan() / img.width() as f64;
    let half_width = img.width() as f64 / 2.0;
    let half_height = img.height() as f64 / 2.0;
    let light = Vector3::new(1.0, 1.0, 1.0).normalize();

    let mut ray = Ray {
        origin: camera.location,
        direction_unit: Vector3::new(0., 0., 0.),
    };
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let pixel_normal = Vector3::new(
            (x as f64 - half_width) * pixel_size,
            (y as f64 - half_height) * pixel_size,
            1f64,
        )
        .normalize();

        ray.direction_unit = pixel_normal;

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
    let pixel_size = (camera.fov_radians_horizontal / 2.0).tan() / img.width() as f64;
    let half_width = img.width() as f64 / 2.0;
    let half_height = img.height() as f64 / 2.0;
    let light = Vector3::new(1.0, 1.0, 1.0).normalize();

    let mut ray = Ray {
        origin: camera.location,
        direction_unit: Vector3::new(0., 0., 0.),
    };
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let pixel_normal = Vector3::new(
            (x as f64 - half_width) * pixel_size,
            (y as f64 - half_height) * pixel_size,
            1f64,
        )
        .normalize();

        ray.direction_unit = pixel_normal;
        // let mut lowest_distance = f64::MAX;
        let mut lowest_distance_polygon: Option<&Polygon> = None;
        for polygon in polygons {
            if let Some(distance) = polygon.get_hit_distance(&ray) {
                // if distance < lowest_distance {
                    *pixel = Rgba([
                        (255.0) as u8,
                        (255.0) as u8,
                        (255.0) as u8,
                        255,
                    ]);
                    lowest_distance_polygon = Some(polygon);
                    break;
                    // lowest_distance = distance;
                // }
            }
        }
        if let Some(polygon) = lowest_distance_polygon {
            // println!("test");
            // let strength = polygon.normal.dot(&light).abs();
            // *pixel = Rgba([
            //     (255.0 * strength) as u8,
            //     (255.0 * strength) as u8,
            //     (255.0 * strength) as u8,
            //     255,
            // ])
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
    let pixel_size = (camera.fov_radians_horizontal / 2.0).tan() / img.width() as f64;
    let half_width = img.width() as f64 / 2.0;
    let half_height = img.height() as f64 / 2.0;
    let (mut min_dist, mut max_dist) = (9999f64, 0f64);

    let mut ray = Ray {
        origin: camera.location,
        direction_unit: Vector3::new(0., 0., 0.),
    };

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let pixel_normal = Vector3::new(
            (x as f64 - half_width) * pixel_size,
            (y as f64 - half_height) * pixel_size,
            1f64,
        )
        .normalize();

        ray.direction_unit = pixel_normal;

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
