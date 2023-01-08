use nalgebra::Vector3;

pub struct Camera {
    pub location: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub fov_radians_horizontal: f64,
}

impl Camera {
    pub fn new(fov_radians_horizontal: f64) -> Self {
        Camera {
            location: Vector3::new(0., 0., 0.),
            direction: Vector3::new(0., 0., 0.),
            fov_radians_horizontal,
        }
    }
}
