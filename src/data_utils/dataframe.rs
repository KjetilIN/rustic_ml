use super::datacolumn::DataColumnTrait;
use crate::data_utils::datacolumn::DataColumn;
use std::fs;

/// A  enumeration type that represents different types of columns that can be present in a dataset.
///
/// The variants of this enum are `Integer`, `Float`, `Boolean`, and
/// `Text`, which correspond to the possible data types that a column can have. This enum is used in the
/// `Dataframe` struct to infer the type of data present in each column when reading data from a file.
pub enum ColumnType {
    Integer,
    Float,
    Boolean,
    Text,
}

/// `DataColumnEnum` enum is used to represent different types of `DataColumn` instances.
///
/// Each variant of the enum corresponds to a specific type of data column
/// - `IntColumn` for columns containing integer data.
/// - `FloatColumn` for columns containing floating-point data.
/// - `BoolColumn` for columns containing boolean data.
/// - `TextColumn` for columns containing text data.
#[allow(dead_code)]
pub enum DataColumnEnum {
    /// Data column with i32 values
    IntColumn(DataColumn<i32>),

    /// Data column with f32 values
    FloatColumn(DataColumn<f32>),

    /// Data column with boolean values
    BoolColumn(DataColumn<bool>),

    /// Data column with string values
    TextColumn(DataColumn<String>),
}

/// `Dataframe` that represents a collection of columns of different data types.
///
/// Properties:
///
/// - `columns`: The `Dataframe` struct has a property `columns` which is a vector of `DataColumnEnum`
/// elements.
#[allow(dead_code)]
pub struct Dataframe {
    columns: Vec<DataColumnEnum>,
}

impl Dataframe {
    pub fn from_csv(path: String) -> Result<Self, ()> {
        Self::from_file(path, ';')
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

            if is_float && value.parse::<f32>().is_err() {
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
        let column_count: usize = csv_lines[0].split(delimiter).count();

        // Column names for the dataset
        let column_names: Vec<_> = csv_lines[0].split(delimiter).collect();

        // Get the data types for each column and initialize each column
        let columns_with_data: Vec<_> = csv_lines[1].split(delimiter).collect();

        // Create the vector of column data
        let mut dataframe_columns: Vec<DataColumnEnum> = Vec::with_capacity(column_count);

        for (index, _) in columns_with_data.iter().enumerate() {
            // Gather all data in this column as a vector of items
            let column_data: Vec<_> = csv_lines
                .iter()
                .skip(1)
                .map(|line| {
                    let values = line.split(delimiter).collect::<Vec<_>>();
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
                            let value = line.split(delimiter).collect::<Vec<_>>()[index];
                            match value.parse::<i32>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-integer values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::IntColumn(new_column));
                }
                ColumnType::Float => {
                    let data_vec: Vec<Option<f32>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(delimiter).collect::<Vec<_>>()[index];
                            match value.parse::<f32>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-float values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::FloatColumn(new_column));
                }
                ColumnType::Boolean => {
                    let data_vec: Vec<Option<bool>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value = line.split(delimiter).collect::<Vec<_>>()[index];
                            match value.parse::<bool>() {
                                Ok(parsed_val) => Some(parsed_val),
                                Err(_) => None, // Handle non-boolean values as None
                            }
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::BoolColumn(new_column));
                }
                ColumnType::Text => {
                    let data_vec: Vec<Option<String>> = csv_lines
                        .iter()
                        .skip(1)
                        .map(|line| {
                            let value =
                                line.split(delimiter).collect::<Vec<_>>()[index].to_string();
                            Some(value)
                        })
                        .collect();

                    let new_column = DataColumn::new(data_vec, column_names[index].to_owned());
                    dataframe_columns.push(DataColumnEnum::TextColumn(new_column));
                }
            }
        }

        Ok(Dataframe {
            columns: dataframe_columns,
        })
    }

    pub fn to_csv(&self, path: String) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn print(&self) {
        self.head();
        println!("............");
        self.tail();
    }

    fn print_full_table(&self) {
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

    /// Calculate the total memory used for the `Dataframe`
    ///
    /// Returns:
    ///
    /// The total memory usage of all columns in the `Dataframe` in bytes.
    pub fn memory_usage(&self) -> usize {
        let mut total_memory: usize = 0;

        for column in &self.columns {
            let column_memory = match column {
                DataColumnEnum::IntColumn(col) => {
                    let memory = col.size() * (size_of::<Option<i32>>());
                    memory
                }
                DataColumnEnum::FloatColumn(col) => {
                    let memory = col.size() * (size_of::<Option<f32>>());
                    memory
                }
                DataColumnEnum::BoolColumn(col) => {
                    let memory = col.size() * (size_of::<Option<bool>>());
                    memory
                }
                DataColumnEnum::TextColumn(col) => {
                    let memory: usize = col
                        .iter_column()
                        .map(|opt| {
                            match opt {
                                Some(s) => size_of::<Option<String>>() + s.capacity(), // size of the string's heap allocation
                                None => size_of::<Option<String>>(), // just the size of the Option
                            }
                        })
                        .sum();
                    println!("{:<15} {:<15}", col.name, memory);
                    memory
                }
            };

            total_memory += column_memory;
        }

        total_memory
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn drop_column(&self) {
        unimplemented!()
    }

    pub fn at(&self) {
        unimplemented!()
    }

    pub fn at_index(&self) {
        unimplemented!()
    }
}
