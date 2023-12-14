pub mod imgarray;
pub mod filters;
pub mod linalg;
pub mod driver;

use ndarray::Array3;

use crate::driver::{Context, get_user_input, driver};
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