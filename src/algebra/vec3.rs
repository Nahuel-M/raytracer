use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, iter::Sum,
};

use image::Rgb;

use super::axis::Axis;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[allow(dead_code)]
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub const fn uniform(value: f64) -> Self {
        Vec3{x: value, y: value, z: value}
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn project(&self, onto : &Vec3) -> Vec3{
        *onto * self.dot(onto) / onto.dot(onto)
    }

    pub fn project_onto_unit_vector(&self, onto : Vec3) -> Vec3{
        onto * self.dot(&onto)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Vec3 {
        self / self.magnitude()
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }
    pub fn min(&self) -> f64{
        self.x.min(self.y).min(self.z)
    }
    pub fn max(&self) -> f64{
        self.x.max(self.y).max(self.z)
    }
    /// Element-wise minimum for x, y, z
    pub fn ew_min(&self, other: &Vec3) -> Vec3{
        Vec3::new(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
    }
    /// Element-wise maximum for x, y, z
    pub fn ew_max(&self, other: &Vec3) -> Vec3{
        Vec3::new(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
    }
    pub fn clamp_to_rgb(&self) -> Rgb<u8>{
        Rgb([(self.x*255.).min(255.) as u8, (self.y*255.).min(255.) as u8, (self.z*255.).min(255.) as u8])
    }

    pub fn axis(&self, axis: Axis) -> & f64{
        match axis{
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }

    pub fn axis_mut(&mut self, axis: Axis) -> &mut f64{
        match axis{
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
    pub const MAX : Vec3 = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
    pub const MIN : Vec3 = Vec3::new(f64::MIN, f64::MIN, f64::MIN);
    pub const X : Vec3 = Vec3 {x: 1., y: 0., z: 0. };
    pub const Y : Vec3 = Vec3 {x: 0., y: 1., z: 0. };
    pub const Z : Vec3 = Vec3 {x: 0., y: 0., z: 1. };
    pub const ZEROS : Vec3 = Vec3 {x: 0., y: 0., z: 0. };
    pub const ONES : Vec3 = Vec3 {x: 1., y: 1., z: 1. };
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vec3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<&Rgb<f32>> for Vec3 {
    fn from(value: &Rgb<f32>) -> Self {
        Vec3 {
            x: value.0[0] as f64,
            y: value.0[1] as f64,
            z: value.0[2] as f64,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y, self.z + rhs.z).into()
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        (self.x + rhs, self.y + rhs, self.z + rhs).into()
    }
}
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y, self.z - rhs.z).into()
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f64) -> Self::Output {
        (self.x - rhs, self.y - rhs, self.z - rhs).into()
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (self.x * rhs.x, self.y * rhs.y, self.z * rhs.z).into()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        (self.x * rhs, self.y * rhs, self.z * rhs).into()
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (self * rhs.x, self * rhs.y, self * rhs.z).into()
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        (self.x / rhs.x, self.y / rhs.y, self.z / rhs.z).into()
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (self.x / rhs, self.y / rhs, self.z / rhs).into()
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum<Vec3> for Vec3{
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        iter.reduce(|acc, vec3| acc + vec3).unwrap()
    }
}

impl<'a> Sum<&'a Self> for Vec3{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZEROS,|acc, &vec3| acc + vec3)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.3}, {:.3}, {:.3}]", self.x, self.y, self.z)
    }
}

impl AddAssign<Vec3> for Vec<Vec3>{
    fn add_assign(&mut self, rhs: Vec3) {
        self.iter_mut().for_each(|vec3| *vec3 += rhs);
    }
}

pub trait MinMaxVec3{
    fn min_max(self) -> (Vec3, Vec3);
}

impl<'a, T: Iterator<Item = &'a Vec3>> MinMaxVec3 for T{
    fn min_max(self) -> (Vec3, Vec3){
        self.fold(
            (Vec3::MAX, Vec3::MIN), 
            |(min, max), new_vec| 
            (new_vec.ew_min(&min), new_vec.ew_max(&max))
        )
    }
}

impl Default for Vec3{
    fn default() -> Self {
        Vec3::ZEROS
    }
}