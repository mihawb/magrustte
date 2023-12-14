use std::num::ParseIntError;
use crate::filters::{CommandParse, Filter, Manipulate};
use ndarray::{Array2, Array3, stack, Axis};
use crate::imgarray::AsImage;
use crate::linalg::{gaussian_kernel, outer_product, median};

pub struct Blur {
    radius: i32,
    diameter: i32,
    sigma: f64,
    mode: Mode,
}

#[derive(Debug)]
pub enum Mode {
    Gaussian,
    Box,
    Median,
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gaussian" => Ok(Mode::Gaussian),
            "box" => Ok(Mode::Box),
            "median" => Ok(Mode::Median),
            _ => Err(format!("{} is not a valid blur mode", s)),
        }
    }
}

impl Blur {
    pub fn new(radius: i32, mode: Mode) -> Self {
        Self {
            radius: radius.max(0).min(50),
            diameter: radius.max(0).min(50) * 2 + 1,
            sigma: ((radius.max(0).min(50) as f64) / 2.0).max(1.0),
            mode,
        }
    }

    fn base_blur_channel(channel: &Array2<f64>, kernel: &Array2<f64>, radius: i32) -> Array2<f64> {
        let (width, height) = channel.dim();
        let mut res = Array2::<f64>::zeros((width, height));

        for x in 0..width as i32{
            for y in 0..height as i32 {
                let mut new_val = 0.0;

                for i in -radius..radius {
                    for j in -radius..radius {
                        // capping the values to the image boundaries so as to not make the edges dimmer
                        let x_ = (x + i).max(0).min(width as i32 - 1);
                        let y_ = (y + j).max(0).min(height as i32 - 1);

                        new_val += channel[[x_ as usize, y_ as usize]] * kernel[[(radius + i) as usize, (radius + j) as usize]];
                    }
                }
                res[[x as usize, y as usize]] = new_val
            }
        }
        res
    }

    fn median_blur_channel(channel: &Array2<f64>, kernel: &Array2<f64>, radius: i32) -> Array2<f64> {
        let (width, height) = channel.dim();
        let mut res = Array2::<f64>::zeros((width, height));

        for x in 0..width as i32{
            for y in 0..height as i32 {
                let mut vals = Vec::<f64>::new();

                for i in -radius..radius {
                    for j in -radius..radius {
                        let x_ = (x + i).max(0).min(width as i32 - 1);
                        let y_ = (y + j).max(0).min(height as i32 - 1);

                        vals.push(channel[[x_ as usize, y_ as usize]]);
                    }
                }
                res[[x as usize, y as usize]] = median(&mut vals);
            }
        }
        res
    }
}

impl Manipulate for Blur {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (rc , gc, bc) = img.rgb_as_float();
        let kernel = match self.mode {
            Mode::Gaussian => outer_product(
                &gaussian_kernel(self.diameter, self.sigma),
                &gaussian_kernel(self.diameter, self.sigma),
            ),
            Mode::Box => Array2::<f64>::ones((self.diameter as usize, self.diameter as usize)) / (self.diameter * self.diameter) as f64,
            Mode::Median => Array2::<f64>::ones((self.diameter as usize, self.diameter as usize)),
        };

        let blur_fn = match self.mode {
            Mode::Gaussian => Self::base_blur_channel,
            Mode::Box => Self::base_blur_channel,
            Mode::Median => Self::median_blur_channel,
        };

        stack(Axis(2), &[
            blur_fn(&rc, &kernel, self.radius).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            blur_fn(&gc, &kernel, self.radius).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            blur_fn(&bc, &kernel, self.radius).mapv(|x| x.min(255.0).max(0.0) as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Blur -> radius: {}, mode: {:?}", self.radius, self.mode)
    }
}

impl CommandParse for Blur {
    fn parse(command: Vec<String>) -> Result<Filter, Box<dyn std::error::Error>> {
        let maybe_radius = match command.get(0) {
            Some(s) => s,
            None => "nan",
        };
        let maybe_mode = match command.get(1) {
            Some(s) => s,
            None => "nam",
        };
        let radius = maybe_radius.parse::<i32>()?;
        let mode = maybe_mode.parse::<Mode>()?;
        Ok(Filter::Blur(Blur::new(radius, mode)))
    }
}