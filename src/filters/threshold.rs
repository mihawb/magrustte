use crate::filters::{CommandParse, Filter, Manipulate};
use ndarray::{Array3, stack, Axis};
use crate::imgarray::AsImage;

pub struct Threshold {
    threshold: u8,
}

impl Threshold {
    pub fn new(threshold: i32) -> Self {
        Self { threshold: threshold.max(0).min(255) as u8  }
    }
}

impl Manipulate for Threshold {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (r, g, b) = img.split_channels();
        let r_f = r.mapv(|x| x as f64);
        let g_f = g.mapv(|x| x as f64);
        let b_f = b.mapv(|x| x as f64);
        let luma = r_f * 0.2126 + g_f * 0.7152 + b_f * 0.0722; // https://en.wikipedia.org/wiki/Grayscale#Colorimetric_(perceptual_luminance-preserving)_conversion_to_grayscale
        let effect = luma.mapv(|x| if x as u8 > self.threshold { 255 } else { 0 });
        stack(Axis(2), &[effect.view(), effect.view(), effect.view()]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Threshold -> threshold: {}", self.threshold)
    }
}

impl CommandParse for Threshold {
    fn parse(command: Vec<String>) -> Result<Filter, Box<dyn std::error::Error>> {
        let maybe_threshold = match command.get(0) {
            Some(s) => s,
            None => "nan",
        };
        let threshold = maybe_threshold.parse::<i32>()?;
        Ok(Filter::Threshold(Threshold::new(threshold)))
    }
}