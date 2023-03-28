use std::{marker::PhantomData, ops::{Add, Div, Mul, AddAssign}, iter::Sum};
use image::{Rgb, Rgb32FImage};
use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Linear;
#[derive(Copy, Clone)]
pub struct SRgb;

#[derive(Clone, Copy, Debug)]
pub struct Color<Space = Linear>{
    rgb : Vec3,
    space : PhantomData<Space>
}

impl<Space> Color<Space>{
    pub fn clamp_to_rgb(&self) -> Rgb<u8>{
        Rgb([(self.rgb.x*255.).min(255.) as u8, (self.rgb.y*255.).min(255.) as u8, (self.rgb.z*255.).min(255.) as u8])
    }
    pub const BLACK : Color<Space> = Color::<Space>{
        rgb: Vec3::ZEROS,
        space : PhantomData::<Space>
    };
    pub const WHITE : Color<Space> = Color::<Space>{
        rgb: Vec3::ONES,
        space : PhantomData::<Space>
    };
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
impl<Space> AddAssign<Color<Space>> for Color<Space>{
    fn add_assign(self: &mut Color<Space>, rhs: Color<Space>){
        self.rgb += rhs.rgb;
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
impl<Space> Sum for Color<Space>{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, |acc, val| Color{
            rgb: val.rgb + acc.rgb,
            space: PhantomData::<Space>,
        })
    }
}
impl<Space> Mul<Color<Space>> for Vec3{
    type Output = Color<Space>;
    fn mul(self, rhs: Color<Space>) -> Self::Output {
        Color::<Space>{
            rgb: rhs.rgb * self,
            space: rhs.space,
        }
    }
}

pub trait SpaceCast{
    fn srgb_to_linear(self) -> Self;
    fn linear_to_srgb(self) -> Self;
}

impl SpaceCast for Rgb32FImage{
    fn srgb_to_linear(mut self) -> Self{
        let gamma = 2.2;
        self
            .pixels_mut()
            .for_each(|pixel| {
                pixel.0[0] = pixel.0[0].powf(gamma);
                pixel.0[1] = pixel.0[1].powf(gamma);
                pixel.0[2] = pixel.0[2].powf(gamma);
            });
        self
    }
    
    fn linear_to_srgb(mut self) -> Self{    
        let inv_gamma = 1./2.2;
        self
            .pixels_mut()
            .for_each(|pixel| {
                pixel.0[0] = pixel.0[0].powf(inv_gamma);
                pixel.0[1] = pixel.0[1].powf(inv_gamma);
                pixel.0[2] = pixel.0[2].powf(inv_gamma);
            });
        self
    }
}

