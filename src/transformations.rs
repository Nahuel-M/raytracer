use crate::{shape::polygon::Polygon, Vec3};

#[allow(dead_code)]
pub fn translate(polygons : &mut Vec<Polygon>, factor : Vec3){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            *vertex += factor;
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
#[allow(dead_code)]
pub fn flip_x(polygons : &mut Vec<Polygon>){
    for polygon in polygons{
        for vertex in &mut polygon.vertices{
            vertex.x *= -1.;
        }
        polygon.normal.x *= -1.;
    }
}
