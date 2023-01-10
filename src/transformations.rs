use nalgebra::Vector3;

use crate::shapes::polygon::Polygon;
#[allow(dead_code)]
pub fn translate(polygons : &mut Vec<Polygon>, factor : Vector3<f64>){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            *vertex += &factor;
        }
    }
}
#[allow(dead_code)]
pub fn scale(polygons : &mut Vec<Polygon>, factor : f64){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            *vertex *= factor;
        }
    }
}

#[allow(dead_code)]
pub fn flip_z(polygons : &mut Vec<Polygon>){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            vertex.z *= -1.;
        }
        polygon.normal.z *= -1.;
    }
}

pub fn flip_x(polygons : &mut Vec<Polygon>){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            vertex.x *= -1.;
        }
        polygon.normal.x *= -1.;
    }
}