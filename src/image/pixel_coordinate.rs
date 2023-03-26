#[derive(Clone, Copy)]
pub struct PixelCoordinate{
    pub x : usize,
    pub y : usize,
}

impl From<(usize, usize)> for PixelCoordinate{
    fn from(value: (usize, usize)) -> Self {
        Self{x: value.0, y: value.1}
    }
}
