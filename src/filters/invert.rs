use crate::filters::Manipulate;
use ndarray::Array3;

#[derive(Default)]
pub struct Invert;

impl Invert {
    pub fn new() -> Self { Self }
}

impl Manipulate for Invert {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8> {
        255 - img
    }
}