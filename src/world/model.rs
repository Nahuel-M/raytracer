use std::{fmt::Display, sync::{Arc, RwLock}};

use crate::material::Material;

pub struct Model{
    pub vertex_indexes : Vec<usize>,
    pub material : Arc<RwLock<Material>>
}

impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model with {} vertices.", self.vertex_indexes.len())
    }
}