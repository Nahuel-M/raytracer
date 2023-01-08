use nalgebra::Vector3;
use crate::{ray::Ray, renderer::RayRenderable};

pub struct Mandelbulb{}

impl Mandelbulb{
    
    fn distance_at_point(point : Vector3<f64>) -> f64 {
        const POWER : f64 = 8.0;
        const BAILOUT : f64 = 10_000.0;
        let mut z = point;
        let mut dr = 1.0;
        let mut r = 0.0;
        for  _ in 0..40 {
            r = z.magnitude();
            if r>BAILOUT { break;}
            
            // convert to polar coordinates
            let mut theta = f64::acos(z.z/r);
            let mut phi = f64::atan2(z.y,z.x);
            dr =  r.powf( POWER-1.0 )*POWER*dr + 1.0;
            
            // scale and rotate the point
            let zr = r.powf(POWER);
            theta *= POWER;
            phi *= POWER;
            
            // convert back to cartesian coordinates
            z = zr*Vector3::new(theta.sin()*phi.cos(), phi.sin()*theta.sin(), theta.cos());
            z+=point;
        }
        0.5*r.ln()*r/dr
    }
}

impl RayRenderable for Mandelbulb{
    fn get_hit_distance(&self, ray: &Ray) -> Option<f64> {
        let mut point = ray.origin + ray.direction_unit;
        let mut distance = 0.0;
        for _ in 0..10{
            let distance_to_mandlebulb = Mandelbulb::distance_at_point(point);
            distance += distance_to_mandlebulb;
            if distance_to_mandlebulb < 0.001 {
                return Some(distance)
            }
            point = ray.origin + ray.direction_unit * distance;
            // println!("{distance_to_mandlebulb}");
        }
        // println!("{distance_to_mandlebulb}");
        None
    }
}