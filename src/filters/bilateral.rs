use ndarray::{Array2, Array3, stack, Axis};
use crate::filters::{CommandParse, Filter, Manipulate};
use crate::imgarray::AsImage;
use crate::linalg::{gaussian_kernel, outer_product, gaussian, array_slice};

pub struct Bilateral {
    radius: i32,
    diameter: i32,
    spatial_sigma: f64,
    color_sigma: f64,
}

impl Bilateral {
    pub fn new(radius: i32, spatial_sigma: f64, intensity_sigma: f64) -> Self {
        Self {
            radius: radius.max(0).min(50),
            diameter: radius.max(0).min(50) * 2 + 1,
            spatial_sigma: spatial_sigma.max(0.1).min(50.0),
            color_sigma: intensity_sigma.max(0.1).min(50.0),
        }
    }

    // https://python.algorithmexamples.com/web/digital_image_processing/filters/bilateral_filter.html
    fn bilateral_channel(&self, channel: &Array2<f64>) -> Array2<f64> {
        let (width, height) = channel.dim();
        let mut res = Array2::<f64>::zeros((width, height));
        let spatial_kernel = outer_product(
            &gaussian_kernel(self.diameter, self.spatial_sigma),
            &gaussian_kernel(self.diameter, self.spatial_sigma));

        for x in 0..width {
            for y in 0..height {
                let img_s = array_slice(channel, x as i32, y as i32, self.radius);
                let img_i = &img_s - channel[[x, y]];
                let img_ig = img_i.mapv(|x| gaussian(x, 0.0, self.color_sigma));
                let weights = &spatial_kernel * &img_ig;
                let vals = &img_s * &weights;
                let val = vals.sum() / weights.sum();
                res[[x, y]] = val;
            }
        }
        res
    }
}

impl Manipulate for Bilateral {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (rc , gc, bc) = img.rgb_as_float();

        stack(Axis(2), &[
            self.bilateral_channel(&(rc / 255.0))
                .mapv(|x| (x * 255.0).min(255.0).max(0.0) as u8).view(),
            self.bilateral_channel(&(gc / 255.0))
                .mapv(|x| (x * 255.0).min(255.0).max(0.0) as u8).view(),
              self.bilateral_channel(&(bc / 255.0))
                  .mapv(|x| (x * 255.0).min(255.0).max(0.0) as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Bilateral filter -> radius: {}, spatial sigma: {}, color sigma: {}",
            self.radius, self.spatial_sigma, self.color_sigma)
    }
}

impl CommandParse for Bilateral {
    fn parse(command: Vec<String>) -> Result<Filter, Box<dyn std::error::Error>> {
        let maybe_radius = match command.get(0) {
            Some(s) => s,
            None => "nan",
        };
        let maybe_spatial_sigma = match command.get(1) {
            Some(s) => s,
            None => "nan",
        };
        let maybe_color_sigma = match command.get(2) {
            Some(s) => s,
            None => "nan",
        };

        let radius = maybe_radius.parse::<i32>()?;
        let spatial_sigma = maybe_spatial_sigma.parse::<f64>()?;
        let color_sigma = maybe_color_sigma.parse::<f64>()?;

        Ok(Filter::Bilateral(Bilateral::new(radius, spatial_sigma, color_sigma)))
    }
}