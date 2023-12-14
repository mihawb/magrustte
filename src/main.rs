pub mod imgarray;
pub mod filters;
pub mod linalg;
pub mod driver;

use crate::driver::{Context, get_user_input, driver};

fn main() {
    let mut ctx = Context::default();

    println!("Welcome to Magrustte!");
    println!("Type 'help' to see available commands.");
    while ctx.is_running {
        let command = get_user_input();
        driver(&mut ctx, command);
    }
}