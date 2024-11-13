use crate::data_utils::matrix::Matrix;

/// Layer trait for each layer
pub trait Layer {
    fn feed_forward(&self, input: &Matrix) -> Matrix;
    fn backward(&mut self, gradient: &Matrix) -> &Matrix;
    fn get_params(&self) -> usize;
}
