use rustic_ml::data_utils::dataframe::Dataframe;

fn main() {
    let path = String::from("./datasets/european_cities.csv");
    let dataframe = Dataframe::from_csv(path).unwrap();

    dataframe.info();
}
