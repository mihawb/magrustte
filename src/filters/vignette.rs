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
    pub fn new(radius: f64, opacity: f64) -> Self {
        Self {
            radius: radius.max(-1.0).min(1.0) * 0.5 + 0.5,
            opacity: opacity.max(-1.0).min(1.0) * 0.5 + 0.5
        }
    }
}

impl Manipulate for Vignette {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8> {
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
}