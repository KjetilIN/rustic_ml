// Import the perceptron
use rustic_ml::perceptron::Perceptron;

fn main() {
    // Lets imagine we have the following data:
    //
    // ________________________________________________
    // | Movie # | Alice    | Bob       | Profitable? |
    // |_________|__________|___________|_____________|
    // |  1      | 1        | 1         |      no     |
    // |_________|__________|___________|_____________|
    // |  2      | 4        | 3         |      yes    |
    // |_________|__________|___________|_____________|
    // |  3      | 3        | 5         |      yes    |
    // |_________|__________|___________|_____________|
    // |  4      | 5        | 6         |      yes    |
    // |_________|__________|___________|_____________|
    // |  5      | 2        | 3         |      no     |
    // |_________|__________|___________|_____________|
    //
    // Our goal is to classify a profitable movie, based on two critics score.
    // The score goes from 1-6.

    //Initialize the perceptron with a learning rate of 0.1
    let mut perceptron = Perceptron::init().learning_rate(1.0).bias(-1.0);

    // Setting up training data
    let x_train: Vec<(f64, f64)> = vec![(1.0, 1.0), (4.0, 3.0), (3.0, 5.0), (5.0, 6.0), (2.0, 3.0)];
    let y_train = vec![0.0, 1.0, 1.0, 1.0, 0.0];

    // Training until it learns the system.
    // This is something we can do since we know the data is linearly separable. (Plot out the data and see for yourself).
    // We can also use `fit_until_halt` if we don't want to log each epoch
    perceptron.fit_until_halt_with_logging(&x_train, &y_train);

    // Calculate the accuracy for the model
    let accuracy = perceptron.calculate_accuracy(&x_train, &y_train);

    // Print the accuracy
    println!("Model accuracy: {}%", accuracy);

    // Print model details
    perceptron.print_model();
}
