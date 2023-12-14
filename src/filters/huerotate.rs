use crate::filters::Manipulate;
use ndarray::{Array3, Axis, stack};
use crate::imgarray::AsImage;

pub struct Huerotate {
    deg: f64,
}

impl Huerotate {
    pub fn new(deg: i32) -> Self {
        Self { deg: deg as f64 }
    }
}

impl Manipulate for Huerotate {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let sin_deg = self.deg.to_radians().sin();
        let cos_deg = self.deg.to_radians().cos();
        let (r, g, b) = AsImage::rgb_as_float(img);

        // https://beesbuzz.biz/code/16-hsv-color-transforms
        let ret_r = &r * (0.299 + 0.701 * cos_deg + 0.168 * sin_deg) +
            &g * (0.587 - 0.587 * cos_deg + 0.330 * sin_deg) +
            &b * (0.114 - 0.114 * cos_deg - 0.497 * sin_deg);
        let ret_g = &r * (0.299 - 0.299 * cos_deg - 0.328 * sin_deg) +
            &g * (0.587 + 0.413 * cos_deg + 0.035 * sin_deg) +
            &b * (0.114 - 0.114 * cos_deg + 0.292 * sin_deg);
        let ret_b = &r * (0.299 - 0.3 * cos_deg + 1.25 * sin_deg) +
            &g * (0.587 - 0.588 * cos_deg - 1.05 * sin_deg) +
            &b * (0.114 + 0.886 * cos_deg - 0.203 * sin_deg);

        stack(Axis(2), &[
            ret_r.map(|x| x.min(255.0).max(0.0).round() as u8).view(),
            ret_g.map(|x| x.min(255.0).max(0.0).round() as u8).view(),
            ret_b.map(|x| x.min(255.0).max(0.0).round() as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Huerotate -> degrees: {}", self.deg)
    }
}