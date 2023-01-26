use std::ops::{Add, Mul};

use super::vec3::Vec3;
#[derive(Debug, Clone, Copy)]
pub struct Rot3 {
    pub matrix : [f64;9]
}
impl Rot3 {
    pub fn new() -> Self {
        Rot3 {
           matrix : [0_f64;9]
        }
    }

    pub fn from_vec_pair(from_vec: Vec3, to_vec: Vec3) -> Rot3 {
        let cross = from_vec.cross(&to_vec);
        let dot = from_vec.dot(&to_vec);
        let identity = Rot3{matrix:[1f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 1f64]};
        let v =  Rot3{matrix:[0., -cross.z, cross.y, cross.z, 0., -cross.x, -cross.y, cross.x, 0.]};
        identity + v + v*v*(1./1.+dot)
    }

}

impl Mul<Rot3> for Rot3{
    type Output = Rot3;

    fn mul(self, rhs: Rot3) -> Self::Output {
        Rot3{matrix : [
            self.matrix[0] * rhs.matrix[0] + self.matrix[1] * rhs.matrix[3] + self.matrix[2] * rhs.matrix[6],
            self.matrix[3] * rhs.matrix[0] + self.matrix[4] * rhs.matrix[3] + self.matrix[5] * rhs.matrix[6],
            self.matrix[6] * rhs.matrix[0] + self.matrix[7] * rhs.matrix[3] + self.matrix[8] * rhs.matrix[6],
            self.matrix[0] * rhs.matrix[1] + self.matrix[1] * rhs.matrix[4] + self.matrix[2] * rhs.matrix[7],
            self.matrix[3] * rhs.matrix[1] + self.matrix[4] * rhs.matrix[4] + self.matrix[5] * rhs.matrix[7],
            self.matrix[6] * rhs.matrix[1] + self.matrix[7] * rhs.matrix[4] + self.matrix[8] * rhs.matrix[7],
            self.matrix[0] * rhs.matrix[2] + self.matrix[1] * rhs.matrix[5] + self.matrix[2] * rhs.matrix[8],
            self.matrix[3] * rhs.matrix[2] + self.matrix[4] * rhs.matrix[5] + self.matrix[5] * rhs.matrix[8],
            self.matrix[6] * rhs.matrix[2] + self.matrix[7] * rhs.matrix[5] + self.matrix[8] * rhs.matrix[8],
        ]}
    }
}

impl Add<Rot3> for Rot3{
    type Output = Rot3;
    fn add(self, rhs: Rot3) -> Self::Output {
        Rot3{matrix : [
            self.matrix[0] + rhs.matrix[0],
            self.matrix[1] + rhs.matrix[1],
            self.matrix[2] + rhs.matrix[2],
            self.matrix[3] + rhs.matrix[3],
            self.matrix[4] + rhs.matrix[4],
            self.matrix[5] + rhs.matrix[5],
            self.matrix[6] + rhs.matrix[6],
            self.matrix[7] + rhs.matrix[7],
            self.matrix[8] + rhs.matrix[8],
        ]}
    }
}

impl Mul<f64> for Rot3{
    type Output = Rot3;

    fn mul(self, rhs: f64) -> Self::Output {
        Rot3{matrix : [
            self.matrix[0] * rhs,
            self.matrix[1] * rhs,
            self.matrix[2] * rhs,
            self.matrix[3] * rhs,
            self.matrix[4] * rhs,
            self.matrix[5] * rhs,
            self.matrix[6] * rhs,
            self.matrix[7] * rhs,
            self.matrix[8] * rhs,
        ]}
    }
}


impl Mul<Vec3> for Rot3{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.matrix[0] * rhs.x + self.matrix[1] * rhs.y + self.matrix[2] * rhs.z,
            y: self.matrix[3] * rhs.x + self.matrix[4] * rhs.y + self.matrix[5] * rhs.z,
            z: self.matrix[6] * rhs.x + self.matrix[7] * rhs.y + self.matrix[8] * rhs.z,
        }
    }
}