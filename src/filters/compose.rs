use crate::filters::{Filter, Manipulate};
use ndarray::Array3;

pub struct Compose {
    filters: Vec<Filter>,
}

impl Compose {
    pub fn new(filters_vec: Vec<Filter>) -> Self {
        Self { filters: filters_vec }
    }

    pub fn attach(self, filter: Filter) -> Compose {
        Compose::new(self.filters.into_iter().chain(vec![filter]).collect())
    }
}

impl Manipulate for Compose {
    fn apply(&self, img: &Array3<u8>) -> Array3<u8> {
        let mut res = img.clone();
        self.filters.iter().for_each(|filter| {
            res = filter.apply(&res);
        });
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