use std::time::Instant;
use image::GenericImageView;
use ndarray::{Array3, Axis, s, stack};
use fltk::{app::App, frame::Frame, window::Window, image::RgbImage, enums::ColorDepth};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
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



struct TestContainer {
    filter: Filter,
    name: String,
}

fn mainaa() {
    //
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

    let filter_tests: Vec<TestContainer> = vec![
        // TestContainer {
        //     filter: Filter::Threshold(Threshold::new(0.0)),
        //     name: "Threshold".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Invert(Invert::new()),
        //     name: "Invert".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Grayscale(Grayscale::new()),
        //     name: "Grayscale".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Huerotate(Huerotate::new(0.5)),
        //     name: "Huerotate".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Lighting(Lighting::new(0.5, 0.0)),
        //     name: "Lighting_brightness_up".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Lighting(Lighting::new(0.0, -0.5)),
        //     name: "Lighting_contrast_down".to_string(),
        // },
        // TestContainer {
        //     filter: Filter::Lighting(Lighting::new(0.0, 0.5)),
        //     name: "Lighting_contrast_up".to_string(),
        // },
        TestContainer {
            filter: Filter::Vignette(Vignette::new(-0.8, 0.6)),
            name: "Vignette".to_string(),
        }
    ];

    let img: Array3<u8> = AsImage::read("bob.png");
    println!("dimensions {:?}", img.dim());

    // let filter = Filter::Threshold(Threshold::new(0.0));
    // let test_name = "threshold".to_string();
    // let start = Instant::now();
    // let res = filter.apply(&img);
    // println!("{} filter applied in {:?}", test_name, start.elapsed());
    // res.save(&format!("res_{}.png", test_name));

    for test_case in filter_tests {
        let start = Instant::now();
        let res = test_case.filter.apply(&img);
        println!("{} filter applied in {:?}", test_case.name, start.elapsed());
        res.save(&format!("res_{}.png", test_case.name));
    }
}

fn mainbb() {
    let img: Array3<u8> = AsImage::read("pic.jpg");
    let filter = Filter::Blur(Blur::new(-0.8, BlurMode::Gaussian));
    println!("dimensions {:?}", img.dim());
    filter.apply(&img);
}

fn main() {
    let img: Array3<u8> = AsImage::read("pic.jpg");
    println!("dimensions {:?}", img.dim());

    let gaussian = Filter::Blur(Blur::new(1.0, BlurMode::Gaussian));
    let boxb = Filter::Blur(Blur::new(1.0, BlurMode::Box));
    let median = Filter::Blur(Blur::new(1.0, BlurMode::Median));

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

// fn get_patch(img: &Array3<u8>, x: i32, y: i32, r: i32) -> Array3<u8> {
//     let patch = Array3::<u8>::zeros((r as usize * 2 + 1, r as usize * 2 + 1, 3));
//     // iterujac po pixelach w patch
//     // wyciagamy pixel z img jako Option(pxl)
//     // Some(pxl) => patch[i][j] = pxl
//     // None => patch[i][j] = 0 || img[x][y]
//     patch
// }