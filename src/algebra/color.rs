use std::{marker::PhantomData, ops::{Add, Div}};

use image::Rgb;

use super::vec3::Vec3;

pub struct Linear;
pub struct SRgb;

pub struct Color<Space = Linear>{
    rgb : Vec3,
    space : PhantomData<Space>
}

impl Color{
    pub fn clamp_to_rgb(&self) -> Rgb<u8>{
        Rgb([(self.rgb.x*255.).min(255.) as u8, (self.rgb.y*255.).min(255.) as u8, (self.rgb.z*255.).min(255.) as u8])
    }
}

impl Color<Linear>{
    pub fn to_srgb(self) -> Color<SRgb>{
        let inv_gamma = 1./2.2;
        Color{
            rgb: Vec3{
                x: self.rgb.x.powf(inv_gamma),
                y: self.rgb.y.powf(inv_gamma),
                z: self.rgb.z.powf(inv_gamma),
            },
            space: PhantomData::<SRgb>,
        }
    }
}
impl Color<SRgb>{
    pub fn to_linear(self) -> Color<Linear>{
        let gamma = 2.2;
        Color{
            rgb: Vec3{
                x: self.rgb.x.powf(gamma),
                y: self.rgb.y.powf(gamma),
                z: self.rgb.z.powf(gamma),
            },
            space: PhantomData::<Linear>,
        }
    }
}

impl<Space> Add<Color<Space>> for Color<Space>{
    type Output = Color<Space>;
    fn add(self, rhs: Color<Space>) -> Self::Output {
        Color{
            rgb: self.rgb + rhs.rgb,
            ..self
        }
    }
}
impl<Space> Add<&Color<Space>> for Color<Space>{
    type Output = Color<Space>;
    fn add(self, rhs: &Color<Space>) -> Self::Output {
        Color{
            rgb: self.rgb + rhs.rgb,
            ..self
        }
    }
}
impl<Space> Div<f64> for Color<Space>{
    type Output = Color<Space>;

    fn div(self, rhs: f64) -> Self::Output {
        Color{
            rgb: self.rgb / rhs,
            ..self
        }
    }
}
impl<Space> From<Vec3> for Color<Space>{
    fn from(value: Vec3) -> Self {
        Color { rgb: value, space: PhantomData::<Space> }
    }
}
