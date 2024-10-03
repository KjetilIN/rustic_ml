use std::{
    any::{type_name, Any},
    slice::Iter,
};

// Define a trait for DataColumn
pub trait DataColumnTrait {
    type Item; // Associated type to represent the type of data (T)

    fn new(data: Vec<Option<Self::Item>>, name: String) -> Self
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn size(&self) -> usize;
    fn none_count(&self) -> usize;
    fn some_count(&self) -> usize;
    fn set(&mut self, index: usize, item: Self::Item);
    fn remove(&mut self, index: usize);
    fn append(&mut self, item: Self::Item);
    fn reset(&mut self);
    fn extract(&self) -> Vec<Option<Self::Item>>;
    fn reset_default(&mut self)
    where
        Self::Item: Default;
    fn iter_column(&self) -> Iter<Option<Self::Item>>;
}

#[allow(dead_code)]
pub struct DataColumn<T> {
    data: Vec<Option<T>>,
    pub name: String,
    pub data_type: &'static str,
}

impl<T: 'static + Default + Clone> DataColumnTrait for DataColumn<T> {
    type Item = T;

    fn new(data: Vec<Option<T>>, name: String) -> Self {
        Self {
            data,
            name,
            data_type: type_name::<T>(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index].as_ref()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn none_count(&self) -> usize {
        self.data.len() - self.data.iter().flatten().count()
    }

    fn some_count(&self) -> usize {
        self.data.iter().flatten().count()
    }

    fn set(&mut self, index: usize, item: T) {
        if index < self.data.len() {
            self.data[index] = Some(item);
        }
    }

    fn remove(&mut self, index: usize) {
        if index < self.data.len() {
            self.data[index] = None;
        }
    }

    fn append(&mut self, item: T) {
        self.data.push(Some(item));
    }

    fn reset(&mut self) {
        self.data = (0..self.data.len()).map(|_| None).collect();
    }

    fn reset_default(&mut self) {
        self.data = (0..self.data.len()).map(|_| Some(T::default())).collect();
    }

    fn iter_column(&self) -> Iter<Option<T>> {
        self.data.iter()
    }
    
    fn extract(&self) -> Vec<Option<T>> {
        let vec: Vec<_> = self.data.iter().cloned().collect();
        return vec
    }
}
