use rustic_ml::data_utils::dataframe::Dataframe;

fn main() {
    let path = String::from("./datasets/european_cities.csv");
    let dataframe = Dataframe::from_csv(path).unwrap();

    // Print the info
    dataframe.info();

    // We can also get the total amount of bytes used
    let total_bytes_used = dataframe.memory_usage();
    println!(
        "\nMemory usage for the dataframe: {} bytes",
        total_bytes_used
    );
}
