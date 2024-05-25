use std::ops::Range;

/// Matrix implementation
/// 
/// A mathematical data structure used.
/// Read more about Matrixes here: 
/// https://en.wikipedia.org/wiki/Matrix_(mathematics)
pub struct Matrix{
    pub data: Vec<Vec<f64>>,
    pub rows: usize, 
    pub cols: usize
}

impl Matrix {
    /// Create a new empty matrix
    /// 
    /// Uses rows and cols for defining the size of the matrix.
    /// All values within the matrix is 0.0.
    /// Returns an instance of Matrix 
    pub fn new(rows: usize, cols: usize) -> Self{
        let data = vec![vec![0.0; cols]; rows];
        Matrix { data, rows, cols }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Self{
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
    }
 
    pub fn get(&self, row: usize, col: usize) -> Option<&f64>{
        unimplemented!()
    }

    // Mutable Reference 
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f64>{
        unimplemented!()
    }

    /// Get a sub mutable matrix of the given matrix 
    /// 
    /// Returns None if the given rage did not fit the dimensions of the Matrix 
    pub fn get_sub_matrix(self, rows:Range<usize>, cols:Range<usize>) ->Option<Matrix>{
        //TODO: Get mutable vs reference to this matrix (?) 
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

        let data_expected = vec![vec![0.0; 2]; 3];

        assert_eq!(matrix.data, data_expected);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.rows, rows);
    }   
}

