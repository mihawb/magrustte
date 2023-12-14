use crate::filters::Manipulate;
use ndarray::{Array3, stack, Axis};
use ndarray_stats::QuantileExt;
use crate::imgarray::AsImage;
use crate::linalg::{gaussian_kernel, outer_product};

pub struct Vignette {
    radius: f64,
    opacity: f64,
}

impl Vignette {
    pub fn new(radius: i32, opacity: i32) -> Self {
        Self {
            radius: radius.max(0).min(100) as f64 / 100.0,
            opacity: opacity.max(0).min(100) as f64 / 100.0,
        }
    }
}

impl Manipulate for Vignette {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (rc , gc, bc) = img.rgb_as_float();

        let (width, height, _) = img.dim();
        let a = gaussian_kernel(width as i32, width as f64 * self.radius);
        let b = gaussian_kernel(height as i32, height as f64 * self.radius);
        let c = outer_product(&a, &b);
        let d = &c / *c.max().unwrap();
        let e = d.mapv(|x| (x + 1.0 - self.opacity).min(1.0).max(0.0));

        stack(Axis(2), &[
            (rc * &e).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            (gc * &e).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            (bc * &e).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Vignette -> radius: {}%, opacity: {}%", self.radius * 100.0, self.opacity * 100.0)
    }
}