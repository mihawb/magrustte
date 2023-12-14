use crate::filters::{Filter, Manipulate};
use ndarray::Array3;

pub struct Compose {
    filters: Vec<Filter>,
    pub rerender_index: usize,
}

impl Compose {
    pub fn new(filters_vec: Vec<Filter>) -> Self {
        Self { filters: filters_vec, rerender_index: 0 }
    }

    pub fn add(&mut self, filter: Filter) {
        self.filters.append(&mut vec![filter]);
    }

    pub fn remove(&mut self, index: usize) {
        self.filters.remove(index);
        if index < self.rerender_index {
            self.rerender_index = 0;
        } // no need to rerender if we removed a filter that hasn't been applied yet
    }

    pub fn len(&self) -> usize {
        self.filters.len()
    }
}

impl Manipulate for Compose {
    fn apply(&mut self, img: &Array3<u8>) -> Array3<u8> {

        let mut res = img.clone();
        self.filters.iter_mut().enumerate().for_each(|(i, filter)| {
            if i >= self.rerender_index {
                res = filter.apply(&res);
            }
        });
        self.rerender_index = self.filters.len();
        res
    }

    fn details_str(&self) -> String {
        self.filters
            .iter()
            .enumerate()
            .map(|(i, filter)| format!("{} {}", i, filter.details_str()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}