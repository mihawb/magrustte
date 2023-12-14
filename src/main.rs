use std::time::Instant;
use std::io::{stdin,stdout,Write};
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

struct Context {
    path: &'static str,
    init_img: Array3<u8>,
    res_img: Array3<u8>,
    is_img_open: bool,
    filters_composed: Compose,
    is_running: bool,
}

fn main() {
    let mut ctx = Context {
        path: "",
        init_img: Array3::<u8>::zeros((1, 1, 3)),
        res_img: Array3::<u8>::zeros((1, 1, 3)),
        is_img_open: false,
        filters_composed: Compose::new(vec![]),
        is_running: true,
    };

    println!("Welcome to Magrustte!");
    println!("Type 'help' to see available commands.");
    while ctx.is_running {
        let command = get_user_input();
        driver(&mut ctx, command);
    }
}

fn get_user_input() -> Vec<String> {
    let mut input = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    input
        .trim().to_string().split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn driver(ctx: &mut Context, command: Vec<String>) {
    match command[0].as_ref() {
        "open-debug" => {
            if ctx.is_img_open {
                println!("Image already loaded. Type 'reset' to close it.");
            } else {
                ctx.path = "pic.jpg";
                ctx.init_img = AsImage::read(ctx.path);
                ctx.is_img_open = true;
            }
            println!("Image loaded.");
        },
        "add" => {
            if command.len() < 2 {
                println!("Wrong number of arguments. Type 'help' to see available commands.");
            }
            else if ctx.is_img_open {
                handle_add(ctx, command[1..].to_vec());
            } else {
                println!("No image loaded.");
            }
        },
        "remove" => {
            if command.len() < 2 {
                println!("Wrong number of arguments. Type 'help' to see available commands.");
            }
            else if ctx.is_img_open {
                match command[1].parse::<usize>() {
                    Ok(index) => {
                        if index < ctx.filters_composed.len() {
                            ctx.filters_composed.remove(index);
                            println!("Filter at index {} removed.", index);
                        } else {
                            println!("Index out of bounds.");
                        }
                    },
                    Err(_) => println!("Wrong argument. Type 'help' to see available commands."),
                }
            } else {
                println!("No image loaded.");
            }
        }
        "list" => {
            if ctx.is_img_open {
                println!("{}", ctx.filters_composed.details_str());
            } else {
                println!("No image loaded.");
            }
        },
        "show" => {
            if ctx.is_img_open {
                println!("Rendering image...");
                // no-op equivalent if no new filters were added
                // will apply only the filters that were added since last show
                match ctx.filters_composed.rerender_index {
                    0 => ctx.res_img = ctx.filters_composed.apply(&ctx.init_img),
                    _ => ctx.res_img = ctx.filters_composed.apply(&ctx.res_img),
                }
                show_img(&ctx.res_img);
            } else {
                println!("No image loaded.");
            }
        },
        "close" => {
            if ctx.is_img_open {
                ctx.init_img = Array3::<u8>::zeros((1, 1, 3));
                ctx.res_img = Array3::<u8>::zeros((1, 1, 3));
                ctx.is_img_open = false;
                println!("Image closed.");
            } else {
                println!("No image loaded.");
            }
        },
        "exit" => {
            ctx.is_running = false;
        },
        "help" => {
            println!("Available commands:");
            println!("X open - open image");
            println!("X add <filter> <*params> - add filter to image");
            println!("remove <index> - remove filter from image by index");
            println!("list - list all filters");
            println!("show - show image");
            println!("close - close image");
            println!("X save - save image");
            println!("exit - exit program");
            println!("help - show this message");
            println!("\nAvailable filters:");
            println!("threshold <value>");
            println!("invert");
            println!("grayscale");
            println!("huerotate <degrees>");
            println!("lighting <brightness> <contrast>");
            println!("vignette <radius>");
            println!("blur <radius> <gaussian/box/median>");
        },
        _ => println!("Unknown command. Type 'help' to see available commands."),
    }
}

fn handle_add(ctx: &mut Context, command: Vec<String>) {
    match command[0].as_ref() {
        "invert" => {
            ctx.filters_composed.add(Filter::Invert(Invert::new()));
            println!("Invert filter added.");
        },
        "grayscale" => {
            ctx.filters_composed.add(Filter::Grayscale(Grayscale::new()));
            println!("Grayscale filter added.");
        },
        _ => println!("Unknown filter. Type 'help' to see available filters."),
    }
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

// fn main_test() {
//     let img: Array3<u8> = AsImage::read("pic.jpg");
//     println!("dimensions {:?}", img.dim());
//
//     let gaussian = Filter::Blur(Blur::new(5, BlurMode::Gaussian));
//     let boxb = Filter::Blur(Blur::new(5, BlurMode::Box));
//     let median = Filter::Blur(Blur::new(5, BlurMode::Median));
//
//     let mut start = Instant::now();
//     gaussian.apply(&img).save("gaussian.png");
//     println!("Gaussian filter applied in {:?}", start.elapsed());
//
//     start = Instant::now();
//     boxb.apply(&img).save("box.png");
//     println!("Box filter applied in {:?}", start.elapsed());
//
//     start = Instant::now();
//     median.apply(&img).save("median.png");
//     println!("Median filter applied in {:?}", start.elapsed());
// }