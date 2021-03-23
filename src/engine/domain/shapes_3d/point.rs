use std::convert::TryInto;

use crate::engine::math::matrix::Mat;

pub struct Point {
    mat: Mat<3, 1>
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            mat: Mat::new([[x], [y], [z]])
        }
    }

    pub fn from_mat<const DIM: usize>(mat: Mat<DIM, 1>) -> Point {
        let vector_array = (0..3)
            .map(
                |index| { 
                    let value = if index >= DIM { 1.0 } else { mat.index(index, 0) };
                    [value]
                }
            )
            .collect::<Vec<[f64; 1]>>()
            .try_into()
            .unwrap();

        let vector = Mat::new(vector_array);
        Point { mat: vector }
    }

    pub fn x(&self) -> f64 {
        self.mat.index(0, 0)
    }

    pub fn y(&self) -> f64 {
        self.mat.index(1, 0)
    }

    pub fn z(&self) -> f64 {
        self.mat.index(2, 0)
    }

    pub fn apply_projection<const DIM: usize>(&self, projection: &Mat<DIM, DIM>) -> Mat<DIM, 1> {
        let vector_array = (0..DIM)
            .map(
                |index| { 
                    let value = if index < 3 { self.mat.index(index, 0) } else { 1.0 };
                    [value]
                }
            )
            .collect::<Vec<[f64; 1]>>()
            .try_into()
            .unwrap();

        let vector = Mat::new(vector_array);
        projection.times_vec(&vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        // Given
        let point = Point::new(0.0, 1.0, 2.0);

        // When, Then
        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 1.0);
        assert_eq!(point.z(), 2.0);
    }

    #[test]
    fn test_3x3_projection() {
        // Given
        let point = Point::new(0.0, 1.0, 2.0);
        let projection = Mat::new(
            [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        );

        // When
        let result = point.apply_projection(&projection);
        let result_as_point = Point::from_mat(result);

        // Then
        assert_eq!(result_as_point.x(), 0.0);
        assert_eq!(result_as_point.y(), 1.0);
        assert_eq!(result_as_point.z(), 2.0);
    }

    #[test]
    fn test_2x2_projection() {
        // Given
        let point = Point::new(0.0, 1.0, 2.0);
        let projection = Mat::new(
            [
                [1.0, 0.0],
                [0.0, 1.0]
            ]
        );

        // When
        let result = point.apply_projection(&projection);
        let result_as_point = Point::from_mat(result);

        // Then
        assert_eq!(result_as_point.x(), 0.0);
        assert_eq!(result_as_point.y(), 1.0);
        assert_eq!(result_as_point.z(), 1.0);
    }

    #[test]
    fn test_4x4_projection() {
        // Given
        let point = Point::new(0.0, 1.0, 2.0);
        let projection = Mat::new(
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );

        // When
        let result = point.apply_projection(&projection);
        let result_as_point = Point::from_mat(result);

        // Then
        assert_eq!(result_as_point.x(), 0.0);
        assert_eq!(result_as_point.y(), 1.0);
        assert_eq!(result_as_point.z(), 2.0);
    }
}