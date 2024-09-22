use rand::{distributions::Uniform, prelude::Distribution};

use crate::activation::h_step;

/// Perceptron - represents a single-layer neural network with bias and weights.
/// Typically used for a binary classification, if the data is linearly separable.  
///
/// Properties:
///
/// - `learning_rate` is used to control how fast the weights change during learning.
/// - `bias`: a constant value that allows the perceptron to adjust its output independently of the input.
/// It helps the perceptron in making decisions by shifting the decision boundary.
/// - `w1` and `w2`: represent weights in the network. A perceptron has two weights
#[allow(dead_code)]
pub struct Perceptron {
    learning_rate: f64,
    bias: f64,
    w1: f64,
    w2: f64,
}

impl Perceptron {
    /// Initializes a `Perceptron` struct with random values for bias and weights.
    ///
    /// Returns:
    ///
    /// A `Perceptron` struct is being returned with randomly initialized bias and weights.
    /// Learning rate is by default 1. Use `with_learning_rate` to create a new perceptron with given learning rate.
    pub fn init() -> Self {
        let uniform: Uniform<f64> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Perceptron {
            learning_rate: 1.0,
            bias: uniform.sample(&mut rng),
            w1: uniform.sample(&mut rng),
            w2: uniform.sample(&mut rng),
        }
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
    pub fn with_uniform(uniform: &Uniform<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Perceptron {
            learning_rate: 1.0,
            bias: uniform.sample(&mut rng),
            w1: uniform.sample(&mut rng),
            w2: uniform.sample(&mut rng),
        }
    }


    /// Sets the learning rate for the given Perceptron and returns the modified
    /// perceptron.
    /// 
    /// Method should be used by chaining.
    /// 
    /// Arguments:
    /// 
    /// . `learning_rate`: the rate at which a machine learning model adjusts
    /// its parameters during training.
    /// 
    /// Returns:
    /// 
    /// Returns the modified Perceptron with given learning rate
    pub fn learning_rate(mut self, learning_rate: f64) -> Self {
        self.learning_rate = learning_rate;
        self
    }

    /// Sets the bias value of the Perceptron and returns the modified Perceptron.
    /// 
    /// Arguments:
    /// 
    /// - `bias`: The `bias` parameter in the `bias` function is a floating-point number (`f64`) that
    /// represents the bias value to be set for the object.
    /// 
    /// Returns:
    /// 
    /// Returns the modified Perceptron with given bias
    pub fn bias(mut self, bias: f64) -> Self {
        self.bias = bias;
        self
    }

    pub fn fit(&mut self, x_train: &Vec<(f64, f64)>, y_train: &Vec<f64>, epochs: usize) {
        // For each epoch
        for _ in 0..epochs{
            // Iterate over the dataset and recalculate the 
            for (x, y) in x_train.iter().zip(y_train.iter()){
                let target = *y;
                let guess = self.predict(x) as f64;

                // Check if we need to update the weights
                if target != guess{
                    // Update weights
                    self.w1 += self.learning_rate * (target - guess) * x.0;
                    self.w2 += self.learning_rate * (target - guess) * x.1;

                    // Update bias 
                    self.bias += self.learning_rate * (target - guess);
                }

            }
        }
    }

    pub fn fit_with_logging(&mut self, x_train: &Vec<(f64, f64)>, y_train: &Vec<f64>, epochs: usize) {
        // For each epoch
        for epoch in 0..epochs{
            // Iterate over the dataset and recalculate the 
            let mut correct_predictions = 0;
            for (x, y) in x_train.iter().zip(y_train.iter()){
                let target = *y;
                let guess = self.predict(x) as f64;

                // Check if we need to update the weights
                if target != guess{
                    // Update weights
                    self.w1 += self.learning_rate * (target - guess) * x.0;
                    self.w2 += self.learning_rate * (target - guess) * x.1;

                    // Update bias 
                    self.bias += self.learning_rate * (target - guess);
                }else{
                    correct_predictions += 1;
                }

            }

            // Calculate the percentage
            let accuracy = if x_train.len() > 0 {
                (correct_predictions as f64 / x_train.len() as f64) * 100.0
            } else {
                0.0 
            };

            println!("Epoch {}: {}% accuracy", epoch + 1,accuracy);
        }
    }

    pub fn fit_until_halt(&mut self, x_train: &Vec<(f64, f64)>, y_train: &Vec<f64>) {
        // Loop forever
        loop{
            // Variable for if the weights have been updated
            let mut has_updated = false;

            // Iterate over the dataset and recalculate the 
            for (x, y) in x_train.iter().zip(y_train.iter()){
                let target = *y;
                let guess = self.predict(x) as f64;

                // Check if we need to update the weights
                if target != guess{
                    // Update weights
                    self.w1 += self.learning_rate * (target - guess) * x.0;
                    self.w2 += self.learning_rate * (target - guess) * x.1;

                    // Update bias 
                    self.bias += self.learning_rate * (target - guess);
                    has_updated = true
                }
            }

            if !has_updated{
                break;
            }
        }
    }

    pub fn fit_until_halt_with_logging(&mut self, x_train: &Vec<(f64, f64)>, y_train: &Vec<f64>) {
        // Forever loop
        let mut epochs_count = 0;
        loop{
            // Increment the epochs count
            epochs_count += 1;

            // Iterate over the dataset and recalculate the 
            let mut correct_predictions = 0;
            let mut has_updated = false; 
            for (x, y) in x_train.iter().zip(y_train.iter()){
                let target = *y;
                let guess = self.predict(x) as f64;

                // Check if we need to update the weights
                if target != guess{
                    // Update weights
                    self.w1 += self.learning_rate * (target - guess) * x.0;
                    self.w2 += self.learning_rate * (target - guess) * x.1;

                    // Update bias 
                    self.bias += self.learning_rate * (target - guess);

                    has_updated = true;
                }else{
                    correct_predictions += 1;
                }

            }

            // Calculate the percentage
            let accuracy = if x_train.len() > 0 {
                (correct_predictions as f64 / x_train.len() as f64) * 100.0
            } else {
                0.0 
            };

            println!("Epoch {}: {}% accuracy", epochs_count,accuracy);

            if !has_updated{
                break;
            }
        }
    }

    /// Get a prediction on the given features.
    /// 
    /// It uses a Heaviside step function as the activation function. 
    /// Takes the weights dot product the features, and add the bias before using the activation function.
    /// 
    /// Arguments:
    /// 
    /// - `features`: two features as a tuple.
    /// 
    /// Returns:
    /// 
    /// Either 0 or 1, representing a prediction of a class. 
    pub fn predict(&self, features: &(f64, f64)) -> usize {
        // Return the output of the prediction
        // Using the Heaviside step function to get the value as 0 or 1
        h_step(self.bias + self.w1 * features.0 + self.w2 * features.1)
    }

    /// Calculates the accuracy of predictions made by a model based on input data and
    /// target values.
    /// 
    /// Arguments:
    /// 
    /// - `x_data`: a vector of tuples where each tuple contains the two features, that we want to measure accuracy on.
    /// - `t_data`: a vector of labeled data, that are correct for the given data. These target values are
    /// used to compare against the predictions made by the model to determine the accuracy of the
    /// model's predictions.
    /// 
    /// Returns:
    /// 
    /// The accuracy as percentage as a f64
    pub fn calculate_accuracy(&self, x_data: &Vec<(f64, f64)>, t_data: &Vec<f64>) -> f64{
        let mut correct_predictions = 0;
        for (i, features) in x_data.iter().enumerate() {
            let prediction = self.predict(features);
            if prediction == (t_data[i] as usize){
                correct_predictions += 1;
            }
        }

        // Calculate the percentage
        let accuracy = if x_data.len() > 0 {
            (correct_predictions as f64 / x_data.len() as f64) * 100.0
        } else {
            0.0 // Avoid division by zero
        };

        // Return the accuracy 
        accuracy
    }

    /// Print the parameters of the model.
    pub fn print_model(&self){
        println!("         |Perceptron");
        println!("----------------------------------------");
        println!("   Bias  | {}", self.bias);
        println!("----------------------------------------");
        println!("    W1   | {}", self.w1);
        println!("----------------------------------------");
        println!("    W2   | {}", self.w2);
    }
}