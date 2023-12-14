pub mod imgarray;
pub mod filters;
pub mod linalg;
pub mod driver;

use std::path::PathBuf;
use ndarray::Array3;

use crate::driver::{Context, get_user_input, driver};
use crate::filters::compose::Compose;

fn main() {
    let mut ctx = Context {
        path: PathBuf::from(""),
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