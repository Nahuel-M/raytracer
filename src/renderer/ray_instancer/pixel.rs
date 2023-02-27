#[derive(Clone, Copy)]
pub struct Pixel{
    pub x : usize,
    pub y : usize,
}

impl Pixel{

}

impl From<(usize, usize)> for Pixel{
    fn from(value: (usize, usize)) -> Self {
        Self{x: value.0, y: value.1}
    }
}