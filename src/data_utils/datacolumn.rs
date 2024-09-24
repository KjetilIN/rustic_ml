use std::{any::type_name, slice::Iter};

#[allow(dead_code)]
pub struct DataColumn<T> {
    data: Vec<Option<T>>,
    pub data_type: &'static str,
}

impl<T: Default> DataColumn<T> {
    pub fn new(data: Vec<Option<T>>) -> Self {
        Self {
            data,
            data_type: type_name::<T>(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index].as_ref()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn none_count(&self) -> usize {
        self.data.len() - self.data.iter().flatten().count()
    }

    pub fn some_count(&self) -> usize {
        self.data.iter().flatten().count()
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index < self.data.len() {
            self.data[index] = Some(item);
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.data.len() {
            self.data[index] = None;
        }
    }

    pub fn append(&mut self, item: T) {
        self.data.push(Some(item));
    }

    pub fn reset(&mut self) {
        self.data = (0..self.data.len()).map(|_| None).collect();
    }

    pub fn reset_default(&mut self) {
        self.data = (0..self.data.len()).map(|_| Some(T::default())).collect();
    }

    pub fn iter_column(&self) -> Iter<Option<T>> {
        self.data.iter()
    }
}
