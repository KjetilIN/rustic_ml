use std::{fmt::Display, ops::Range};

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


/// Custom Error type for a `Matrix` operation
#[derive(Debug)]
pub enum MatrixError {
    /// An error that occurs when a operation requires that two matrices have the exact same shape
    ShapeMismatch{
        /// Shape of the matrix that called the matrix operation function.
        first_matrix_shape: String,

        /// Shape of the matrix that was given in a matrix operation function.
        second_matrix_shape: String
    },  

    /// Operation lead to division by 0
    DivideByZero,

    /// Matrix operation required a range, but the given range lead to an error,
    IllegalRange(String),

    /// Matrix multiplication with two given matrixes sizes (mxn) and (qxp)
    /// The columns (n) must equal rows (q) => n == q
    MatrixMultiply,
}

// For printing the error of the matrix
impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::ShapeMismatch { first_matrix_shape, second_matrix_shape } => {
                write!(f, "The matrix with shape ({first_matrix_shape}) does not match the shape of the given matrix ({second_matrix_shape})")
            },
            MatrixError::DivideByZero => {
                write!(f, "Given matrix or number lead to division by zero")
            },
            MatrixError::IllegalRange(val) =>{
                write!(f, "Illegal Range Given: {val}")
            },
            MatrixError::MatrixMultiply =>{
                write!(f, "Illegal to multiply the two given matrixes.")
            }
        }
    }
}

// Making the MatrixError an Error
impl std::error::Error for MatrixError {}

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

    /// Create a identity matrix from the given order 
    /// 
    /// Creates an empty matrix from the given order. Sets the value 1.0 on the main diagonal of the matrix
    /// Read more about Identity Matrix: <https://en.wikipedia.org/wiki/Identity_matrix>
    pub fn identity(order: usize)-> Self{
        let mut data = vec![0.0; order*order];

        for i in 0..order{
            data[i*order+i] = 1.0; 
        }

        Matrix{data, rows: order, cols: order}
    }

    /// Create a new matrix based on given data
    /// 
    /// Given the amount of rows and columns and the vector of data, it creates a new instance of the matrix.
    /// Creates a matrix based on the given amount of columns. Will add any missing values as default value 0.0. 
    /// Makes sure that the Matrix has completed rows. 
    pub fn from_vec(cols: usize, mut data: Vec<f32>) -> Self{
        let missing_values = cols - (data.len() % cols);

        if missing_values < cols {
            for _ in 0..missing_values {
                data.push(0.0);
            }
        }

        let rows = data.len() / cols;
        Matrix { rows, cols, data }
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


    /// Get the numbers across the main diagonal if the `rows == cols`.
    /// 
    /// Returns a new vector of all the numbers across the diagonal
    /// Returns none if the amount of rows is not equal to the amount of columns
    pub fn get_diagonal(&self) -> Option<Vec<f32>>{
        if self.rows != self.cols{
            return None;
        }

        let mut diagonal: Vec<f32> = Vec::new();

        for i in 0..self.rows{
            diagonal.push(self.data[i*self.cols+i]); 
        }

        Some(diagonal)
    }

    /// Get the numbers across the main diagonal if the `rows == cols` (as slice)
    /// 
    /// Returns a new vector of all the numbers across the diagonal
    /// Returns none if the amount of rows is not equal to the amount of columns
    pub fn get_diagonal_as_slice(&self) -> Option<Vec<&f32>>{
        if self.rows != self.cols{
            return None;
        }

        let mut diagonal: Vec<&f32> = Vec::new();

        for i in 0..self.rows{
            diagonal.push(&self.data[i*self.cols+i]); 
        }

        Some(diagonal)
    }

    /// Get the numbers across the cross diagonal if the `rows == cols`.
    /// 
    /// The cross diagonal is the diagonal from top right corner to the bottom left corner of the matrix. 
    /// Not to be mistaken with the main diagonal. 
    /// Returns a new vector of all the numbers across the diagonal
    /// Returns none if the amount of rows is not equal to the amount of columns
    pub fn get_cross_diagonal(&self) -> Option<Vec<f32>>{
        if self.rows != self.cols{
            return None;
        }

        let mut diagonal: Vec<f32> = Vec::new();

        for i in 0..self.rows{
            let col_index = self.cols - i - 1; 
            diagonal.push(self.data[i*self.cols+col_index]); 
        }

        Some(diagonal)
    }

    /// Get the numbers across the cross diagonal if the `rows == cols` (as a slice)
    /// 
    /// The cross diagonal is the diagonal from top right corner to the bottom left corner of the matrix. 
    /// Not to be mistaken with the main diagonal. 
    /// Returns a new vector of all the numbers across the diagonal
    /// Returns none if the amount of rows is not equal to the amount of columns
    pub fn get_cross_diagonal_as_slice(&self) -> Option<Vec<&f32>>{
        if self.rows != self.cols{
            return None;
        }

        let mut diagonal: Vec<&f32> = Vec::new();

        for i in 0..self.rows{
            let col_index = self.cols - i - 1; 
            diagonal.push(&self.data[i*self.cols+col_index]); 
        }

        Some(diagonal)
    }

    /// Get the shape of the Matrix.
    /// 
    /// Format of the string is "ROWSxCOLUMNS". Created with the format macro. 
    pub fn shape(&self) -> String{
        format!("{}x{}", self.rows, self.cols)
    }

    /// Reshapes the matrix to the new shape based on the given new rows and columns
    /// 
    /// If the new matrix is bigger than the original, then the method adds default values: `0.0`.
    /// If the new matrix is smaller than the original, then the method removes the extra data at the end of the matrix. 
    pub fn reshape(&mut self, new_rows: usize, new_cols: usize){
        let current_size = self.rows * self.cols;
        let new_size = new_rows * new_cols;

        // Remove or add data to the matrix depending on the size difference
        if new_size > current_size {
            self.data.resize(new_size, 0.0);
        } else if new_size < current_size {
            self.data.truncate(new_size);
        }

        // Update the dimensions
        self.rows = new_rows;
        self.cols = new_cols;
    }

    /// Get a sub mutable matrix of the given matrix 
    /// 
    /// Ranges start from 0 and are not inclusive of the end value.
    /// Returns a `Result` with either the submatrix (`Matrix`) or the matrix error (`MatrixError`)
    /// Error that can occur is when the ranges are bigger than the shape of the `Matrix`
    pub fn submatrix(&self, rows:Range<usize>, cols:Range<usize>) -> Result<Matrix, MatrixError>{
        // Check if the ranges fit the matrix 
        if rows.end > self.rows || cols.end > self.cols{
            return Err(MatrixError::IllegalRange(format!("Did not match the shape: {}", self.shape())));
        }

        // Check that the range is sequential and length at least 1 
        if rows.start >= rows.end || cols.start >= cols.end{
            return Err(MatrixError::IllegalRange("Range must start go from low to high value, and length at least 1".to_string()));
        }
        
        // Format the data for the submatrix
        let mut data: Vec<f32> = Vec::with_capacity(rows.len()*cols.len());
        for row in rows.clone(){
            for col in cols.clone(){
                data.push(self.data[row*self.rows + col].clone())
            }
        }

        Ok(Matrix{
            data,
            rows: rows.end - rows.start,
            cols: cols.end - cols.start
        })
    }

    pub fn submatrix_as_slice(&self, rows:Range<usize>, cols:Range<usize>) -> Result<Matrix, MatrixError>{
        //TODO: Ignoring for now, not use case clear 
        unimplemented!()
    }

    /// Multiply two matrices
    /// 
    /// Condition for multiplication of matrices: 
    /// - Given matrix `(mxn)` and `(qxp)`
    /// - Columns `n` must equal rows `q`
    /// 
    /// Returns `Result` based on if this condition is met 
    pub fn multiply(&self, mat: &Matrix) -> Result<Matrix, MatrixError>{

        // Check the matrix condition 
        if self.cols != mat.rows{
            return Err(MatrixError::MatrixMultiply);
        }

        // Create the new sum 
        let mut matrix = Matrix::new(self.rows, mat.cols);
        
        // Perform matrix multiplication
        for i in 0..self.rows {
            for j in 0..mat.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * mat.data[k * mat.cols + j];
                }
                matrix.data[i * mat.cols + j] = sum;
            }
        }

        Ok(matrix)        
    }

    pub fn cross_product(&mut self, mat: &Matrix) -> Result<Matrix, MatrixError>{
        unimplemented!()
    }

    
    pub fn transpose(&mut self){
        unimplemented!()
    }

    pub fn get_transposed(&self) -> Matrix{
        unimplemented!()
    }

    pub fn inverse(&mut self){
        unimplemented!()
    }

    pub fn get_inverse(&self)-> Matrix{
        unimplemented!()
    }

    pub fn is_vector(&self) -> bool{
        unimplemented!()
    }

    pub fn sub_f(&mut self, numb: f32){
        for item in self.data.iter_mut(){
            *item -= numb;
        };
    }

    pub fn sub_m(&mut self, mat: &Matrix) -> Result<(), MatrixError>{
        // Check shape
        if self.shape() != mat.shape(){
            return Err(MatrixError::ShapeMismatch { 
                first_matrix_shape: self.shape(), 
                second_matrix_shape: mat.shape() 
            });
        }
        unimplemented!()
    }

    pub fn add_f(&mut self, numb: f32){
        for item in self.data.iter_mut(){
            *item += numb;
        };
    }

    pub fn add_m(&mut self, mat: &Matrix) -> Result<(), MatrixError>{
        // Check shape
        if self.shape() != mat.shape(){
            return Err(MatrixError::ShapeMismatch { 
                first_matrix_shape: self.shape(), 
                second_matrix_shape: mat.shape() 
            });
        }
        unimplemented!()
    }

    pub fn scale_f(mut self, numb: f32){
        for item in self.data.iter_mut(){
            *item *= numb;
        };
    }

    pub fn scale_m(&mut self, mat: &Matrix) -> Result<(), MatrixError>{
        // Check shape
        if self.shape() != mat.shape(){
            return Err(MatrixError::ShapeMismatch { 
                first_matrix_shape: self.shape(), 
                second_matrix_shape: mat.shape() 
            });
        }
        unimplemented!()
    }

    pub fn div_f(mut self, numb: f32) -> Result<(), MatrixError>{
        // Check for divide by 0 error
        if numb == 0.0{
            return Err(MatrixError::DivideByZero);
        }

        for item in self.data.iter_mut(){
            *item /= numb;
        };

        Ok(())
    }

    pub fn div_m(&mut self, mat: &Matrix) -> Result<(), MatrixError>{
        // Check shape
        if self.shape() != mat.shape(){
            return Err(MatrixError::ShapeMismatch { 
                first_matrix_shape: self.shape(), 
                second_matrix_shape: mat.shape() 
            });
        }

        // Check for divide by 0 error
        for i in &mat.data{
            if i == &0.0{
                return Err(MatrixError::DivideByZero);
            }
        }

        unimplemented!()
    }

    pub fn mod_f(&mut self, numb: f32){
        for item in self.data.iter_mut(){
            *item %= numb;
        };
    }

    pub fn mod_m(&mut self, mat: &Matrix) -> Result<(), MatrixError>{
        // Check shape
        if self.shape() != mat.shape(){
            return Err(MatrixError::ShapeMismatch { 
                first_matrix_shape: self.shape(), 
                second_matrix_shape: mat.shape() 
            });
        }

        unimplemented!()
    }

    pub fn is_orthogonal(&self) -> bool{
        unimplemented!()
    }

    pub fn det_2x2(){
        unimplemented!()
    }

    pub fn det_3x3(){
        unimplemented!()
    }
    
}

