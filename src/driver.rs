use std::io::{stdin,stdout,Write};
use ndarray::Array3;
use fltk::{app::App, frame::Frame, window::Window, image::RgbImage, enums::ColorDepth, prelude::*};
// use native_dialog::FileDialog;

use crate::imgarray::AsImage;
use crate::filters::{Filter, Manipulate, CommandParse};

use crate::filters::threshold::Threshold;
use crate::filters::invert::Invert;
use crate::filters::grayscale::Grayscale;
use crate::filters::huerotate::Huerotate;
use crate::filters::lighting::Lighting;
use crate::filters::vignette::Vignette;
use crate::filters::blur::{Blur, Mode as BlurMode};
use crate::filters::compose::Compose;

pub struct Context {
    pub path: &'static str,
    pub init_img: Array3<u8>,
    pub res_img: Array3<u8>,
    pub is_img_open: bool,
    pub filters_composed: Compose,
    pub is_running: bool,
}

pub fn get_user_input() -> Vec<String> {
    let mut input = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    input
        .trim().to_string().split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn driver(ctx: &mut Context, command: Vec<String>) {
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
        "vignette" => match Vignette::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Vignette filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        "blur" => match Blur::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Blur filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
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