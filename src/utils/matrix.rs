use std::ops::Range;

/// Matrix implementation
/// 
/// A mathematical data structure. 
/// Read more about matrices here: 
/// <https://en.wikipedia.org/wiki/Matrix_(mathematics)>
pub struct Matrix{
    pub data: Vec<f32>,
    pub rows: usize, 
    pub cols: usize
}

impl Matrix {
    /// Create a new empty matrix
    /// 
    /// Uses rows and cols for defining the size of the matrix.
    /// All values within the matrix is 0.0.
    /// Returns an instance of Matrix.
    pub fn new(rows: usize, cols: usize) -> Self{
        let data = vec![0.0; rows*cols];
        Matrix { data, rows, cols }
    }

    /// Create a new matrix based on given data
    /// 
    /// Given the amount of rows and columns and the vector of data, it creates a new instance of the matrix.
    /// Creates a matrix based on the given amount of columns. Will add any missing values as default value 0.0. 
    /// Makes sure that the Matrix has completed rows. 
    pub fn from_vec(cols: usize, mut data: Vec<f32>) -> Self{
        let mut rows = data.len() / cols; 
        let missing_values = data.len() % cols;

        if missing_values > 0{
            for _ in 0..missing_values{
                data.push(0.0);
            }
            rows += 1;
            
        }

        Matrix{rows, cols, data}
    }
    
    /// Get an item from the Matrix 
    /// 
    /// Given the row and column of the item, retrieve a reference to the item.
    /// If there is not item, or if the row anc column given was to high, it returns None. 
    pub fn get(&self, row: usize, col: usize) -> Option<&f32>{
        let index = row*self.cols + col;
        if row >= self.rows || col >= self.cols || index >= self.data.len() {
            return None;
        }
        return Some(&self.data[index]);
    }

    /// Get an item from the Matrix (as mutable reference)
    /// 
    /// Given the row and column of the item, retrieve a mutable reference to the item.
    /// If there is not item, or if the row anc column given was to high, it returns None. 
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f32>{
        let index = row*self.cols + col;
        if row >= self.rows || col >= self.cols || index >= self.data.len() {
            return None;
        }
        return Some(&mut self.data[index]);
    }

    /// Get a single row from the Matrix
    /// 
    /// Takes the index of the row, and returns a vector of all the values in the given row index.
    /// Returns None if the index is out of range. 
    pub fn get_row(&self, index:usize) -> Option<Vec<f32>>{
        if index >= self.rows{
            return None;
        }

        let start_index = index*self.cols; 
        let end_index = start_index + self.cols; 
        Some(self.data[start_index..end_index].to_vec())
    }

    /// Get a single row from the Matrix (as a mutable reference)
    /// 
    /// Takes the index of the row, and returns a vector of all the values in the given row index.
    /// Returns None if the index is out of range. 
    pub fn get_row_mut(&mut self, index:usize) -> Option<&mut [f32]>{
        if index >= self.rows{
            return None;
        }

        let start_index = index*self.cols; 
        let end_index = start_index + self.cols; 
        Some(&mut self.data[start_index..end_index])
    }

    /// Get a single row from the Matrix (as slice)
    /// 
    /// Takes the index of the row and returns a slice from the Matrix.
    /// Returns None if the index is out of range. 
    pub fn get_row_as_slice(&self, index: usize) -> Option<&[f32]> {
        if index >= self.rows {
            return None;
        }

        let start_index = index * self.cols;
        let end_index = start_index + self.cols;
        Some(&self.data[start_index..end_index])
    }

    /// Get a single col from the Matrix
    /// 
    /// Takes the index of the col, and returns a vector of all the values in the given col index.
    /// Returns None if the index is out of range. 
    pub fn get_col(&self, index:usize) -> Option<Vec<f32>>{
        if index >= self.cols{
            return None;
        }

        let mut result:Vec<f32> = Vec::with_capacity(self.cols);
        for i in 0..self.rows{
            result.push(self.data[i * self.cols + index])
        }
        
        Some(result)
    }

    /// Get a single col from the Matrix (as a mutable reference)
    /// 
    /// Takes the index of the col, and returns a vector of all the values in the given col index.
    /// This code uses `unsafe`, and is therefor not recommended. Use `get_mut` if possible. 
    /// Returns None if the index is out of range.
    pub fn get_col_mut(&mut self, index:usize) -> Option<Vec<&mut f32>>{
        if index >= self.cols{
            return None;
        }

        let mut result:Vec<&mut f32> = Vec::with_capacity(self.cols);
        let data_ptr = self.data.as_mut_ptr();
        for i in 0..self.rows {
            unsafe {
                let elem = &mut *data_ptr.add(i * self.cols + index);
                result.push(elem);
            }
        }

        Some(result)
    }

    /// Get a single row from the Matrix (as slice)
    /// 
    /// Takes the index of the row and returns a slice from the Matrix.
    /// Returns None if the index is out of range. 
    pub fn get_col_as_slice(&self, index:usize) -> Option<Vec<&f32>>{
        if index >= self.cols{
            return None;
        }

        let mut result:Vec<&f32> = Vec::with_capacity(self.cols);
        for i in 0..self.rows{
            result.push(&self.data[i * self.cols + index])
        }
        
        Some(result)
    }

    /// Get the shape of the Matrix.
    /// 
    /// Format of the string is "ROWSxCOLUMNS". Created with the format macro. 
    pub fn shape(&self) -> String{
        format!("{}x{}", self.rows, self.cols)
    }

    /// Get a sub mutable matrix of the given matrix 
    /// 
    /// Returns None if the given rage did not fit the dimensions of the Matrix 
    pub fn get_submatrix(self, rows:Range<usize>, cols:Range<usize>) -> Option<Matrix>{
        //TODO: Get mutable vs reference to this matrix (?) 
        unimplemented!()
    }

    pub fn dot_product(self, m: Matrix) -> Option<f32>{
        unimplemented!()
    }



    
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::Matrix;

    #[test]
    fn test_new_matrix_constructor(){
        let rows = 3; 
        let cols = 2; 

        let matrix: Matrix = Matrix::new(rows, cols);

        let data_expected = vec![0.0; rows*cols]; 

        assert_eq!(matrix.data, data_expected);
        assert_eq!(matrix.data.len(), rows*cols);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.rows, rows);
    }  

    #[test]
    fn test_new_matrix_constructor_from_vec(){
        // Testing a 2x4 matrix 
        let rows = 2; 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        assert_eq!(matrix.data, data);
        assert_eq!(matrix.data.len(), rows*cols);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.rows, rows);
    }

    #[test]
    fn test_new_matrix_constructor_from_vec_missing_values(){
        // Testing a 2x4 matrix 
        let rows = 2; 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0];

        let matrix: Matrix = Matrix::from_vec(cols, data);

        // Standard Check that the matrix was correctly created 
        assert_eq!(matrix.data, vec![1.0,2.0,3.0,4.0,5.0,6.0, 0.0, 0.0]);
        assert_eq!(matrix.data.len(), rows*cols);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.rows, rows);

        // Check if the last three values:
        assert_eq!(matrix.data[5], 6.0);
        assert_eq!(matrix.data[6], 0.0);
        assert_eq!(matrix.data[7], 0.0);


    } 

    #[test]
    fn test_get_matrix_positive(){
        // Testing a 2x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,
                                 5.0,6.0,7.0,8.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Test value 3 at first row third column
        let three = matrix.get(0, 2);
        match three {
            Some(val) =>{
                assert_eq!(val, &3.0);
            }, 
            None =>{
                panic!("Value at (0, 2) should exist")
            }
            
        }

        // Test value 6 at second row second column
        let six = matrix.get(1, 1);
        match six {
            Some(val) =>{
                assert_eq!(val, &6.0);
            }, 
            None =>{
                panic!("Value at (1,1) should exist")
            }
            
        }

        // Test value 8 at second row fourth column 
        let eight = matrix.get(1, 3);
        match eight {
            Some(val) =>{
                assert_eq!(val, &8.0);
            }, 
            None =>{
                panic!("Value at (1, 3) should exist")
            }
            
        }
        
    }

    
    #[test]
    fn test_get_matrix_negative(){
        // Testing a 2x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Test value that is over the given 
        let non_valid_item = matrix.get(1, 4);
        match non_valid_item {
            Some(val) =>{
                panic!("Value at (1, 4) should not exist: {}", val)
            }, 
            None =>{
                assert!(true)
            }
            
        }
    }

    #[test]
    fn test_get_mut_matrix_positive(){
        // Testing a 2x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,
                                 5.0,6.0,7.0,8.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Retrieve a value 
        let three: &mut f32 = match matrix.get_mut(0, 2){
            Some(val) =>{
                assert_eq!(val, &3.0);
                val
            }, 
            None =>{
                panic!("Value at (0, 2) should exist")
            }
            
        };
        
        // Increment the value
        *three += 1.0; 

        // Check if the increment was successful
        assert_eq!(*three, 4.0);

        // Check the value in the matrix
        match matrix.get(0, 2) {
            Some(val) => assert_eq!(*val, 4.0),
            None => panic!("Value at (0, 2) should exist"),
        }

        // Test value 6 at second row second column
        let six: &mut f32 = match matrix.get_mut(1, 1) {
            Some(val) =>{
                assert_eq!(val, &6.0);
                val
            }, 
            None =>{
                panic!("Value at (1, 1) should exist")
            }
            
        };

        // Increment the value
        *six += 1.0; 

        // Check if the increment was successful
        assert_eq!(*six, 7.0);

        // Check the value in the matrix
        match matrix.get(1, 1) {
            Some(val) => assert_eq!(*val, 7.0),
            None => panic!("Value at (0, 2) should exist"),
        }

        // Test value 8 at second row fourth column 
        let eight: &mut f32 = match matrix.get_mut(1, 3){
            Some(val) =>{
                assert_eq!(val, &8.0);
                val
            }, 
            None =>{
                panic!("Value at (1, 3) should exist")
            }
            
        };

        // Increment the value
        *eight += 1.0; 

        // Check if the increment was successful
        assert_eq!(*eight, 9.0);

        // Check the value in the matrix
        match matrix.get(1, 3) {
            Some(val) => assert_eq!(*val, 9.0),
            None => panic!("Value at (1, 3) should exist"),
        }


        // Check that the matrix has the correct values 
        // Values incremented: 3, 6, 8 => 4, 7, 9
        let expected_data = vec![1.0,2.0,4.0,4.0,
                                            5.0,7.0,7.0,9.0];

        assert_eq!(matrix.data, expected_data);
        
    }

    #[test]
    fn test_get_mut_matrix_negative(){
        // Testing a 2x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Test value that is over the given 
        let non_valid_item = matrix.get_mut(1, 4);
        match non_valid_item {
            Some(val) =>{
                assert_eq!(&0.0, val, "Expected None, got value");
            }, 
            None =>{
                assert!(true)
            }
            
        }
    }

    #[test]
    fn test_get_row_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        let first_row: &[f32] = &vec![1.0, 2.0, 3.0, 4.0];
        let second_row: &[f32] = &vec![5.0, 6.0, 7.0, 8.0];
        let third_row: &[f32] = &vec![9.0, 10.0, 11.0, 12.0]; 

        // First row
        match matrix.get_row(0) {
            Some(values) => assert_eq!(values, first_row),
            None => panic!("Expected row of index 0 to have values"),
        }

        // Second row
        match matrix.get_row(1) {
            Some(values) => assert_eq!(values, second_row),
            None => panic!("Expected row of index 1 to have values"),
        }

        // Third row
        match matrix.get_row(2) {
            Some(values) => assert_eq!(values, third_row),
            None => panic!("Expected row of index 2 to have values"),
        }

    }

    #[test]
    fn test_get_row_matrix_negative(){

        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_row(3){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }

    #[test]
    fn test_get_row_mut_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        let first_row: &[f32] = &vec![1.0, 2.0, 3.0, 4.0];
        let second_row: &[f32] = &vec![5.0, 6.0, 7.0, 8.0];
        let third_row: &[f32] = &vec![9.0, 10.0, 11.0, 12.0]; 

        // First row
        let first_row_mutable: &mut [f32] = match matrix.get_row_mut(0) {
            Some(values) => {
                assert_eq!(values, first_row);
                values
            },
            None => panic!("Expected row of index 0 to have values"),
        };

        // Changing some values 
        first_row_mutable[0] = 2.0;
        first_row_mutable[1] += 12.0;
        first_row_mutable[2] -= 1.0;

        // Second row
        let second_row_mutable: &mut [f32] = match matrix.get_row_mut(1) {
            Some(values) => {
                assert_eq!(values, second_row);
                values
            },
            None => panic!("Expected row of index 1 to have values"),
        };

        // Changing some values 
        second_row_mutable[0] = 0.0;
        second_row_mutable[1] += 4.0;
        second_row_mutable[2] -= 2.0;

        // Third row
        let third_row_mutable: &mut [f32] = match matrix.get_row_mut(2) {
            Some(values) => {
                assert_eq!(values, third_row);
                values
            },
            None => panic!("Expected row of index 2 to have values"),
        };

        // Changing some values 
        third_row_mutable[0] = 40.0;
        third_row_mutable[1] += 0.0;
        third_row_mutable[2] -= 30.0;


        // Check that the matrix is as expected
        assert_eq!(matrix.get_row(0), Some(vec![2.0, 14.0, 2.0, 4.0]));

        assert_eq!(matrix.get_row(1), Some(vec![0.0, 10.0, 5.0, 8.0]));

        assert_eq!(matrix.get_row(2), Some(vec![40.0, 10.0, -19.0, 12.0]))


    }

    #[test]
    fn test_get_row_mut_matrix_negative(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_row_mut(3){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }


    #[test]
    fn test_get_row_as_slice_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        let first_row: &[f32] = &vec![1.0, 2.0, 3.0, 4.0];
        let second_row: &[f32] = &vec![5.0, 6.0, 7.0, 8.0];
        let third_row: &[f32] = &vec![9.0, 10.0, 11.0, 12.0]; 

        // First row
        match matrix.get_row_as_slice(0) {
            Some(values) => {
                assert_eq!(values, first_row);
                values
            },
            None => panic!("Expected row of index 0 to have values"),
        };

        // Second row
        match matrix.get_row_mut(1) {
            Some(values) => {
                assert_eq!(values, second_row);
                values
            },
            None => panic!("Expected row of index 1 to have values"),
        };
        // Third row
        match matrix.get_row_mut(2) {
            Some(values) => {
                assert_eq!(values, third_row);
                values
            },
            None => panic!("Expected row of index 2 to have values"),
        };
    }

    #[test]
    fn test_get_row_as_slice_matrix_negative(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_row_as_slice(3){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }


    #[test]
    fn test_get_col_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        let first_col: &[f32] = &vec![1.0, 5.0, 9.0];
        let second_col: &[f32] = &vec![2.0, 6.0, 10.0];
        let third_col: &[f32] = &vec![3.0, 7.0, 11.0]; 

        // First col
        match matrix.get_col(0) {
            Some(values) => assert_eq!(values, first_col),
            None => panic!("Expected col of index 0 to have values"),
        }

        // Second col
        match matrix.get_col(1) {
            Some(values) => assert_eq!(values, second_col),
            None => panic!("Expected col of index 1 to have values"),
        }

        // Third col
        match matrix.get_col(2) {
            Some(values) => assert_eq!(values, third_col),
            None => panic!("Expected col of index 2 to have values"),
        }

    }

    #[test]
    fn test_get_col_matrix_negative(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_col(4){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }

    #[test]
    fn test_get_col_mut_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Test first column
        if let Some(mut col) = matrix.get_col_mut(0) {
            assert_eq!(*col[0], 1.0);
            assert_eq!(*col[1], 5.0);
            assert_eq!(*col[2], 9.0);
            *col[0] = 10.0;
            *col[1] = 11.0;
            *col[2] = 12.0;
        } else {
            panic!("Expected column 0 to have values");
        }

        // Test second column
        if let Some(mut col) = matrix.get_col_mut(1) {
            assert_eq!(*col[0], 2.0);
            assert_eq!(*col[1], 6.0);
            assert_eq!(*col[2], 10.0);
            *col[0] = 20.0;
            *col[1] = 21.0;
            *col[2] = 22.0;
        } else {
            panic!("Expected column 1 to have values");
        }

        // Test third column
        if let Some(mut col) = matrix.get_col_mut(2) {
            assert_eq!(*col[0], 3.0);
            assert_eq!(*col[1], 7.0);
            assert_eq!(*col[2], 11.0);
            *col[0] = 30.0;
            *col[1] = 31.0;
            *col[2] = 32.0;
        } else {
            panic!("Expected column 2 to have values");
        }

        // Verify the matrix after modification
        assert_eq!(matrix.data, vec![
            10.0, 20.0, 30.0, 4.0,
            11.0, 21.0, 31.0, 8.0,
            12.0, 22.0, 32.0, 12.0,
        ]);


    }

    #[test]
    fn test_get_col_mut_matrix_negative(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let mut matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_col_mut(4){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }

    #[test]
    fn test_get_col_as_slice_matrix_positive(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        let first_col: Vec<&f32> = vec![&1.0, &5.0, &9.0];
        let second_col: Vec<&f32> = vec![&2.0, &6.0, &10.0];
        let third_col: Vec<&f32> = vec![&3.0, &7.0, &11.0]; 

        // First col
        match matrix.get_col_as_slice(0) {
            Some(values) => assert_eq!(values, first_col),
            None => panic!("Expected col of index 0 to have values"),
        }

        // Second col
        match matrix.get_col_as_slice(1) {
            Some(values) => assert_eq!(values, second_col),
            None => panic!("Expected col of index 1 to have values"),
        }

        // Third col
        match matrix.get_col_as_slice(2) {
            Some(values) => assert_eq!(values, third_col),
            None => panic!("Expected col of index 2 to have values"),
        }

    }

    #[test]
    fn test_get_col_as_slice_matrix_negative(){
        // Testing a 3x4 matrix 
        let cols = 4; 

        let data:Vec<f32> = vec![1.0,  2.0,  3.0,  4.0,
                                 5.0,  6.0,  7.0,  8.0,
                                 9.0, 10.0, 11.0, 12.0];

        let matrix: Matrix = Matrix::from_vec(cols, data.clone());

        // Get a row that does not exist => row nr.4 does not exist 
        match matrix.get_col_as_slice(4){
            Some(values) => panic!("Expected no values in row 4, got: {:?}", values),
            None => (), //Success 
        }
    }

    #[test]
    fn test_shape_matrix(){
        let matrix_one_dim: Matrix = Matrix::new(1, 1);
        let matrix_two_dim: Matrix = Matrix::new(2, 2);
        let matrix_three_dim: Matrix = Matrix::new(3, 3);
        let matrix_odd: Matrix = Matrix::new(3, 5);


        assert_eq!(matrix_one_dim.shape(), "1x1");
        assert_eq!(matrix_two_dim.shape(), "2x2");
        assert_eq!(matrix_three_dim.shape(), "3x3");
        assert_eq!(matrix_odd.shape(), "3x5");
    }
}