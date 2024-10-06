use super::datacolumn::DataColumnTrait;
use crate::data_utils::datacolumn::DataColumn;
use std::fs;

/// A  enumeration type that represents different types of columns that can be present in a dataset.
///
/// The variants of this enum are `Integer`, `Float`, `Boolean`, and
/// `Text`, which correspond to the possible data types that a column can have. This enum is used in the
/// `Dataframe` struct to infer the type of data present in each column when reading data from a file.
#[derive(PartialEq, Eq, Debug)]
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
/// Used for managing data in an efficient way.
#[allow(dead_code)]
pub struct Dataframe {
    columns: Vec<DataColumnEnum>,
    rows_count: u32,
}

impl Dataframe {
    /// Reads data from a CSV file using a semicolon as the delimiter, and creates a `Dataframe`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// ```
    ///
    /// # Arguments:
    ///
    /// - `path`: The `path` parameter is a `String` that represents the file path to a CSV file that
    /// you want to read from.
    ///
    /// # Errors:
    /// - When file is not found, path was not correct
    ///
    /// # Returns:
    ///
    /// The `from_csv` function is returning a `Result` containing either an instance of the struct it
    /// belongs to (represented by `Self`) or an empty tuple `()`.
    pub fn from_csv(path: String) -> Result<Self, ()> {
        Self::from_file(path, ';')
    }

    /// Get the `ColumnType` of a given list of data.
    ///
    /// Will check the whole column, and determine its data based on what it was able to cast to.
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

    /// Infer the column type from a vector
    fn infer_column_type_from_vec<T>(column_data: &Vec<T>) -> ColumnType
    where
        T: ToString, // Ensure T can be converted to string and parsed
    {
        let mut is_integer = true;
        let mut is_float = true;
        let mut is_boolean = true;

        for value in column_data.iter() {
            let value_as_string = value.to_string().trim().to_string();

            if value_as_string.is_empty() {
                continue; // Skip empty values
            }

            // Check if all values can be integers
            if is_integer && value_as_string.parse::<i32>().is_err() {
                is_integer = false;
            }

            // Check if all values can be floats
            if is_float && value_as_string.parse::<f64>().is_err() {
                is_float = false;
            }

            // Check if all values can be booleans
            if is_boolean && value_as_string.parse::<bool>().is_err() {
                is_boolean = false;
            }

            // If none of the parsing succeeded, treat the column as text
            if !is_integer && !is_float && !is_boolean {
                return ColumnType::Text;
            }
        }

        // Return the most appropriate type based on successful parsing
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

    /// Reads data from a  file using the given delimiter, and creates a `Dataframe`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.txt");
    /// let dataframe = Dataframe::from_file(path, ' ').unwrap();
    /// ```
    ///
    /// # Arguments:
    ///
    /// - `path`: The `path` parameter is a `String` that represents the file path to a CSV file that
    /// you want to read from.
    /// - 'delimiter': The delimiter that septate records
    ///
    /// # Errors:
    /// - When file is not found, path was not correct
    ///
    /// # Returns:
    ///
    /// The `from_csv` function is returning a `Result` containing either an instance of the struct it
    /// belongs to (represented by `Self`) or an empty tuple `()`.    
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
            rows_count: contents.lines().count() as u32,
        })
    }

    pub fn to_csv(&self, _path: String) -> Result<(), ()> {
        unimplemented!()
    }

    /// Get all the column names for the `Dataframe`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// let columns = dataframe.column_names();
    /// assert!(columns == vec!["Barcelona","Belgrade","Berlin","Brussels", "Bucharest","Budapest","Copenhagen","Dublin","Hamburg","Istanbul","Kyiv","London","Madrid","Milan","Moscow","Munich","Paris","Prague","Rome","Saint Petersburg","Sofia","Stockholm","Vienna","Warsaw"])
    ///
    /// ```
    pub fn column_names(&self) -> Vec<String> {
        // Vector of all the names created with capacity
        let mut names: Vec<String> = Vec::with_capacity(self.columns.len());

        // Loop through each column and take add the name to the vector
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(data_column) => names.push(data_column.name.clone()),
                DataColumnEnum::FloatColumn(data_column) => names.push(data_column.name.clone()),
                DataColumnEnum::BoolColumn(data_column) => names.push(data_column.name.clone()),
                DataColumnEnum::TextColumn(data_column) => names.push(data_column.name.clone()),
            }
        }

        // Return names
        return names;
    }

    pub fn print(&self) {
        self.head();
        println!("............");
        self.tail();
    }

    pub fn print_full_table(&self) {
        unimplemented!()
    }

    /// Rename the column at given index to a new column name
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let mut dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// assert!(dataframe.has_column("Barcelona"));
    /// assert!(!dataframe.has_column("Oslo"));
    ///
    /// dataframe.rename_column(0, "Oslo");
    /// assert!(dataframe.has_column("Oslo"));
    /// ```
    ///
    /// # Errors
    ///
    /// This method does not throw any error. If there is not column at given index, it does nothing.
    /// Assume that given column is renamed, if a valid index is given.
    pub fn rename_column(&mut self, index: usize, column_name: &str) {
        if index < self.columns.len() {
            match &mut self.columns[index] {
                DataColumnEnum::IntColumn(data_column) => data_column.name = column_name.to_owned(),
                DataColumnEnum::FloatColumn(data_column) => {
                    data_column.name = column_name.to_owned()
                }
                DataColumnEnum::BoolColumn(data_column) => {
                    data_column.name = column_name.to_owned()
                }
                DataColumnEnum::TextColumn(data_column) => {
                    data_column.name = column_name.to_owned()
                }
            }
        }
    }

    /// Print the first 5 rows of the `Dataframe`.
    ///
    /// If the `Dataframe` has less then 5 rows, then it prints the whole `Dataframe`.
    /// Note that current implementation does not take into account the terminal width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// dataframe.head();
    /// ```
    ///
    /// # Errors
    ///
    /// Does create an error. If the dataframe is empty, then it will print a information string
    pub fn head(&self) {
        // Determine the number of rows to display (5 or fewer if not enough rows)
        let row_count = self.columns.get(0).map_or(0, |col| match col {
            DataColumnEnum::IntColumn(c) => c.size(),
            DataColumnEnum::FloatColumn(c) => c.size(),
            DataColumnEnum::BoolColumn(c) => c.size(),
            DataColumnEnum::TextColumn(c) => c.size(),
        });

        let rows_to_display = usize::min(5, row_count);

        if row_count == 0 {
            println!("Dataframe is empty.");
            return;
        }

        // Print column headers (names)
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::FloatColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::BoolColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::TextColumn(c) => print!("{:<15}", c.name),
            }
        }
        println!();

        // Print separator
        for _ in &self.columns {
            print!("{:-<15}", "_");
        }
        println!();

        // Print the rows
        for row_idx in 0..rows_to_display {
            for column in &self.columns {
                match column {
                    DataColumnEnum::IntColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::FloatColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::BoolColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::TextColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                }
            }
            println!(); // Move to the next line after each row
        }
    }

    /// Print the last 5 rows of the `Dataframe`.
    ///
    /// If the `Dataframe` has less then 5 rows, then it prints the whole `Dataframe`.
    /// Note that current implementation does not take into account the terminal width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// dataframe.tail();
    /// ```
    ///
    /// # Errors
    ///
    /// Does create an error. If the dataframe is empty, then it will print a information string
    pub fn tail(&self) {
        // Determine the number of rows to display (5 or fewer if not enough rows)
        let row_count = self.columns.get(0).map_or(0, |col| match col {
            DataColumnEnum::IntColumn(c) => c.size(),
            DataColumnEnum::FloatColumn(c) => c.size(),
            DataColumnEnum::BoolColumn(c) => c.size(),
            DataColumnEnum::TextColumn(c) => c.size(),
        });

        let start_row_index = usize::max(0, row_count - 5);
        let end_row_index = usize::max(5, row_count);

        if row_count == 0 {
            println!("Dataframe is empty.");
            return;
        }

        // Print column headers (names)
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::FloatColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::BoolColumn(c) => print!("{:<15}", c.name),
                DataColumnEnum::TextColumn(c) => print!("{:<15}", c.name),
            }
        }
        println!();

        // Print separator
        for _ in &self.columns {
            print!("{:-<15}", "_");
        }
        println!();

        // Print the rows
        for row_idx in start_row_index..end_row_index {
            for column in &self.columns {
                match column {
                    DataColumnEnum::IntColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::FloatColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::BoolColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                    DataColumnEnum::TextColumn(c) => {
                        if let Some(value) = c.get(row_idx) {
                            print!("{:<15}", value);
                        } else {
                            print!("{:<15}", "None");
                        }
                    }
                }
            }
            println!(); // Move to the next line after each row
        }
    }

    /// Prints information about columns in the `Dataframe`
    ///
    /// Print information about each column. For each column it prints the following information:
    /// - column name
    /// - type
    /// - counts of None
    /// - count of Some values
    /// - Total length of rows.
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
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// assert!(dataframe.memory_usage() == 4608);
    /// ```
    ///
    /// # Returns:
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

    /// Check if the `Dataframe` has rows.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// assert!(dataframe.has_rows());
    /// ```
    ///
    /// # Returns
    /// Returns true if there are rows, that could be None, rows.
    pub fn has_rows(&self) -> bool {
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(data_column) => {
                    if data_column.size() > 0 {
                        return true;
                    }
                }
                DataColumnEnum::FloatColumn(data_column) => {
                    if data_column.size() > 0 {
                        return true;
                    }
                }
                DataColumnEnum::BoolColumn(data_column) => {
                    if data_column.size() > 0 {
                        return true;
                    }
                }
                DataColumnEnum::TextColumn(data_column) => {
                    if data_column.size() > 0 {
                        return true;
                    }
                }
            }
        }

        // No column had any rows
        false
    }

    /// Check if the `Dataframe` any records.
    ///
    /// Record is a line with no `None` values. Use
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// assert!(dataframe.has_records());
    /// ```
    ///
    /// # Returns
    /// Returns true if there are rows, that could be None, rows.
    pub fn has_records(&self) -> bool {
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(data_column) => {
                    if data_column.iter_column().any(|x| x.is_some()) {
                        return true;
                    }
                }
                DataColumnEnum::FloatColumn(data_column) => {
                    if data_column.iter_column().any(|x| x.is_some()) {
                        return true;
                    }
                }
                DataColumnEnum::BoolColumn(data_column) => {
                    if data_column.iter_column().any(|x| x.is_some()) {
                        return true;
                    }
                }
                DataColumnEnum::TextColumn(data_column) => {
                    if data_column.iter_column().any(|x| x.is_some()) {
                        return true;
                    }
                }
            }
        }

        // No column had any records
        false
    }

    /// Check if the `Dataframe` has columns defined.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// assert!(dataframe.has_columns());
    /// ```
    ///
    /// Returns true if there is at least one `DataColumn`
    pub fn has_columns(&self) -> bool {
        self.columns.len() > 0
    }

    /// Check if a column with given column name exists in the `Dataframe`
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// assert!(dataframe.has_column("Barcelona"));
    /// assert!(!dataframe.has_column("Oslo"));
    /// ```
    ///
    /// # Returns
    ///
    /// True if there is a column that has the given column name
    pub fn has_column(&self, column_name: &str) -> bool {
        self.columns.iter().any(|col| match col {
            DataColumnEnum::IntColumn(int_col) => int_col.name == column_name,
            DataColumnEnum::FloatColumn(float_col) => float_col.name == column_name,
            DataColumnEnum::BoolColumn(bool_col) => bool_col.name == column_name,
            DataColumnEnum::TextColumn(text_col) => text_col.name == column_name,
        })
    }

    /// Drop the column with the given column name
    ///
    /// Method is not verbose, and therefor assume that the column was removed, or that it never existed.
    ///
    /// # Example
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let mut dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// assert!(dataframe.has_column("Barcelona"));
    ///
    /// dataframe.drop_column("Barcelona");
    /// assert!(!dataframe.has_column("Barcelona"));
    /// ```
    ///
    pub fn drop_column(&mut self, column_name: &str) {
        self.columns.retain(|col| match col {
            DataColumnEnum::IntColumn(int_col) => int_col.name != column_name,
            DataColumnEnum::FloatColumn(float_col) => float_col.name != column_name,
            DataColumnEnum::BoolColumn(bool_col) => bool_col.name != column_name,
            DataColumnEnum::TextColumn(text_col) => text_col.name != column_name,
        })
    }

    /// Add a new column to the `Dataframe`
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let mut dataframe = Dataframe::from_csv(path).unwrap();
    ///
    /// dataframe.add_column(vec![1, 2, 3, 4], "custom_index_column");
    /// ```
    pub fn add_column<T: ToString>(&mut self, list: Vec<T>, column_name: &str) {
        // Infer the column type based on the list values
        match Self::infer_column_type_from_vec(&list) {
            ColumnType::Integer => {
                // Parse values as i32 and collect them into a Vec<Option<i32>>
                let data: Vec<Option<i32>> = list
                    .iter()
                    .map(|value| {
                        value.to_string().parse::<i32>().ok() // Parse i32, return None on failure
                    })
                    .collect();
                // Add a new integer column to the dataframe
                let new_column = DataColumn::new(data, column_name.to_owned());
                self.columns.push(DataColumnEnum::IntColumn(new_column));
            }
            ColumnType::Float => {
                // Parse values as f64 and collect them into a Vec<Option<f64>>
                let data: Vec<Option<f32>> = list
                    .iter()
                    .map(|value| {
                        value.to_string().parse::<f32>().ok() // Parse f64, return None on failure
                    })
                    .collect();
                // Add a new float column to the dataframe
                let new_column = DataColumn::new(data, column_name.to_owned());
                self.columns.push(DataColumnEnum::FloatColumn(new_column));
            }
            ColumnType::Boolean => {
                // Parse values as bool and collect them into a Vec<Option<bool>>
                let data: Vec<Option<bool>> = list
                    .iter()
                    .map(|value| {
                        value.to_string().parse::<bool>().ok() // Parse bool, return None on failure
                    })
                    .collect();
                // Add a new boolean column to the dataframe
                let new_column = DataColumn::new(data, column_name.to_owned());
                self.columns.push(DataColumnEnum::BoolColumn(new_column));
            }
            ColumnType::Text => {
                // Treat all values as strings, convert to Vec<Option<String>>
                let data: Vec<Option<String>> = list
                    .into_iter()
                    .map(|value| {
                        Some(value.to_string()) // Convert T to string and wrap in Some
                    })
                    .collect();
                // Add a new text column to the dataframe
                let new_column = DataColumn::new(data, column_name.to_owned());
                self.columns.push(DataColumnEnum::TextColumn(new_column));
            }
        };
    }

    pub fn add_record(&self) {
        unimplemented!()
    }

    /// Get the `ColumnType` for a given column.
    ///
    /// # Example
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    /// use rustic_ml::data_utils::dataframe::ColumnType;
    ///
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// assert!(dataframe.has_column("Barcelona"));
    /// assert!(!dataframe.has_column("Oslo"));
    ///
    /// assert!(dataframe.get_column_type("Barcelona") == Some(ColumnType::Float));
    /// assert!(dataframe.get_column_type("Oslo") == None);        
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `Ǹone` if no column had given name, or the `ColumnType` of the column with the given name.
    ///
    pub fn get_column_type(&self, column_name: &str) -> Option<ColumnType> {
        // Iterate through each column and check if there is any column with the given name
        for column in &self.columns {
            match column {
                DataColumnEnum::IntColumn(data_column) => {
                    if data_column.name == column_name {
                        return Some(ColumnType::Integer);
                    }
                }
                DataColumnEnum::FloatColumn(data_column) => {
                    if data_column.name == column_name {
                        return Some(ColumnType::Float);
                    }
                }
                DataColumnEnum::BoolColumn(data_column) => {
                    if data_column.name == column_name {
                        return Some(ColumnType::Boolean);
                    }
                }
                DataColumnEnum::TextColumn(data_column) => {
                    if data_column.name == column_name {
                        return Some(ColumnType::Text);
                    }
                }
            }
        }

        // No column name match, return None
        None
    }

    /// Extract a single feature of floats into a `Vec<Option<f32>>`
    ///
    /// Creates a clone of the column. Values within the vector might be None.
    /// Use the column name to identify the column that will be extracted.
    pub fn float_feature(&self, column_name: &str) -> Option<Vec<Option<f32>>> {
        // Return none if there is no vector with
        if !self.has_column(column_name) {
            return None;
        }

        // Iterate through the columns until the correct one is found
        for column in &self.columns {
            match column {
                DataColumnEnum::FloatColumn(float_col) => {
                    if float_col.name == column_name {
                        return Some(float_col.extract());
                    }
                }
                _ => continue,
            }
        }

        // The desired column was not a float value
        None
    }

    /// Extract two sets of features into a single vector of tuples (`Vec<Option<(f32, f32)>>`).
    ///
    /// Creates a clone of the column. Values within the vector might be `None`.
    /// A row in the vector is `None`, if one of the vectors are none.
    /// Use the column name to identify the column that will be extracted.
    ///
    ///
    /// # Returns
    ///
    /// Returns a `Vec<Option<(f32, f32)>>` created from the two features.
    /// Returns `None` if the two feature vectors are not the same length or of any vector did not exist.
    pub fn float_features(
        &self,
        first_column_name: &str,
        second_column_name: &str,
    ) -> Option<Vec<Option<(f32, f32)>>> {
        if !self.has_column(first_column_name) || !self.has_column(second_column_name) {
            return None;
        }

        let mut first_column: Option<Vec<Option<f32>>> = None;
        let mut second_column: Option<Vec<Option<f32>>> = None;

        for column in &self.columns {
            match column {
                DataColumnEnum::FloatColumn(float_col) => {
                    if float_col.name == first_column_name {
                        first_column = Some(float_col.extract());
                    } else if float_col.name == second_column_name {
                        second_column = Some(float_col.extract());
                    }
                }
                _ => continue,
            }
        }

        // Return none if one of the columns are none
        if first_column.is_none() || second_column.is_none() {
            return None;
        }

        // Merge the two columns
        let merged_column: Option<Vec<Option<(f32, f32)>>> = match (first_column, second_column) {
            (Some(first_vec), Some(second_vec)) => {
                // Ensure the lengths of both vectors are the same
                if first_vec.len() == second_vec.len() {
                    // Combine the two vectors element-wise
                    Some(
                        first_vec
                            .into_iter()
                            .zip(second_vec.into_iter())
                            .map(|(first_opt, second_opt)| {
                                match (first_opt, second_opt) {
                                    (Some(first_val), Some(second_val)) => {
                                        Some((first_val, second_val))
                                    }
                                    _ => None, // If either is None, return None
                                }
                            })
                            .collect(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        };
        merged_column
    }

    /// Get the value at given column and given row index.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    /// 
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// 
    /// assert!(dataframe.at_str("Barcelona", 2) == Some("1497.61".to_string()));
    /// ```
    ///
    ///
    /// # Returns
    ///
    /// The value as a `String` or `None` if:
    /// - there was no column with that name
    /// - the given row index was out of bounce
    /// - the value at that entry was None
    pub fn at_str(&self, column_name: &str, row_index: usize) -> Option<String> {
        if self.has_column(column_name) {
            for column in &self.columns {
                match column {
                    DataColumnEnum::IntColumn(data_column) => {
                        if data_column.name == column_name {
                            return data_column.get(row_index).map(|v| v.to_string());
                        }
                    }
                    DataColumnEnum::FloatColumn(data_column) => {
                        if data_column.name == column_name {
                            return data_column.get(row_index).map(|v| v.to_string());
                        }
                    }
                    DataColumnEnum::BoolColumn(data_column) => {
                        if data_column.name == column_name {
                            return data_column.get(row_index).map(|v| v.to_string());
                        }
                    }
                    DataColumnEnum::TextColumn(data_column) => {
                        if data_column.name == column_name {
                            return data_column.get(row_index).map(|v| v.to_string());
                        }
                    }
                }
            }
        }

        // No match none is returned
        None
    }

    /// Get the value at given the index of the item.
    /// Index 0 is the item in the first row, first column. 
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use rustic_ml::data_utils::dataframe::Dataframe;
    /// 
    /// let path = String::from("./datasets/european_cities.csv");
    /// let dataframe = Dataframe::from_csv(path).unwrap();
    /// 
    /// assert!(dataframe.at_index_str(2) == Some("1497.61".to_string()));
    /// ```
    ///
    ///
    /// # Returns
    ///
    /// The value as a `String` or `None` if:
    /// - the given index was out of bounce
    /// - the value at that entry was None
    pub fn at_index_str(&self, index: usize) -> Option<String> {
        let column_index = index % self.columns.len();
        let row_index = index / self.columns.len();

        match &self.columns[column_index]{
            DataColumnEnum::IntColumn(data_column) => {
                if row_index > data_column.size(){
                    return None;
                }
                return data_column.get(row_index).map(|v| v.to_string());
            },
            DataColumnEnum::FloatColumn(data_column) => {
                if row_index > data_column.size(){
                    return None;
                }
                return data_column.get(row_index).map(|v| v.to_string());
            },
            DataColumnEnum::BoolColumn(data_column) => {
                if row_index > data_column.size(){
                    return None;
                }
                return data_column.get(row_index).map(|v| v.to_string());
            },
            DataColumnEnum::TextColumn(data_column) => {
                if row_index > data_column.size(){
                    return None;
                }
                return data_column.get(row_index).map(|v| v.to_string());
            },
        }
    }
}
