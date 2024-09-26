use std::{any::Any, fs};

use crate::data_utils::datacolumn::DataColumn;

use super::datacolumn::DataColumnTrait;

enum ColumnType {
    Integer,
    Float,
    Boolean,
    Text,
}

// Enum to represent different types of DataColumn
#[allow(dead_code)]
enum DataColumnEnum {
    IntColumn(Box<DataColumn<i32>>),
    FloatColumn(Box<DataColumn<f64>>),
    BoolColumn(Box<DataColumn<bool>>),
    TextColumn(Box<DataColumn<String>>),
}

#[allow(dead_code)]
pub struct Dataframe {
    columns: Vec<DataColumnEnum>,
}

impl Dataframe {
    pub fn from_csv(path: String) -> Result<Self, ()> {
        // Read the file
        let contents = match fs::read_to_string(&path) {
            Ok(val) => val,
            Err(_) => {
                println!("ERROR: could not read csv file: {}", path);
                return Err(());
            }
        };

        // Collect to a vector of lines
        let csv_lines: Vec<_> = contents.lines().collect();

        // Count how many columns there are
        let column_count: usize = csv_lines[0].split(";").count();

        // Column names for the dataset
        let column_names: Vec<_> = csv_lines[0].split(";").collect();

        // Get the data types for each column and initialize each column
        let columns_with_data: Vec<_> = csv_lines[1].split(";").collect();

        // Create the vector of column data
        let mut dataframe_columns: Vec<DataColumnEnum> = Vec::with_capacity(column_count);

        for (index, _) in columns_with_data.iter().enumerate() {
            // Gather all data in this column as a vector of items
            let column_data: Vec<_> = csv_lines
                .iter()
                .skip(1)
                .map(|line| {
                    let values = line.split(";").collect::<Vec<_>>();
                    values[index].trim().to_string() // Trim the value
                })
                .collect();

            // Get the column type
            let column_type = Self::infer_column_type(&column_data);

            match column_type {
                ColumnType::Integer => {
                    // Collect all data for the given column
                    let data_vec: Vec<Option<i32>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(";").collect::<Vec<_>>()[index];
                            match value.parse::<i32>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-integer values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::IntColumn(Box::new(new_column)));
                }
                ColumnType::Float => {
                    let data_vec: Vec<Option<f64>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(";").collect::<Vec<_>>()[index];
                            match value.parse::<f64>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-float values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::FloatColumn(Box::new(new_column)));
                }
                ColumnType::Boolean => {
                    let data_vec: Vec<Option<bool>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(";").collect::<Vec<_>>()[index];
                            match value.parse::<bool>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-boolean values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::BoolColumn(Box::new(new_column)));
                }
                ColumnType::Text => {
                    let data_vec: Vec<Option<String>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(";").collect::<Vec<_>>()[index].to_string();
                            Some(value)
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::TextColumn(Box::new(new_column)));
                }
            }
        }

        Ok(Dataframe {
            columns: dataframe_columns,
        })
    }

    fn infer_column_type(column_data: &[String]) -> ColumnType {
        let mut is_integer = true;
        let mut is_float = true;
        let mut is_boolean = true;

        for value in column_data {
            if value.is_empty() {
                continue; // Skip empty values
            }

            if is_integer && value.parse::<i32>().is_err() {
                is_integer = false;
            }

            if is_float && value.parse::<f64>().is_err() {
                is_float = false;
            }

            if is_boolean && value.parse::<bool>().is_err() {
                is_boolean = false;
            }

            // If none of the above parsers succeeded, it must be text
            if !is_integer && !is_float && !is_boolean {
                return ColumnType::Text;
            }
        }

        // Decide the type based on what was true
        if is_integer {
            ColumnType::Integer
        } else if is_float {
            ColumnType::Float
        } else if is_boolean {
            ColumnType::Boolean
        } else {
            ColumnType::Text
        }
    }

    pub fn from_file(path: String, delimiter: char) -> Result<Self, ()> {
        unimplemented!()
    }

    pub fn to_csv(&self, path: String) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn head(&self) {
        unimplemented!()
    }

    pub fn tail(&self) {
        unimplemented!()
    }

    pub fn info(&self) {
        // Print table headers
        println!(
            "{:<20} {:<10} {:<10} {:<15} {:<15}",
            "Column Name", "Type", "None", "Some", "Total Length"
        );
        println!("{:-<65}", ""); // Divider line

        // Iterate through each column and print the info
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(col) => {
                    println!(
                        "{:<20} {:<10} {:<10} {:<15} {:<15}",
                        col.name,         // Column name
                        "Integer",        // Type
                        col.none_count(), // None values count
                        col.some_count(), // Some values count
                        col.size()        // Total length
                    );
                }
                DataColumnEnum::FloatColumn(col) => {
                    println!(
                        "{:<20} {:<10} {:<10} {:<15} {:<15}",
                        col.name,
                        "Float",
                        col.none_count(),
                        col.some_count(),
                        col.size()
                    );
                }
                DataColumnEnum::BoolColumn(col) => {
                    println!(
                        "{:<20} {:<10} {:<10} {:<15} {:<15}",
                        col.name,
                        "Boolean",
                        col.none_count(),
                        col.some_count(),
                        col.size()
                    );
                }
                DataColumnEnum::TextColumn(col) => {
                    println!(
                        "{:<20} {:<10} {:<10} {:<15} {:<15}",
                        col.name,
                        "Text",
                        col.none_count(),
                        col.some_count(),
                        col.size()
                    );
                }
            }
        }
    }

    pub fn memory_usage(&self) {
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn drop_column(&self) {}

    pub fn at(&self) {
        unimplemented!()
    }

    pub fn at_index(&self) {
        unimplemented!()
    }
}
