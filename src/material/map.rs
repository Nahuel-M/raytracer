use image::{ImageBuffer, Luma, Rgb};

use crate::algebra::vec3::Vec3;
 
#[allow(dead_code)]
#[derive(Debug)]
pub enum RgbMap{
    Color(Vec3),
    Texture(ImageBuffer<Rgb<f32>, Vec<f32>>)
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum LumaMap{
    Value(f64),
    Texture(ImageBuffer<Luma<f32>, Vec<f32>>)
}

pub trait GetValueAt<T>{
    fn get_value_at(&self, u: f64, v: f64) -> T;
}

impl GetValueAt<Vec3> for ImageBuffer<Rgb<f32>, Vec<f32>>{
    fn get_value_at(&self, u: f64, v: f64) -> Vec3 {
        // Bilinear interpolation
        let u_2 = self.width() as f64 * u.min(0.99999);
        let v_2 = self.height() as f64 * v.min(0.99999);

        let (u_low, u_high) = (u_2 as u32, (u_2 as u32 + 1) % self.width());
        let (v_low, v_high) = (v_2 as u32, (v_2 as u32 + 1) % self.height());    

        let u_factor = u_2 - u_low as f64; 
        let v_factor = v_2 - v_low as f64;

        let ll : Vec3 = self.get_pixel(u_low, v_low).into();
        let lh : Vec3 = self.get_pixel(u_low , v_high).into();
        let hl : Vec3 = self.get_pixel(u_high, v_low).into();
        let hh : Vec3 = self.get_pixel(u_high, v_high).into();

        ll * (1.-v_factor) * (1.-u_factor) + 
        lh * v_factor * (1.-u_factor) + 
        hl * (1.-v_factor) * u_factor + 
        hh * u_factor * v_factor
    }
}

impl From<Vec3> for RgbMap{
    fn from(val: Vec3) -> Self {
        RgbMap::Color(val)
    }
}

impl From<f64> for LumaMap{
    fn from(val: f64) -> Self {
        LumaMap::Value(val)
    }
}