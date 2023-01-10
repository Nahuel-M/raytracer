use self::{sphere::Sphere, polygon::Polygon, mandelbulb::Mandelbulb};

pub mod sphere;
pub mod polygon;
pub mod mandelbulb;

pub enum Shapes{
    Sphere(Sphere),
    Polygon(Polygon),
}