#[cfg(test)]
mod tests {
    use ferrix::{Matrix, Vector};

    #[test]
    fn test_default() {
        let matrix = Matrix::<i32, 2, 3>::default();
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix[(0, 1)], 0);
        assert_eq!(matrix[(0, 2)], 0);
        assert_eq!(matrix[(1, 0)], 0);
        assert_eq!(matrix[(1, 1)], 0);
        assert_eq!(matrix[(1, 2)], 0);
    }

    #[test]
    fn test_eye() {
        let eye_matrix = Matrix::<i32, 2, 2>::eye();
        assert_eq!(eye_matrix[(0, 0)], 1);
        assert_eq!(eye_matrix[(0, 1)], 0);
        assert_eq!(eye_matrix[(1, 0)], 0);
        assert_eq!(eye_matrix[(1, 1)], 1);
    }

    #[test]
    fn test_zeros() {
        let zeros_matrix = Matrix::<i32, 2, 3>::zeros();
        assert_eq!(zeros_matrix[(0, 0)], 0);
        assert_eq!(zeros_matrix[(0, 1)], 0);
        assert_eq!(zeros_matrix[(0, 2)], 0);
        assert_eq!(zeros_matrix[(1, 0)], 0);
        assert_eq!(zeros_matrix[(1, 1)], 0);
        assert_eq!(zeros_matrix[(1, 2)], 0);
    }

    #[test]
    fn test_ones() {
        let ones_matrix = Matrix::<i32, 3, 2>::ones();
        assert_eq!(ones_matrix[(0, 0)], 1);
        assert_eq!(ones_matrix[(0, 1)], 1);
        assert_eq!(ones_matrix[(1, 0)], 1);
        assert_eq!(ones_matrix[(1, 1)], 1);
        assert_eq!(ones_matrix[(2, 0)], 1);
        assert_eq!(ones_matrix[(2, 1)], 1);
    }

    #[test]
    fn test_new() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(0, 2)], 3);
        assert_eq!(matrix[(1, 0)], 4);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(1, 2)], 6);
    }

    #[test]
    fn test_fill() {
        let matrix = Matrix::<i32, 2, 3>::fill(1);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 1);
        assert_eq!(matrix[(0, 2)], 1);
        assert_eq!(matrix[(1, 0)], 1);
        assert_eq!(matrix[(1, 1)], 1);
        assert_eq!(matrix[(1, 2)], 1);
    }

    #[test]
    fn test_shape() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix.shape(), (2, 3));
    }

    #[test]
    fn test_t() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let transposed = matrix.t();

        assert_eq!(transposed.shape(), (3, 2));
        assert_eq!(transposed[(0, 0)], 1);
        assert_eq!(transposed[(0, 1)], 4);
        assert_eq!(transposed[(1, 0)], 2);
        assert_eq!(transposed[(1, 1)], 5);
        assert_eq!(transposed[(2, 0)], 3);
        assert_eq!(transposed[(2, 1)], 6);
    }

    #[test]
    fn test_t_mut() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let mut transposed = matrix.t_mut();

        transposed[(0, 1)] = 10;
        transposed[(2, 0)] = 20;

        assert_eq!(transposed.shape(), (3, 2));
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(0, 2)], 20);
        assert_eq!(matrix[(1, 0)], 10);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(1, 2)], 6);
    }

    #[test]
    fn test_view() {
        let matrix = Matrix::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
        let view = matrix.view::<2, 2>((1, 1)).unwrap();

        assert_eq!(view.shape(), (2, 2));
        assert_eq!(view[(0, 0)], 6);
        assert_eq!(view[(0, 1)], 7);
        assert_eq!(view[(1, 0)], 10);
        assert_eq!(view[(1, 1)], 11);
    }

    #[test]
    fn test_view_out_of_bounds() {
        let matrix = Matrix::new([[1, 2], [3, 4]]);
        assert!(matrix.view::<2, 2>((1, 1)).is_none());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_view_index_out_of_bounds() {
        let matrix = Matrix::new([[1, 2, 3], [5, 6, 7], [9, 10, 11]]);
        let view = matrix.view::<2, 2>((1, 1)).unwrap();
        let _ = view[(2, 3)];
    }

    #[test]
    fn test_view_mut() {
        let mut matrix = Matrix::new([[1, 2], [5, 6], [9, 10]]);

        {
            let mut view = matrix.view_mut::<2, 2>((1, 0)).unwrap();
            assert_eq!(view.shape(), (2, 2));
            view[(0, 0)] = 20;
            view[(1, 1)] = 30;
        }

        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(1, 0)], 20);
        assert_eq!(matrix[(1, 1)], 6);
        assert_eq!(matrix[(2, 0)], 9);
        assert_eq!(matrix[(2, 1)], 30);
    }

    #[test]
    fn test_view_mut_out_of_bounds() {
        let mut matrix = Matrix::new([[1, 2], [3, 4]]);
        assert!(matrix.view_mut::<2, 2>((1, 1)).is_none());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_view_mut_index_out_of_bounds() {
        let matrix = Matrix::new([[1, 2, 3], [5, 6, 7], [9, 10, 11]]);
        let view = matrix.view::<2, 2>((1, 1)).unwrap();
        let _ = view[(2, 3)];
    }

    #[test]
    fn test_index_single() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix[0], 1);
        assert_eq!(matrix[1], 2);
        assert_eq!(matrix[2], 3);
        assert_eq!(matrix[3], 4);
        assert_eq!(matrix[4], 5);
        assert_eq!(matrix[5], 6);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_single_out_of_bounds() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let _ = matrix[6];
    }

    #[test]
    fn test_index_mut_single() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        matrix[1] = 10;
        matrix[4] = 20;
        assert_eq!(matrix[0], 1);
        assert_eq!(matrix[1], 10);
        assert_eq!(matrix[2], 3);
        assert_eq!(matrix[3], 4);
        assert_eq!(matrix[4], 20);
        assert_eq!(matrix[5], 6);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_mut_single_out_of_bounds() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        matrix[6] = 10;
    }

    #[test]
    fn test_index() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(0, 2)], 3);
        assert_eq!(matrix[(1, 0)], 4);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(1, 2)], 6);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_out_of_bounds() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let _ = matrix[(2, 0)];
    }

    #[test]
    fn test_index_mut() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        matrix[(0, 1)] = 10;
        matrix[(1, 2)] = 20;
        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 10);
        assert_eq!(matrix[(0, 2)], 3);
        assert_eq!(matrix[(1, 0)], 4);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(1, 2)], 20);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_mut_out_of_bounds() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        matrix[(0, 3)] = 10;
    }

    #[test]
    fn test_from_vector_transpose_view() {
        let vector = Vector::new([1, 2, 3, 4]);
        let t_view = vector.t();
        let matrix: Matrix<_, 1, 4> = Matrix::from(t_view);
        assert_eq!(matrix, Matrix::new([[1, 2, 3, 4]]));
    }

    #[test]
    fn test_from_vector_transpose_view_mut() {
        let mut vector = Vector::new([1, 2, 3, 4]);
        let t_view = vector.t_mut();
        let matrix: Matrix<_, 1, 4> = Matrix::from(t_view);
        assert_eq!(matrix, Matrix::new([[1, 2, 3, 4]]));
    }

    #[test]
    fn test_from_matrix_view() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let view = matrix.view::<2, 2>((0, 1)).unwrap();
        let new_matrix: Matrix<_, 2, 2> = Matrix::from(view);
        assert_eq!(new_matrix, Matrix::new([[2, 3], [5, 6]]));
    }

    #[test]
    fn test_from_matrix_view_mut() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let view = matrix.view_mut::<2, 2>((0, 1)).unwrap();
        let new_matrix: Matrix<_, 2, 2> = Matrix::from(view);
        assert_eq!(new_matrix, Matrix::new([[2, 3], [5, 6]]));
    }

    #[test]
    fn test_from_matrix_transpose_view() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let t_view = matrix.t();
        let new_matrix: Matrix<_, 3, 2> = Matrix::from(t_view);
        assert_eq!(new_matrix, Matrix::new([[1, 4], [2, 5], [3, 6]]));
    }

    #[test]
    fn test_from_matrix_transpose_view_mut() {
        let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let t_view = matrix.t_mut();
        let new_matrix: Matrix<_, 3, 2> = Matrix::from(t_view);
        assert_eq!(new_matrix, Matrix::new([[1, 4], [2, 5], [3, 6]]));
    }
}