use nalgebra::Vector3;

pub enum LightType{
    Point(Vector3<f64>),
    Directional(Vector3<f64>, Vector3<f64>),
    SkyLight(Vector3<f64>),
    Ambient,
}

pub struct Light{
    color : Vector3<f64>,
    light_type : LightType
}