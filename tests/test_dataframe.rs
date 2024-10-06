#[cfg(test)]
mod tests {
    use rustic_ml::data_utils::dataframe::{ColumnType, Dataframe};

    #[test]
    fn test_from_csv() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path);
        assert!(dataframe.is_ok())
    }

    #[test]
    fn test_from_file() {
        let path = String::from("./datasets/european_cities.txt");
        let dataframe = Dataframe::from_file(path, ' ');
        assert!(dataframe.is_ok())
    }

    #[test]
    fn test_column_names() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path);
        assert!(dataframe.is_ok());

        let columns = dataframe.unwrap().column_names();
        assert!(
            columns
                == vec![
                    "Barcelona",
                    "Belgrade",
                    "Berlin",
                    "Brussels",
                    "Bucharest",
                    "Budapest",
                    "Copenhagen",
                    "Dublin",
                    "Hamburg",
                    "Istanbul",
                    "Kyiv",
                    "London",
                    "Madrid",
                    "Milan",
                    "Moscow",
                    "Munich",
                    "Paris",
                    "Prague",
                    "Rome",
                    "Saint Petersburg",
                    "Sofia",
                    "Stockholm",
                    "Vienna",
                    "Warsaw"
                ]
        )
    }

    #[test]
    fn test_rename_colum() {
        let path = String::from("./datasets/european_cities.csv");
        let mut dataframe = Dataframe::from_csv(path).unwrap();

        assert!(dataframe.has_column("Barcelona"));
        assert!(!dataframe.has_column("Oslo"));

        dataframe.rename_column(0, "Oslo");
        assert!(dataframe.has_column("Oslo"));
    }

    #[test]
    fn test_memory_usage() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();
        assert!(
            dataframe.memory_usage() == 4608,
            "Memory usage was {}",
            dataframe.memory_usage()
        );
    }

    #[test]
    fn test_has_rows() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();
        assert!(dataframe.has_rows());
    }

    #[test]
    fn test_has_records() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();

        assert!(dataframe.has_records());
    }

    #[test]
    fn test_has_columns() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();
        assert!(dataframe.has_columns());
    }

    #[test]
    fn test_has_column() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();

        assert!(dataframe.has_column("Barcelona"));
        assert!(!dataframe.has_column("Oslo"));
    }

    #[test]
    fn test_drop_column() {
        let path = String::from("./datasets/european_cities.csv");
        let mut dataframe = Dataframe::from_csv(path).unwrap();

        assert!(dataframe.has_column("Barcelona"));

        dataframe.drop_column("Barcelona");
        assert!(!dataframe.has_column("Barcelona"));
    }

    #[test]
    fn test_add_column() {
        let path = String::from("./datasets/european_cities.csv");
        let mut dataframe = Dataframe::from_csv(path).unwrap();
        assert!(!dataframe.has_column("custom_index_column"));

        dataframe.add_column(vec![1, 2, 3, 4], "custom_index_column");
        assert!(dataframe.has_column("custom_index_column"));
    }

    #[test]
    fn test_get_column_type() {
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();

        assert!(dataframe.has_column("Barcelona"));
        assert!(!dataframe.has_column("Oslo"));

        assert!(dataframe.get_column_type("Barcelona") == Some(ColumnType::Float));
        assert!(dataframe.get_column_type("Oslo") == None);
    }

    #[test]
    fn test_at_str(){
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();


        assert!(dataframe.at_str("Barcelona", 2) == Some("1497.61".to_string()));
        assert!(dataframe.at_str("Berlin", 1) == Some("999.25".to_string()));
        assert!(dataframe.at_str("Paris", 5) == Some("1247.61".to_string()));
        assert!(dataframe.at_str("Warsaw", 23) == Some("0".to_string()));
        assert!(dataframe.at_str("Brussels", 2) == Some("651.62".to_string()));


        assert!(dataframe.at_str("Oslo", 1) == None);
        assert!(dataframe.at_str("Brussels", 50) == None);
        assert!(dataframe.at_str("America", 5) == None);
    }

    #[test]
    fn test_at_index_str(){
        let path = String::from("./datasets/european_cities.csv");
        let dataframe = Dataframe::from_csv(path).unwrap();


        assert!(dataframe.at_index_str(2) == Some("1497.61".to_string()));
        assert!(dataframe.at_index_str(24) == Some("1528.13".to_string()));
        assert!(dataframe.at_index_str(49) == Some("999.25".to_string()));

        assert!(dataframe.at_index_str((24*24 +1) as usize) == None)
    }


}
