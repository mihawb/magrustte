use std::time::Instant;
use ndarray::Array3;
use fltk::{app::App, frame::Frame, window::Window, image::RgbImage, enums::ColorDepth, prelude::*};
// use native_dialog::FileDialog;

pub mod imgarray;
pub mod filters;
pub mod linalg;

use crate::imgarray::*;
use crate::filters::*;

use crate::filters::threshold::Threshold;
use crate::filters::invert::Invert;
use crate::filters::grayscale::Grayscale;
use crate::filters::huerotate::Huerotate;
use crate::filters::lighting::Lighting;
use crate::filters::vignette::Vignette;
use crate::filters::blur::{Blur, Mode as BlurMode};
use crate::filters::compose::Compose;

// let image_path = match FileDialog::new()
//     .set_location("~")
//     .add_filter("Image (png, jpg, heic)", &["png", "jpg", "heic"])
//     .show_open_single_file() {
//     Ok(op) => match op {
//         Some(p) => p,
//         None => panic!("PathBuf option is empty (file was not chosen)"),
//     },
//     Err(e) => panic!("Problem with file dialog: {}", e),
// };

fn main() {
    let img: Array3<u8> = AsImage::read("pic.jpg");
    println!("dimensions {:?}", img.dim());

    let gaussian = Filter::Blur(Blur::new(5, BlurMode::Gaussian));
    let boxb = Filter::Blur(Blur::new(5, BlurMode::Box));
    let median = Filter::Blur(Blur::new(5, BlurMode::Median));

    let mut start = Instant::now();
    gaussian.apply(&img).save("gaussian.png");
    println!("Gaussian filter applied in {:?}", start.elapsed());

    start = Instant::now();
    boxb.apply(&img).save("box.png");
    println!("Box filter applied in {:?}", start.elapsed());

    start = Instant::now();
    median.apply(&img).save("median.png");
    println!("Median filter applied in {:?}", start.elapsed());
}

fn show_img(img: &Array3<u8>) {
    let app = App::default();
    let mut wind = Window::new(100, 100, img.dim().0 as i32, img.dim().1 as i32, "Magrustte");
    let mut frame = Frame::new(0, 0, img.dim().0 as i32, img.dim().1 as i32, "");
    frame.set_image(Some(RgbImage::new(
        &img.to_rgb_image(),
        img.dim().0 as i32, img.dim().1 as i32, ColorDepth::Rgb8).unwrap()));
    wind.end();
    wind.show();
    app.run().unwrap();
}