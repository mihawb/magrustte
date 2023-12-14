pub mod invert;
pub mod threshold;
pub mod compose;
pub mod grayscale;
pub mod huerotate;
pub mod lighting;
pub mod vignette;
pub mod blur;
pub(crate) mod sepia;

use ndarray::Array3;

pub enum Filter {
    Threshold(threshold::Threshold),
    Invert(invert::Invert),
    Compose(compose::Compose),
    Grayscale(grayscale::Grayscale),
    Huerotate(huerotate::Huerotate),
    Lighting(lighting::Lighting),
    Vignette(vignette::Vignette),
    Blur(blur::Blur),
    Sepia(sepia::Sepia),
}
pub trait Manipulate {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8>;
    fn details_str(&self) -> String;
}

impl Manipulate for Filter {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        match self {
            Filter::Threshold(threshold) => threshold.apply(img),
            Filter::Invert(invert) => invert.apply(img),
            Filter::Compose(compose) => compose.apply(img),
            Filter::Grayscale(grayscale) => grayscale.apply(img),
            Filter::Huerotate(huerotate) => huerotate.apply(img),
            Filter::Lighting(lighting) => lighting.apply(img),
            Filter::Vignette(vignette) => vignette.apply(img),
            Filter::Blur(blur) => blur.apply(img),
            Filter::Sepia(sepia) => sepia.apply(img),
        }
    }

    fn details_str(&self) -> String {
        match self {
            Filter::Threshold(threshold) => threshold.details_str(),
            Filter::Invert(invert) => invert.details_str(),
            Filter::Compose(compose) => compose.details_str(),
            Filter::Grayscale(grayscale) => grayscale.details_str(),
            Filter::Huerotate(huerotate) => huerotate.details_str(),
            Filter::Lighting(lighting) => lighting.details_str(),
            Filter::Vignette(vignette) => vignette.details_str(),
            Filter::Blur(blur) => blur.details_str(),
            Filter::Sepia(sepia) => sepia.details_str(),
        }
    }
}

pub trait CommandParse {
    fn parse(command: Vec<String>) -> Result<Filter, Box<dyn std::error::Error>>;
}