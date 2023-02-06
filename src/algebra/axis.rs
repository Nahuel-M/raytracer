#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub fn next(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}