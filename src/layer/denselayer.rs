

use crate::data_utils::matrix::Matrix;
use super::layer_trait::Layer;

pub struct DenseLayer;


impl Layer for DenseLayer{
    fn get_params(&self) -> usize {
        todo!()
    }
    
    fn feed_forward(&self, input:&mut Matrix) -> &Matrix {
        todo!()
    }
    
    fn backward(&mut self, gradient:&mut Matrix) -> &Matrix {
        todo!()
    }
}