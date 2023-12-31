use ndarray::Array3;
use crate::filters::Manipulate;

#[derive(Default)]
pub struct Invert;

impl Invert {
    pub fn new() -> Self { Self }
}

impl Manipulate for Invert {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        255 - img
    }

    fn details_str(&self) -> String {
        "Color invert".to_string()
    }
}