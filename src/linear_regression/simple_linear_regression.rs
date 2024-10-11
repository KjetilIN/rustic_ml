use rand::{distributions::Uniform, prelude::Distribution};

use crate::cost::CostFunction;

/// `SimpleLinearRegression` that represents a linear regression model with a single coefficient and
/// intercept.
///
/// Properties:
///
/// - `coefficient`: It is the value that multiplies the independent variable in the linear equation.
/// In a simple linear regression model with one independent variable, the coefficient represents the slope of the line.
/// - `intercept`: It is the value where the regression line crosses the y-axis.
/// - `cost_function`: represents a function that calculates the performance of the model.
/// This function takes two parameters: a reference to a vector of input values and a reference to a vector of target values, and returns a
/// single `f32` value representing the
/// - `learning_rate`: The `learning_rate` property in the `SimpleLinearRegression` struct represents
/// the rate at which the model parameters are updated during training. It is a hyperparameter that
/// determines the size of the steps taken while searching for the optimal values of the coefficients
/// (slope) and intercept of the linear regression model.
/// - `log_output`: a boolean flag that determines whether logging should be enabled during the training process of the model. If set
/// to `true`, the model will output logs during training, which can be helpful for monitoring the
/// progress and debugging any issues that. Default is set to `false`
#[allow(dead_code)]
pub struct SimpleLinearRegression {
    /// Coefficient of the model
    coefficient: f32,

    /// Intercept of the model
    intercept: f32,

    /// Function for calculating the performance of the model
    cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32,

    /// Learning rate of the model. Used during training the model
    learning_rate: f32,

    /// When set to true, will log during training
    log_output: bool,
}

impl SimpleLinearRegression {
    /// Create a new `SimpleLinearRegression` model
    ///
    /// A new instance of a struct is being returned. The struct contains fields for coefficient,
    /// intercept, cost_function, learning_rate, and log_output. The values for coefficient and
    /// intercept are randomly sampled from a uniform distribution between 0.0 and 1.0. The
    /// cost_function is set to mean_square_error, learning_rate is set to 1.0, and log_output is set to
    /// false
    pub fn new() -> Self {
        let uniform: Uniform<f32> = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Self {
            coefficient: uniform.sample(&mut rng),
            intercept: uniform.sample(&mut rng),
            cost_function: CostFunction::mean_square_error,
            learning_rate: 1.0,
            log_output: false,
        }
    }

    /// Allows for setting a cost function of the simple linear regression model.
    ///
    /// # Arguments:
    ///
    /// - `cost_function`: The cost function is a function that accepts two parameters: a reference to a vector
    /// of `f32` values and another reference to a vector of `f32` values, and returns an `f32
    ///
    /// # Returns:
    ///
    /// The simple linear regression model with the given cost function set.
    pub fn cost(mut self, cost_function: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Self {
        self.cost_function = cost_function;
        self
    }

    /// Sets the learning rate for the simple linear regression model.
    ///
    /// # Arguments:
    /// - `learning_rate`: represents the rate at which the model changes the intercept and coefficient
    ///
    /// # Returns:
    ///
    /// The simple linear regression model with the given learning rate set.
    pub fn learning_rate(mut self, learning_rate: f32) -> Self {
        self.learning_rate = learning_rate;
        self
    }

    /// Set logging of the model.
    ///
    /// Will log the training progress of the model each epoch.
    ///
    /// # Arguments:
    ///
    /// - `log_output`: if true, will log output of fitting the model
    ///
    /// # Returns:
    ///
    /// The simple linear regression model with the given learning rate set.
    pub fn logging(mut self, log_output: bool) -> Self {
        self.log_output = log_output;
        self
    }

    /// Train the model by performing gradient descent for a specified number of epochs to train a
    /// model and logs the cost of each epoch if enabled.
    ///
    /// # Arguments:
    ///
    /// - `x_train`: Training data of a model as a `Vec<f32>``
    /// - `y_train`: The expected output of each training values
    /// - `epochs`: Represents the number of times the model will iterate over the entire training dataset during the training process.
    pub fn fit(&mut self, x_train: &Vec<f32>, y_train: &Vec<f32>, epochs: usize) {
        for i in 0..epochs {
            // Perform one gradient descent
            self.gradient_descent_step(x_train, y_train);
            let cost = self.calculate_cost(x_train, y_train);

            // Log the cost of the epoch
            if self.log_output {
                println!("Epoch {}: Cost = {}, Coefficient = {}, Intercept = {}", i + 1, cost, self.coefficient, self.intercept);
            }
        }
    }

    /// Returns a predicted value from the given feature value
    ///
    /// # Arguments:
    ///
    /// - `x_feature`: represents the input feature value for which you want
    /// to make a prediction.
    ///
    /// # Returns:
    ///
    /// The predicted value for the given
    pub fn predict(&self, x_feature: f32) -> f32 {
        self.intercept + self.coefficient * x_feature
    }

    /// Calculates the cost on the provided training data.
    ///
    /// # Arguments:
    ///
    /// - `x_train`: Training data of a model as a `Vec<f32>``
    /// - `y_train`: The expected output of each training values
    ///
    /// # Returns:
    ///
    /// Returns the cost of the training dataset
    pub fn calculate_cost(&self, x_train: &Vec<f32>, y_train: &Vec<f32>) -> f32 {
        (self.cost_function)(x_train, y_train)
    }

    // Perform one step of gradient descent to update coefficient and intercept
    fn gradient_descent_step(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        let n = inputs.len() as f32;
        // Compute gradients
        let mut gradient_coefficient: f32 = 0.0;
        let mut gradient_intercept: f32 = 0.0;

        for (x, y_true) in inputs.iter().zip(targets) {
            let error = y_true - (self.coefficient*x + self.intercept);
            gradient_coefficient += error * x;
            gradient_intercept += error;
        }

        // Make the coefficient the average error
        gradient_coefficient *= 2.0/n;
        gradient_intercept *= 2.0/n;

        // Update the coefficient and intercept using the learning rate
        self.coefficient -= self.learning_rate * gradient_coefficient;
        self.intercept -= self.learning_rate * gradient_intercept;
    }
}
