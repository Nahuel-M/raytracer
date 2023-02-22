use crate::{algebra::vec3::Vec3};

type VertexIndex = usize;
type VertexNormalIndex = usize;
type VertexUVIndex = usize;

// #[derive(Clone, Copy)]
pub struct Triangle {
    pub normal: Vec3,
    pub vertices: [VertexIndex; 3],
    pub smoothing: bool,
    pub vertex_normals : Option<[VertexNormalIndex; 3]>,
    pub vertex_uvs : Option<[VertexUVIndex; 3]>,
}