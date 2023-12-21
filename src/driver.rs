use std::io::{stdin,stdout,Write};
use std::path::{PathBuf, Path};
use ndarray::Array3;
use fltk::{app::App, frame::Frame, window::Window, image::RgbImage, enums::ColorDepth, prelude::*};
use native_dialog::FileDialog;

use crate::imgarray::AsImage;
use crate::filters::{Filter, Manipulate, CommandParse};

use crate::filters::sepia::Sepia;
use crate::filters::invert::Invert;
use crate::filters::grayscale::Grayscale;
use crate::filters::threshold::Threshold;
use crate::filters::vignette::Vignette;
use crate::filters::huerotate::Huerotate;
use crate::filters::sharpen::Sharpen;
use crate::filters::lighting::Lighting;
use crate::filters::blur::Blur;
use crate::filters::bilateral::Bilateral;
use crate::filters::compose::Compose;

pub struct Context {
    pub path: PathBuf,
    pub init_img: Array3<u8>,
    pub res_img: Array3<u8>,
    pub is_img_open: bool,
    pub filters_composed: Compose,
    pub is_running: bool,
}

impl Context {
    pub fn clear(&mut self) {
        self.path = PathBuf::from("");
        self.init_img = Array3::<u8>::zeros((1, 1, 3));
        self.res_img = Array3::<u8>::zeros((1, 1, 3));
        self.is_img_open = false;
        self.filters_composed = Compose::new(vec![]);
        self.is_running = true;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            path: PathBuf::from(""),
            init_img: Array3::<u8>::zeros((1, 1, 3)),
            res_img: Array3::<u8>::zeros((1, 1, 3)),
            is_img_open: false,
            filters_composed: Compose::new(vec![]),
            is_running: true,
        }
    }
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
        "open" => {
            if ctx.is_img_open {
                println!("Image already loaded, close it first by typing 'close'.");
            }
            ctx.path = if command.len() > 1 {
                PathBuf::from(command[1].as_str())
            } else {
                match handle_file_dialog() {
                    Ok(path) => path,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }
            };
            match Array3::read(ctx.path.clone().into_os_string().to_str().unwrap()) {
                Ok(img) => {
                    ctx.is_img_open = true;
                    println!("Image loaded.");
                    ctx.init_img = img;
                }
                Err(_) => println!("Unable to open image: {}", command[1].as_str()),
            }

        }
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
                render_image(ctx);
                show_img(&ctx.res_img);
            } else {
                println!("No image loaded.");
            }
        },
        "close" => {
            if ctx.is_img_open {
                ctx.clear();
                println!("Image closed.");
            } else {
                println!("No image loaded.");
            }
        },
        "save" => {
            if !ctx.is_img_open {
                println!("No image loaded.");
                return;
            }
            if command.len() < 2 {
                println!("Wrong number of arguments. Type 'help' to see available commands.");
                return;
            }
            render_image(ctx);
            let dest = Path::join(ctx.path.parent().unwrap(), command[1].as_str())
                .into_os_string().to_str().unwrap().to_string();
            ctx.res_img.save(&dest);
            println!("Image saved at {}.", dest);
        },
        "exit" => {
            ctx.is_running = false;
        },
        "help" => {
            println!("Available commands:");
            println!("open - open image");
            println!("add <filter> <*params> - add filter to image");
            println!("remove <index> - remove filter from image by index");
            println!("list - list all filters");
            println!("show - show image");
            println!("close - close image");
            println!("save <filename> - save image");
            println!("exit - exit program");
            println!("help - show this message");
            println!("\nAvailable filters:");
            println!("sepia");
            println!("invert");
            println!("grayscale");
            println!("threshold <value>");
            println!("vignette <radius>");
            println!("huerotate <degrees>");
            println!("sharpen <gaussian/box/median>");
            println!("lighting <brightness> <contrast>");
            println!("blur <radius> <gaussian/box/median>");
            println!("bilateral <radius> <spatial sigma> <intensity sigma>");
        },
        _ => println!("Unknown command. Type 'help' to see available commands."),
    }
}

fn handle_file_dialog() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let pwd = std::env::current_dir()?;
    match FileDialog::new()
        .set_location(&pwd)
        .add_filter("Image (png, jpg)", &["png", "jpg", "jpeg"])
        .show_open_single_file() {
            Ok(op) => match op {
                Some(p) => Ok(p),
                None => Err(Box::try_from("No file was chosen".to_string()).unwrap()),
            },
            Err(e) => Err(Box::try_from(e.to_string()).unwrap()),
        }
}

fn handle_add(ctx: &mut Context, command: Vec<String>) {
    // TODO: impl CommandParse for Filter to avoid TERRIBLE code duplication
    // DRY code not feasible atm since dyn CommandParse structs would have to be passed as params
    match command[0].as_ref() {
        "sepia" => {
            ctx.filters_composed.add(Filter::Sepia(Sepia::new()));
            println!("Sepia filter added.");
        },
        "invert" => {
            ctx.filters_composed.add(Filter::Invert(Invert::new()));
            println!("Invert filter added.");
        },
        "grayscale" => {
            ctx.filters_composed.add(Filter::Grayscale(Grayscale::new()));
            println!("Grayscale filter added.");
        },
        "threshold" => match Threshold::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Threshold filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        "vignette" => match Vignette::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Vignette filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        "huerotate" => match Huerotate::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Huerotate filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        "sharpen" => match Sharpen::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Sharpen filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        "lighting" => match Lighting::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Lighting filter added.");
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
        "bilateral" => match Bilateral::parse(command[1..].to_vec()) {
            Ok(filter) => {
                ctx.filters_composed.add(filter);
                println!("Bilateral filter added.");
            },
            Err(_) => println!("Wrong arguments. Type 'help' to see available commands."),
        },
        _ => println!("Unknown filter. Type 'help' to see available filters."),
    }
}

fn render_image(ctx: &mut Context) {
    println!("Rendering image...");
    // no-op equivalent if no new filters were added
    // will apply only the filters that were added since last show
    match ctx.filters_composed.rerender_index {
        0 => ctx.res_img = ctx.filters_composed.apply(&ctx.init_img),
        _ => ctx.res_img = ctx.filters_composed.apply(&ctx.res_img),
    };
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