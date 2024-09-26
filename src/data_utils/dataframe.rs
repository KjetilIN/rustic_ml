use std::{any::Any, fs};

use crate::data_utils::datacolumn::DataColumn;

use super::datacolumn::DataColumnTrait;


enum ColumnType{
    Integer,
    Float,
    Boolean,
    Text
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
        println!("[INFO] {} columns in the dataset", column_count);

        // Get the data types for each column and initialize each column
        let columns_with_data = csv_lines[1].split(";").collect::<Vec<_>>();

        // Create the vector of column data
        let mut dataframe_columns: Vec<DataColumnEnum> = Vec::with_capacity(column_count);

        for column in columns_with_data {
            let column_type = Self::infer_column_type(column);

            match column_type {
                ColumnType::Integer => {
                    let data: i32 = column.parse().expect("parse integer data");
                    let data_vec: Vec<Option<i32>> = vec![Some(data)];
                    let new_column = DataColumn::new(data_vec);
                    dataframe_columns.push(DataColumnEnum::IntColumn(Box::new(new_column)));
                }
                ColumnType::Float => {
                    let data: f64 = column.parse().expect("parse float data");
                    let data_vec: Vec<Option<f64>> = vec![Some(data)];
                    let new_column = DataColumn::new(data_vec);
                    dataframe_columns.push(DataColumnEnum::FloatColumn(Box::new(new_column)));
                }
                ColumnType::Boolean => {
                    let data: bool = column.parse().expect("parse boolean data");
                    let data_vec: Vec<Option<bool>> = vec![Some(data)];
                    let new_column = DataColumn::new(data_vec);
                    dataframe_columns.push(DataColumnEnum::BoolColumn(Box::new(new_column)));
                }
                ColumnType::Text => {
                    let data = column.to_string();
                    let data_vec: Vec<Option<String>> = vec![Some(data)];
                    let new_column = DataColumn::new(data_vec);
                    dataframe_columns.push(DataColumnEnum::TextColumn(Box::new(new_column)));
                }
            }
        }

        Ok(Dataframe {
            columns: dataframe_columns,
        })
    }

    fn infer_column_type(value: &str) -> ColumnType {
        if value.parse::<i64>().is_ok() {
            ColumnType::Integer
        } else if value.parse::<f64>().is_ok() {
            ColumnType::Float
        } else if value.parse::<bool>().is_ok() {
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
        println!("{:<15} {:<10}", "Column Type", "Count");
        println!("{:-<25}", "");

        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(col) => {
                    println!("{:<15} {:<10}", "Integer", col.size());
                }
                DataColumnEnum::FloatColumn(col) => {
                    println!("{:<15} {:<10}", "Float", col.size());
                }
                DataColumnEnum::BoolColumn(col) => {
                    println!("{:<15} {:<10}", "Boolean", col.size());
                }
                DataColumnEnum::TextColumn(col) => {
                    println!("{:<15} {:<10}", "Text", col.size());
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
