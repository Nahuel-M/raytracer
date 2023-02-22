use crate::{renderer::tracer::trace_package::TracePackage, algebra::vec3::Vec3};

pub enum ShadePackage{
    Ray(TracePackage),
    Color(Vec3)
}

impl From<Vec3> for ShadePackage{
    fn from(val: Vec3) -> Self {
        ShadePackage::Color(val)
    }
}

impl From<TracePackage> for ShadePackage{
    fn from(val: TracePackage) -> Self {
        ShadePackage::Ray(val)
    }
}