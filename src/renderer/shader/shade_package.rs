use crate::{renderer::tracer::trace_package::TracePackage, algebra::{vec3::Vec3, color::Color}};

pub enum ShadePackage{
    Ray(TracePackage),
    Color(Color)
}

impl From<Vec3> for ShadePackage{
    fn from(val: Vec3) -> Self {
        ShadePackage::Color(val.into())
    }
}

impl From<TracePackage> for ShadePackage{
    fn from(val: TracePackage) -> Self {
        ShadePackage::Ray(val)
    }
}