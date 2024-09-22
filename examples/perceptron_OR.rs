// Import the perceptron 
use rustic_ml::perceptron::Perceptron;

fn main() {
    //Initialize the perceptron with a learning rate of 0.1
    let mut perceptron = Perceptron::with_learning_rate(0.1);


    // This demo 
    // The dataset consists of two features and a bias term (bias, x1, x2), and the corresponding labels
    // Each input feature vector contains the bias term (set to 1.0 for simplicity) followed by two features
    let x_train = vec![
        vec![1.0, 0.0, 0.0],  // Sample 1
        vec![1.0, 0.0, 1.0],  // Sample 2
        vec![1.0, 1.0, 0.0],  // Sample 3
        vec![1.0, 1.0, 1.0],  // Sample 4
    ];

    // Corresponding labels (binary classification: 0 or 1)
    //let y_train = vec![0.0, 0.0, 0.0, 1.0];

    // Number of epochs to train
    //let epochs = 10;

    // Step 3: Train the perceptron using the fit method
    //perceptron.fit(&x_train, &y_train, &epochs);
    //// Step 4: Make predictions on new data after training
    //let test_data = vec![
    //    vec![1.0, 0.0, 0.0],  // Test 1
    //    vec![1.0, 1.0, 1.0],  // Test 2
    //    vec![1.0, 0.5, 0.5],  // Test 3 (example of unseen data)
    //];
    //for (i, features) in test_data.iter().enumerate() {
    //    let prediction = perceptron.predict(features);
    //    println!("Prediction for test {}: {}", i + 1, prediction);
    //}
}
