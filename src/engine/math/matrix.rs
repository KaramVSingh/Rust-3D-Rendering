#[derive(PartialEq, Debug)]
pub struct Mat<const ROWS: usize, const COLS: usize> {
    // Indexing - data[row][col]
    data: [[f64; COLS]; ROWS]
}

impl <const ROWS: usize, const COLS: usize> Mat<ROWS, COLS> {
    pub fn new(data: [[f64; COLS]; ROWS]) -> Mat<ROWS, COLS> {
        Mat { data: data }
    }

    pub fn add(&self, other: &Mat<ROWS, COLS>) -> Mat<ROWS, COLS> {
        Mat::static_operation(self, other, &|a, b| { a + b })
    }

    pub fn sub(&self, other: &Mat<ROWS, COLS>) -> Mat<ROWS, COLS> {
        Mat::static_operation(self, other, &|a, b| { a - b })
    }

    pub fn times_vec(&self, other: &Mat<COLS, 1>) -> Mat<ROWS, 1> {
        let new_data = &mut [[0.0; 1]; ROWS];
        for row in 0..ROWS {
            let summed = &mut 0.0;
            for col in 0..COLS {
                *summed += self.data[row][col] * other.data[col][0];
            }

            new_data[row][0] = *summed;
        }

        Mat::new(*new_data)
    }

    // ----------------------- Private Static Functions ----------------------- //
    fn static_operation(a: &Mat<ROWS, COLS>, b: &Mat<ROWS, COLS>, operation: &dyn Fn(f64, f64) -> f64) -> Mat<ROWS, COLS> {
        let new_data = &mut [[0.0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                new_data[row][col] = operation(a.data[row][col], b.data[row][col]);
            }
        }

        Mat::new(*new_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        // Given
        let a = Mat::new([[5.0, 6.0]]);
        let b = Mat::new([[5.0, 6.0]]);
        let c = Mat::new([[7.0, 8.0]]);

        // When, Then
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn test_times_vec() {
        // Given
        let a = Mat::new(
            [
                [1.0, -1.0, 2.0],
                [0.0, -3.0, 1.0]
            ]
        );

        let b = Mat::new(
            [
                [2.0],
                [1.0],
                [0.0]
            ]
        );

        let expected = Mat::new(
            [
                [1.0],
                [-3.0]
            ]
        );

        // When
        let c = a.times_vec(&b);

        // Then
        assert_eq!(c, expected);
    }

    #[test]
    fn test_sub() {
        // Given
        let a = Mat::new(
            [
                [1.0],
                [2.0]
            ]
        );

        let b = Mat::new(
            [
                [3.0],
                [4.0]
            ]
        );

        let expected = Mat::new(
            [
                [-2.0],
                [-2.0]
            ]
        );

        // When
        let c = a.sub(&b);

        // Then
        assert_eq!(c, expected);
    }

    #[test]
    fn test_add() {
        // Given
        let a: Mat<1, 2> = Mat::new(
            [
                [0.0, 1.0]
            ]
        );

        let b: Mat<1, 2> = Mat::new(
            [
                [2.0, 3.0]
            ]
        );

        let expected = Mat::new(
            [
                [2.0, 4.0]
            ]
        );

        // When
        let c = a.add(&b);

        // Then
        assert_eq!(c, expected);
    }
}