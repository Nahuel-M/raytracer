#[derive(Clone, Copy, Debug)]
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

    pub const ALL : [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
}