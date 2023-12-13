use ndarray::{Array1, Array2};
use std::cmp::Ordering;

pub fn  gaussian(x: f64, mu: f64, sigma: f64) -> f64 {
    let a = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());
    let b = -0.5 * ((x - mu) / sigma).powi(2);
    a * b.exp()
}

pub fn gaussian_kernel(size: i32, sigma: f64) -> Array1<f64> {
    let mu = size / 2;
    let mut kernel = Array1::<f64>::linspace(0.0, size as f64, size as usize);
    kernel = kernel.mapv(|x| gaussian(x, mu as f64, sigma));
    &kernel / kernel.sum()
}

// necessary because ndarray doesn't support matrix multiplication between a row vector and a column vector
// i.e. it "does" but returns a dot product instead of an outer product
pub fn outer_product(x: &Array1<f64>, y: &Array1<f64>) -> Array2<f64> {
    let (size_x, size_y) = (x.shape()[0], y.shape()[0]);
    let x_reshaped = x.view().into_shape((size_x, 1)).unwrap();
    let y_reshaped = y.view().into_shape((1, size_y)).unwrap();
    x_reshaped.dot(&y_reshaped)
}

pub fn median(numbers: &mut Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
}