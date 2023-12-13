use crate::filters::Manipulate;
use ndarray::Array3;

pub struct Lighting {
    brightness: f64,
    contrast: f64,
}

impl Lighting {
    pub fn new(brightness: i32, contrast: i32) -> Self {
        Self {
            brightness: brightness.max(-255).min(255) as f64,
            contrast: contrast.max(-255).min(255) as f64,
        }
    }
}

impl Manipulate for Lighting {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8> {
        // on brightness https://math.stackexchange.com/a/906280
        // on contrast https://www.dfstudios.co.uk/articles/programming/image-programming-algorithms/image-processing-algorithms-part-5-contrast-adjustment/
        let f = 259.0 * (self.contrast + 255.0) / (255.0 * (259.0 - self.contrast));

        img.mapv(|x| {
            let c = f * (x as f64 - 128.0) + 128.0 + self.brightness;
            c.min(255.0).max(0.0).round() as u8
        })
    }
}