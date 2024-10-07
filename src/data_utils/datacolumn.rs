use std::{any::type_name, slice::Iter};

// Define a trait for DataColumn
pub trait DataColumnTrait {
    type Item; // Associated type to represent the type of data (T)

    /// Create a new DataColumn
    fn new(data: Vec<Option<Self::Item>>, name: String) -> Self
    where
        Self: Sized;

    /// Get the item in the column
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Get the size of the column
    fn size(&self) -> usize;

    /// Return the count of how many `None` values there are in the column
    fn none_count(&self) -> usize;

    /// Return the count of how many `Some` values there are
    fn some_count(&self) -> usize;

    /// Set a value for an item in the column
    fn set(&mut self, index: usize, item: Self::Item);

    /// Remove an item the column
    fn remove(&mut self, index: usize);

    /// Append data to the column
    fn append(&mut self, item: Self::Item);

    /// Reset the column to a vector of `None` values
    fn reset(&mut self);

    /// Extract method
    fn extract(&self) -> Vec<Option<Self::Item>>;

    /// Reset to the default value of the given column
    fn reset_default(&mut self)
    where
        Self::Item: Default;

    /// Return an `Iter` for the datacolumn
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
        return vec;
    }
}
