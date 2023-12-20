use ndarray::{Array2, Array3, stack, Axis};
use crate::filters::{CommandParse, Filter, Manipulate};
use crate::imgarray::AsImage;
use crate::linalg::{gaussian_kernel, outer_product};

pub struct Bilateral {
    radius: i32,
    diameter: i32,
    spatial_sigma: f64,
    intensity_sigma: f64,
}

impl Bilateral {
    pub fn new(radius: i32, spatial_sigma: f64, intensity_sigma: f64) -> Self {
        Self {
            radius: radius.max(0).min(50),
            diameter: radius.max(0).min(50) * 2 + 1,
            spatial_sigma: spatial_sigma.max(0.1).min(50.0),
            intensity_sigma: intensity_sigma.max(0.1).min(50.0),
        }
    }

    fn bilateral_channel(channel: &Array2<f64>, kernel: &Array2<f64>, radius: i32, intensity_sigma: f64) -> Array2<f64> {
        let (width, height) = channel.dim();
        let mut res = Array2::<f64>::zeros((width, height));

        for x in 0..width as i32{
            for y in 0..height as i32 {
                let mut new_val = 0.0;
                let mut weight_sum = 0.0;

                for i in -radius..radius {
                    for j in -radius..radius {
                        // capping the values to the image boundaries so as to not make the edges dimmer
                        let x_ = (x + i).max(0).min(width as i32 - 1);
                        let y_ = (y + j).max(0).min(height as i32 - 1);

                        // hallucinated by copilot and doesn't work
                        // try to copy this https://python.algorithmexamples.com/web/digital_image_processing/filters/bilateral_filter.html
                        let spatial_weight = kernel[[(radius + i) as usize, (radius + j) as usize]];
                        let intensity_weight = (-((channel[[x_ as usize, y_ as usize]] - channel[[x as usize, y as usize]]).powi(2)) / (2.0 * intensity_sigma.powi(2))).exp();

                        new_val += channel[[x_ as usize, y_ as usize]] * spatial_weight * intensity_weight;
                        weight_sum += spatial_weight * intensity_weight;
                    }
                }
                res[[x as usize, y as usize]] = new_val / weight_sum;
            }
        }
        res
    }
}

impl Manipulate for Bilateral {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {
        let (rc , gc, bc) = img.rgb_as_float();

        let kernel = outer_product(
            &gaussian_kernel(self.diameter, self.spatial_sigma),
            &gaussian_kernel(self.diameter, self.spatial_sigma),
        );

        stack(Axis(2), &[
            Self::bilateral_channel(&rc, &kernel, self.radius, self.intensity_sigma)
                .mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            Self::bilateral_channel(&gc, &kernel, self.radius, self.intensity_sigma)
                .mapv(|x| x.min(255.0).max(0.0) as u8).view(),
            Self::bilateral_channel(&bc, &kernel, self.radius, self.intensity_sigma)
                .mapv(|x| x.min(255.0).max(0.0) as u8).view(),
        ]).unwrap()
    }

    fn details_str(&self) -> String {
        format!("Bilateral filter with radius {}, spatial sigma {}, intensity sigma {}",
            self.radius, self.spatial_sigma, self.intensity_sigma)
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
        let maybe_intensity_sigma = match command.get(2) {
            Some(s) => s,
            None => "nan",
        };

        let radius = maybe_radius.parse::<i32>()?;
        let spatial_sigma = maybe_spatial_sigma.parse::<f64>()?;
        let intensity_sigma = maybe_intensity_sigma.parse::<f64>()?;

        Ok(Filter::Bilateral(Bilateral::new(radius, spatial_sigma, intensity_sigma)))
    }
}