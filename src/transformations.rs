
use nalgebra::Vector3;

use crate::shapes::polygon::{Polygon, self};

pub fn scale(polygons : &mut Vec<Polygon>, factor : Vector3<f64>){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            vertex.component_mul_assign(&factor);
        }
        polygon.normal.component_mul_assign(&Vector3::<f64>::new(factor.y*factor.z, factor.x*factor.z, factor.x*factor.y));
    }
}