use rustic_ml::{
    data_utils::dataframe::Dataframe,
    linear_regression::simple_linear_regression::SimpleLinearRegression,
};

fn main() {
    // Reading the dataset
    let path: String = "./datasets/sat_gpa.csv".to_owned();
    let dataframe = Dataframe::from_file(path, ',').unwrap();

    // Create the model
    let mut model = SimpleLinearRegression::new()
        .learning_rate(0.01)
        .logging(true);

    // Extract features
    let x_train = dataframe
        .float_feature("SAT")
        .expect("SAT column not in dataset");
    let y_train = dataframe
        .float_feature("GPA")
        .expect("GPA feature did not exist");
//
    //let min = x_train.iter().cloned().fold(f32::INFINITY, f32::min);
    //let max = x_train.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    //let x_train_normalized: Vec<f32> = x_train.iter().map(|&x| (x - min) / (max - min)).collect();
//
    //let y_train_normalized: Vec<f32> = y_train.iter().map(|&y| (y - min) / (max - min)).collect();

    // Fit the model
    let epochs = 1000;
    model.fit(&x_train, &y_train, epochs);
}
