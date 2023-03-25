use crate::algebra::color::Color;
use crate::algebra::ray::Ray;
use crate::renderer::ray_instancer::RayInstancer;
use crate::renderer::Tracer;
use super::{ImageChunk8, CHUNK_WIDTH};
use super::image_chunk::PixelColorArray;
use super::shader::Shader;
use super::shader::shade_package::ShadePackage;

pub type PixelColorArray8 = PixelColorArray<CHUNK_WIDTH>;


pub fn trace_chunk(chunk: ImageChunk8, ray_instancer: &RayInstancer, tracer : &Tracer, shader: &Shader, max_bounces : u8) -> PixelColorArray8{
    let mut result = PixelColorArray8::new(chunk.top_left);
    for pixel in chunk.pixels(){
        let start_rays = ray_instancer.rays_for_pixel(pixel);
        let mut accumulated_color = Color::BLACK;
        for ray in &start_rays{
            accumulated_color += process_ray(ray, tracer, shader, max_bounces);
        }
        result.set(pixel, accumulated_color / start_rays.len() as f64);
    }
    result
}

fn process_ray(ray: &Ray, tracer: &Tracer, shader: &Shader, remaining_bounces: u8) -> Color {
    if remaining_bounces == 0 {
        return Color::BLACK;
    }

    let trace_result = tracer.trace_ray(ray);
    let shade_result = shader.shade_hit(&trace_result, ray);

    shade_result
    .iter()
    .map(|shade_package| match shade_package{
        ShadePackage::Ray(trace_package) => {
            trace_package.multiplier *
            process_ray(
                &trace_package.ray,
                tracer,
                shader,
                remaining_bounces - 1)
        },
        ShadePackage::Color(color) => *color,
    })
    .sum()
}