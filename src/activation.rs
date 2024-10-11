pub struct Activation{}

impl Activation {
    /// Heaviside step function is a activation function that returns either 0 or 1.
    ///
    /// Read more about the step function here: <https://en.wikipedia.org/wiki/Heaviside_step_function>
    ///
    /// Arguments:
    ///
    /// - `value`: The `value` parameter is a floating-point number (f64) that the `h_step` function takes
    /// as input.
    ///
    /// Returns:
    ///
    /// The function `h_step` returns an unsigned integer value of `1` if the input `value` is greater than
    /// or equal to `1.0`, otherwise it returns `0`.
    pub fn h_step(value: f64) -> usize {
        if value >= 1.0 {
            1
        } else {
            0
        }
    }    


    pub fn sigmoid(value: f64) -> f64{
        1.0 / (1.0 + (-value).exp())
    }

    pub fn sigmoid_derivative(value: f64) -> f64{
        let sigmoid = Self::sigmoid(value);
        sigmoid*(1.0-sigmoid)
    }
}
