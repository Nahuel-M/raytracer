use crate::algebra::vec3::Vec3;
#[derive(Debug)]
pub struct Hit {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub parallel_to_surface: Vec3,
}

impl Hit {
    pub fn max() -> Self {
        Hit {
            distance: f64::MAX,
            position: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            parallel_to_surface: Vec3::new(0., 0., 0.),
        }
    }
}
