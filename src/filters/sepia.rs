use ndarray::{Array3, Axis, stack};
use crate::filters::Manipulate;
use crate::imgarray::AsImage;

#[derive(Default)]
pub struct Sepia;

impl Sepia {
    pub fn new() -> Self { Self }
}

impl Manipulate for Sepia {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (r, g, b) = img.rgb_as_float();

        let nr = &r * 0.393 + &g * 0.769 + &b * 0.189;
        let ng = &r * 0.349 + &g * 0.686 + &b * 0.168;
        let nb = &r * 0.272 + &g * 0.534 + &b * 0.131;

        stack(Axis(2), &[
            nr.mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            ng.mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            nb.mapv(|x| x.min(255.0).max(0.0) as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        "Sepia".to_string()
    }
}