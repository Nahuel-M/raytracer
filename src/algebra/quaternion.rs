use std::ops::{Div, Mul};
use std::fmt::Display;


use crate::Vec3;
#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    real: f64,
    i: f64,
    j: f64,
    k: f64,
}

#[allow(dead_code)]
impl Quaternion {
    pub fn new() -> Self{
        Quaternion{real: 0., i: 0., j: 0., k: 0.}
    }
    pub fn from_unit_vectors(from: Vec3, to: Vec3) -> Self {
        let d = from.dot(&to);
        let w = from.cross(&to);

        Quaternion::from_vector(d + f64::sqrt(d * d + w.dot(&w)), w).normalize()
    }
    pub fn from_vector(real: f64, vec3: Vec3) -> Self {
        Quaternion {
            real,
            i: vec3.x,
            j: vec3.y,
            k: vec3.z,
        }
    }

    pub fn normalize(&self) -> Quaternion {
        *self / self.magnitude()
    }
    
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.real * self.real + self.i * self.i + self.j * self.j + self.k * self.k)
    }

    pub fn conjugate(&self) -> Quaternion{
        Quaternion{real: self.real, i: -self.i, j: -self.j, k: -self.k}
    }

    pub fn rotate_vector(self, vec3: &mut Vec3){
        *vec3 = (self * (*vec3).into() * self.conjugate()).into()
    }
}

impl Display for Quaternion{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.real, self.i, self.j, self.k)
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec3> for Quaternion{
    fn from(value: Vec3) -> Self {
        Quaternion { real: 0., i: value.x, j: value.y, k: value.z }
    }
}

impl From<Quaternion> for Vec3{
    fn from(val: Quaternion) -> Self {
        Vec3{x: val.i, y: val.j, z: val.k}
    }
}

impl Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f64) -> Self::Output {
        Quaternion {
            real: self.real / rhs,
            i: self.i / rhs,
            j: self.j / rhs,
            k: self.k / rhs,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            real: self.real * rhs.real - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.i * rhs.real + self.real * rhs.i + self.j * rhs.k - self.k * rhs.j,
            j: self.j * rhs.real + self.real * rhs.j + self.k * rhs.i - self.i * rhs.k,
            k: self.k * rhs.real + self.real * rhs.k + self.i * rhs.j - self.j * rhs.i,
        }
    }
}
