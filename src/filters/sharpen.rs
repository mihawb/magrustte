use ndarray::Array3;
use crate::filters::{CommandParse, Filter, Manipulate, blur::{Blur, BlurMode}, bilateral::Bilateral};

pub struct Sharpen {
    mode: SharpenMode,
    coarse_radius: i32,
    render_fine_mask: bool,
}

#[derive(Debug)]
pub enum SharpenMode {
    Gaussian,
    Box,
    Median,
    Bilateral,
}

impl std::str::FromStr for SharpenMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gaussian" => Ok(SharpenMode::Gaussian),
            "box" => Ok(SharpenMode::Box),
            "median" => Ok(SharpenMode::Median),
            "bilateral" => Ok(SharpenMode::Bilateral),
            _ => Err(format!("{} is not a valid sharpening mode", s)),
        }
    }
}

impl Sharpen {
    pub fn new(mode: SharpenMode, coarse_radius: i32, render_fine_mask: bool) -> Self {
        // TODO: idk if all those params have to be user defined
        Self { mode, coarse_radius, render_fine_mask }
    }
}

// on sharpening: https://web.stanford.edu/class/cs448f/lectures/2.1/Sharpening.pdf
impl Manipulate for Sharpen {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let coarse = match self.mode {
            SharpenMode::Gaussian => Blur::new(self.coarse_radius, BlurMode::Gaussian).apply(img),
            SharpenMode::Box => Blur::new(self.coarse_radius, BlurMode::Box).apply(img),
            SharpenMode::Median => Blur::new(self.coarse_radius, BlurMode::Median).apply(img),
            SharpenMode::Bilateral =>
                Bilateral::new(self.coarse_radius, self.coarse_radius as f64, 0.05).apply(img),
        };

        // fine = img - coarse (mapped to i32 to avoid underflow)
        let fine = img.mapv(|x| x as i32) - coarse.mapv(|x| x as i32);

        if self.render_fine_mask {
            fine.mapv(|x| x.max(0).min(255) as u8)
        } else {
            (img.mapv(|x| x as i32) + &fine / 2)
                .mapv(|x| x.max(0).min(255) as u8)
        }
    }

    fn details_str(&self) -> String {
        format!("Sharpen -> mode: {:?}", self.mode)
    }
}

impl CommandParse for Sharpen {
    fn parse(command: Vec<String>) -> Result<Filter, Box<dyn std::error::Error>> {
        let maybe_mode = match command.get(0) {
            Some(s) => s,
            None => "nam",
        };
        let maybe_coarse_radius = match command.get(1) {
            Some(s) => s,
            None => "nan",
        };
        let maybe_render_fine_mask = match command.get(2) {
            Some(s) => s,
            None => "nab",
        };
        let mode = maybe_mode.parse::<SharpenMode>()?;
        let coarse_radius = maybe_coarse_radius.parse::<i32>()?;
        let render_fine_mask = maybe_render_fine_mask.parse::<bool>()?;
        Ok(Filter::Sharpen(Sharpen::new(mode, coarse_radius, render_fine_mask)))
    }
}