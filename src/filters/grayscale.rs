use ndarray::{Array2, Array3, stack, Axis};
use crate::filters::Manipulate;
use crate::imgarray::AsImage;

#[derive(Default)]
pub struct Grayscale;

impl Grayscale {
    pub fn new() -> Self { Self }
}

impl Manipulate for Grayscale {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (r, g, b) = img.rgb_as_float();
        let luma: Array2<f64> = r * 0.2126 + g * 0.7152 + b * 0.0722; // https://en.wikipedia.org/wiki/Grayscale#Colorimetric_(perceptual_luminance-preserving)_conversion_to_grayscale
        let res_chan = luma.mapv(|x| x as u8);
        stack(Axis(2), &[res_chan.view(), res_chan.view(), res_chan.view()]).unwrap()
    }

    fn details_str(&self) -> String {
        "Grayscale".to_string()
    }
}