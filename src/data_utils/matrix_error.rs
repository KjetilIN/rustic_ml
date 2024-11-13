use core::fmt::Display;

/// Custom Error type for a `Matrix` operation
#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    /// An error that occurs when a operation requires that two matrices have the exact same shape
    ShapeMismatch {
        /// Shape of the matrix that called the matrix operation function.
        first_matrix_shape: String,

        /// Shape of the matrix that was given in a matrix operation function.
        second_matrix_shape: String,
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
            MatrixError::ShapeMismatch {
                first_matrix_shape,
                second_matrix_shape,
            } => {
                write!(f, "The matrix with shape ({first_matrix_shape}) does not match the shape of the given matrix ({second_matrix_shape})")
            }
            MatrixError::DivideByZero => {
                write!(f, "Given matrix or number lead to division by zero")
            }
            MatrixError::IllegalRange(val) => {
                write!(f, "Illegal Range Given: {val}")
            }
            MatrixError::MatrixMultiply => {
                write!(f, "Illegal to multiply the two given matrixes.")
            }
        }
    }
}

// Making the MatrixError an Error
impl std::error::Error for MatrixError {}
