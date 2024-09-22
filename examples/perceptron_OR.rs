// Import the perceptron
use rustic_ml::perceptron::Perceptron;

fn main() {
    //Initialize the perceptron with a learning rate of 0.1
    let mut perceptron = Perceptron::init()
                                                .learning_rate(0.1)
                                                .bias(-1.0);

    // OR training data set
    // This training dataset includes all possible versions of the OR
    let x_train: Vec<(f64, f64)> = vec![
        (0.0, 0.0),
        (0.0, 1.0),
        (1.0, 0.0),
        (1.0, 1.0),
    ];

    // Corresponding labels (binary classification: 0 or 1)
    let y_train = vec![0.0, 1.0, 1.0, 1.0];


    // Train the perceptron using the fit method
    // Alternatively use the fit() method to not log the accuracy over time
    // Here with 10 epochs 
    perceptron.fit_with_logging(&x_train, &y_train, 10);
    
    // Test data 
    let test_data = vec![
        (0.0, 0.0),  
        (1.0, 0.0),  
        (0.0, 1.0),  
        (1.0, 1.0),  
    ];

    // Calculate the accuracy for the model
    let accuracy = perceptron.calculate_accuracy(&test_data, &y_train);

    // Print the accuracy 
    println!("Model accuracy: {}%", accuracy);
}
