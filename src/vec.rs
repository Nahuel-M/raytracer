use nalgebra::Vector3;

pub trait Clamp<RHS> {
    fn clamp (&self, max:RHS) -> Self;
}

impl<T: PartialOrd + Copy> Clamp<T> for Vector3<T> {
    fn clamp (&self, max: T) -> Self {
        Vector3::new(
            if self[0] > max {max} else {self[0]},
            if self[1] > max {max} else {self[1]},
            if self[2] > max {max} else {self[2]},
        )
    }
}
