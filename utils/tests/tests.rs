
#[cfg(test)]
mod matrix_utils_tests {
    use utils::matrix::{rotate_clockwise, transpose};

    #[test]
    fn matrices_can_be_rotated() {
        let to_rotate = vec![
            vec![11, 12, 13],
            vec![21, 22, 23]
            ];

        let expected = vec![
            vec![21, 11],
            vec![22, 12],
            vec![23, 13]
        ];

        assert_eq!(rotate_clockwise(&to_rotate), expected);
    }

    #[test]
    fn square_matrices_can_be_transposed() {
        let to_transpose = vec![
            vec![11, 12],
            vec![21, 22]
        ];

        let expected = vec![
            vec![11, 21],
            vec![12, 22]
        ];

        assert_eq!(transpose(&to_transpose), expected);
    }

    #[test]
    fn non_square_matrices_can_be_trasposed() {
        let to_transpose = vec![
            vec![11, 12, 13],
            vec![21, 22, 23]
        ];

        let expected = vec![
            vec![11, 21],
            vec![12, 22],
            vec![13, 23]
        ];

        assert_eq!(transpose(&to_transpose), expected);
    }

}

#[cfg(test)]
mod extras_tests {
    use utils::extras::contains_duplicates;

    #[test]
    fn vec_with_duplicates() {
        assert!(contains_duplicates(vec![1,2,3,3].iter()));
    }

    #[test]
    fn vec_without_duplicates() {
        assert!(!contains_duplicates(vec![1,2,3,4].iter()));
    }
}