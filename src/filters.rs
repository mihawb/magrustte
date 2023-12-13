pub mod invert;
pub mod threshold;
pub mod compose;
pub mod grayscale;
pub mod huerotate;
pub mod lighting;
pub mod vignette;
pub mod blur;

use ndarray::Array3;

// ZALOZENIA
// 1. wszystkie parametry sa typu f64 i zawieraja sie w przedziale [-1.0, 1.0]
pub enum Filter {
    Threshold(threshold::Threshold),
    Invert(invert::Invert),
    Compose(compose::Compose),
    Grayscale(grayscale::Grayscale),
    Huerotate(huerotate::Huerotate),
    Lighting(lighting::Lighting),
    Vignette(vignette::Vignette),
    Blur(blur::Blur),
}
pub trait Manipulate {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8>;
}

impl Manipulate for Filter {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8> {
        match self {
            Filter::Threshold(threshold) => threshold.apply(img),
            Filter::Invert(invert) => invert.apply(img),
            Filter::Compose(compose) => compose.apply(img),
            Filter::Grayscale(grayscale) => grayscale.apply(img),
            Filter::Huerotate(huerotate) => huerotate.apply(img),
            Filter::Lighting(lighting) => lighting.apply(img),
            Filter::Vignette(vignette) => vignette.apply(img),
            Filter::Blur(blur) => blur.apply(img),
        }
    }
}