use rand::{distributions::Uniform, prelude::Distribution};

use crate::activation::h_step;

/// Perceptron - represents a single-layer neural network with bias and weights.
/// 
/// Properties:
/// 
/// - `bias`: a constant value that allows the perceptron to adjust its output independently of the input. 
/// It helps the perceptron in making decisions by shifting the decision boundary.
/// - `w1` and `w2`: represent weights in the network. A perceptron has two weights
#[allow(dead_code)]
pub struct Perceptron{
    bias: f64,
    w1: f64,
    w2: f64
}

impl Perceptron {
    /// Initializes a `Perceptron` struct with random values for bias and weights.
    /// 
    /// Returns:
    /// 
    /// A `Perceptron` struct is being returned with randomly initialized bias and weights.
    pub fn init() -> Self{
        let uniform: Uniform<f64> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Perceptron { bias: uniform.sample(&mut rng), w1: uniform.sample(&mut rng), w2: uniform.sample(&mut rng) }
    }

    /// Initializes a new `Perceptron` instance with random bias and weights
    /// sampled from a given uniform distribution.
    /// 
    /// Arguments:
    /// 
    /// - `uniform`: The `with_uniform` function takes a reference to a `Uniform<f64>` object as a
    /// parameter. This object is used to generate random samples from a uniform distribution. 
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Perceptron` struct is being returned, with the `bias`, `w1`, and `w3`
    /// fields initialized with random samples from the provided `Uniform<f64>` distribution.
    pub fn with_uniform(uniform: &Uniform<f64>) -> Self{
        let mut rng = rand::thread_rng();
        Perceptron { bias: uniform.sample(&mut rng), w1: uniform.sample(&mut rng), w2: uniform.sample(&mut rng) }
    } 

    pub fn fit(&self, x_train: &Vec<f32>, y_train: &Vec<f64>, epocs: &usize){
        unimplemented!("fit");
    }
    
    pub fn predict(&self, features: &Vec<f64>) -> usize{
        assert!(features.len() == 3, "Feature vector must be 3, for Perceptron being able to predict value");
        
        // Return the output of the prediction
        // Using the Heaviside step function to get the value as 0 or 1
        h_step(self.bias * features[0] + self.w1 * features[1] + self.w2 * features[2])
    }
}