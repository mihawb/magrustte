use crate::filters::Manipulate;
use crate::imgarray::AsImage;
use ndarray::{Array2, Array3, stack, Axis};

#[derive(Default)]
pub struct Grayscale;

impl Grayscale {
    pub fn new() -> Self { Self }
}

impl Manipulate for Grayscale {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (r, g, b) = img.split_channels();
        let r_f = r.mapv(|x| x as f64);
        let g_f = g.mapv(|x| x as f64);
        let b_f = b.mapv(|x| x as f64);
        let luma: Array2<f64> = r_f * 0.2126 + g_f * 0.7152 + b_f * 0.0722; // https://en.wikipedia.org/wiki/Grayscale#Colorimetric_(perceptual_luminance-preserving)_conversion_to_grayscale
        let res_chan = luma.mapv(|x| x as u8);
        stack(Axis(2), &[res_chan.view(), res_chan.view(), res_chan.view()]).unwrap()
    }

    fn details_str(&self) -> String {
        "Grayscale".to_string()
    }
}