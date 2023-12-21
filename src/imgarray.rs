use ndarray::{Array2, Array3, s};
use image::{ImageBuffer, GenericImageView, Rgb, RgbImage};

pub trait AsImage {
    fn save(&self, path: &str);
    fn read(path: &str) -> Result<Array3<u8>, Box<dyn std::error::Error>>;
    fn to_rgb_image(&self) -> RgbImage;
    fn rgb_as_float(&self) -> (Array2<f64>, Array2<f64>, Array2<f64>);
    fn split_channels(&self) -> (Array2<u8>, Array2<u8>, Array2<u8>);
}

impl AsImage for Array3<u8> {
    fn save(&self, path: &str) {
        self.to_rgb_image().save(path).unwrap();
    }

    fn read(path: &str) -> Result<Array3<u8>, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let mut res = Array3::<u8>::zeros((width as usize, height as usize, 3));
        for (x, y, pixel) in img.pixels() {
            res[[x as usize, y as usize, 0]] = pixel[0];
            res[[x as usize, y as usize, 1]] = pixel[1];
            res[[x as usize, y as usize, 2]] = pixel[2];
        }
        Ok(res)
    }

    fn to_rgb_image(&self) -> RgbImage {
        let (width, height, _) = self.dim();
        let mut copy = self.clone();
        copy.swap_axes(0, 1);
        let raw = copy.as_standard_layout().to_owned().into_raw_vec();
        ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width as u32, height as u32, raw).unwrap()
    }

    fn rgb_as_float(&self) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
        (
            self.slice(s![..,..,0]).to_owned().mapv(|x| x as f64),
            self.slice(s![..,..,1]).to_owned().mapv(|x| x as f64),
            self.slice(s![..,..,2]).to_owned().mapv(|x| x as f64),
        )
    }

    fn split_channels(&self) -> (Array2<u8>, Array2<u8>, Array2<u8>) {
        (
            self.slice(s![..,..,0]).to_owned(), // red
            self.slice(s![..,..,1]).to_owned(), // green
            self.slice(s![..,..,2]).to_owned(), // blue
        )
    }
}