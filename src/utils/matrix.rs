pub struct Matrix{
    pub data: Vec<Vec<f64>>,
    pub rows: usize, 
    pub cols: usize
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self{
        let data = vec![vec![0.0; cols]; rows];
        Matrix { data, rows, cols }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Self{
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
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

