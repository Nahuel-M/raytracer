use std::fmt::Display;

pub struct Model{
    pub name: String,
    pub vertex_indexes : Vec<usize>,
}

impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model \"{}\" with {} vertices.", self.name, self.vertex_indexes.len())
    }
}