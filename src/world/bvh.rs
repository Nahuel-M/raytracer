use crate::algebra::vec3::Vec3;

use super::triangle::Triangle;

pub struct BoundedVolume<'a> {
    min_point: Vec3,
    max_point: Vec3,

    children: BoundedVolumeChildren<'a>,
}

pub enum Direction {
    X,
    Y,
    Z,
}
impl Direction{
    pub fn next(&self) -> Self{
        match self{
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}

pub enum BoundedVolumeChildren<'a> {
    BoundedVolumes([&'a BoundedVolume<'a>; 2]),
    TriangleIndexes(Vec<usize>),
}

impl BoundedVolume<'_> {
    pub fn fit_size_to_elements(elements: &Vec<Triangle>) {
        let points: Vec<Vec3> = elements
            .iter()
            .flat_map(|triangle| triangle.vertices.iter().map(|vertex| vertex.get()))
            .collect();
    }
}
