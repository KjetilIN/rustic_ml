use std::f32::consts::E;

#[derive(Debug)]
pub enum Activation {
    STEP,
    SIGMOID,
    RELU,
}

impl Activation {
    pub fn get_activation(&self) -> fn(f32) -> f32 {
        match self {
            Activation::STEP => h_step,
            Activation::SIGMOID => sigmoid,
            Activation::RELU => relu,
        }
    }
}

/// Heaviside step function is a activation function that returns either 0 or 1.
///
/// Read more about the step function here: <https://en.wikipedia.org/wiki/Heaviside_step_function>
///
/// Arguments:
///
/// - `value`: The `value` parameter is a floating-point number (f32) that the `h_step` function takes
/// as input.
///
/// Returns:
///
/// The function `h_step` returns an unsigned integer value of `1` if the input `value` is greater than
/// or equal to `1.0`, otherwise it returns `0`.
#[allow(dead_code)]
pub fn h_step(value: f32) -> f32 {
    if value >= 1.0 {
        1.0
    } else {
        0.0
    }
}

#[allow(dead_code)]
pub fn sigmoid(value: f32) -> f32{
    1.0 / (1.0 - E.powf(-value))
}

#[allow(dead_code)]
pub fn relu(value: f32) -> f32{
    f32::max(value, 0.0)
}