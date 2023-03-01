use crate::algebra::color::Color;
use crate::algebra::ray::Ray;
use crate::renderer::ray_instancer::RayInstancer;
use crate::renderer::Tracer;
use super::{ImageChunk8, CHUNK_WIDTH};
use super::image_chunk::PixelColorArray;
use super::shader::Shader;
use super::shader::shade_package::ShadePackage;

type PixelColorArray8 = PixelColorArray<CHUNK_WIDTH>;


pub fn trace_chunk(chunk: ImageChunk8, ray_instancer: &RayInstancer, tracer : &Tracer, shader: &Shader) -> PixelColorArray8{
    let mut res = PixelColorArray8::new(chunk.top_left);
    for pixel in chunk.pixels(){
        let start_rays = ray_instancer.rays_for_pixel(pixel);
        let mut accum_col = Color::BLACK;
        for ray in &start_rays{
            accum_col += process_ray(ray, tracer, shader, 4);
        }
        res.set(pixel, accum_col / start_rays.len() as f64);
    }
    res
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