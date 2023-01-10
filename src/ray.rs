use nalgebra::{Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction_unit: Vector3<f64>,
}

impl Ray{
    pub fn at(&self, distance : f64) -> Vector3<f64>{
        self.origin + self.direction_unit*distance
    }
}
