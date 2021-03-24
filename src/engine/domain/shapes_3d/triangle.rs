use crate::engine::math::matrix::Mat;
use super::point::Point;
use super::super::shapes_2d::triangle::Triangle as Triangle2D;
use super::super::shapes_2d::shape::Coord as Coord2D;
use super::super::frame::Color;

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub color: Color
}

impl Triangle {
    pub fn project_to_screenspace(&self, f_near: f64, f_far: f64, f_fov: f64, width: f64, height: f64) -> Triangle2D {
        let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * 3.14159).tan();
        let projection_matrix = Mat::new(
            [
                [height/width*f_fov_rad, 0.0, 0.0, 0.0],
                [0.0, f_fov_rad, 0.0, 0.0],
                [0.0, 0.0, f_far/(f_far-f_near), 1.0],
                [0.0, 0.0, (-f_far*f_near)/(f_far-f_near), 0.0]
            ]
        );

        // Apply the 4x4 transformation matrix
        let p1_mat = self.p1.apply_projection(&projection_matrix); let w1 = p1_mat.index(3, 0);
        let p2_mat = self.p2.apply_projection(&projection_matrix); let w2 = p2_mat.index(3, 0);
        let p3_mat = self.p3.apply_projection(&projection_matrix); let w3 = p3_mat.index(3, 0);

        // Conditionally divide all elements by w
        let normalize_by_w = |w: f64, value: f64| {
            if w != 0.0 { value / w } else { value } 
        };

        let p1_normalized = p1_mat.static_operation(&|value| { normalize_by_w(w1, value) });
        let p2_normalized = p2_mat.static_operation(&|value| { normalize_by_w(w2, value) });
        let p3_normalized = p3_mat.static_operation(&|value| { normalize_by_w(w3, value) });

        // Turn back into points
        let p1_3d = Point::from_mat(p1_normalized);
        let p2_3d = Point::from_mat(p2_normalized);
        let p3_3d = Point::from_mat(p3_normalized);

        // draw as 2D triangle
        Triangle2D {
            p1: Coord2D {
                x: ((p1_3d.x() + 1.0) * 0.5 * width) as i32,
                y: ((p1_3d.y() + 1.0) * 0.5 * height) as i32,
                depth: w1
            },
            p2: Coord2D {
                x: ((p2_3d.x() + 1.0) * 0.5 * width) as i32,
                y: ((p2_3d.y() + 1.0) * 0.5 * height) as i32,
                depth: w2
            },
            p3: Coord2D {
                x: ((p3_3d.x() + 1.0) * 0.5 * width) as i32,
                y: ((p3_3d.y() + 1.0) * 0.5 * height) as i32,
                depth: w3
            },
            color: self.color
        }
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Triangle {
        Triangle {
            p1: self.p1.translate(x, y, z),
            p2: self.p2.translate(x, y, z),
            p3: self.p3.translate(x, y, z),
            color: self.color
        }
    }

    // https://math.stackexchange.com/questions/1882276/combining-all-three-rotation-matrices
    pub fn rotate(&self, rx: f64, ry: f64, rz: f64) -> Triangle {
        let sin_a = rx.sin(); let cos_a = rx.cos();
        let sin_b = ry.sin(); let cos_b = ry.cos();
        let sin_y = rz.sin(); let cos_y = rz.cos();

        let rotation_matrix = Mat::new(
            [
                [ cos_b*cos_y, cos_b*sin_y, -sin_b ],
                [ sin_a*sin_b*cos_y-cos_a*sin_y, sin_a*sin_b*sin_y+cos_a*cos_y, sin_a*cos_b ],
                [ cos_a*sin_b*cos_y+sin_a*sin_y, cos_a*sin_b*sin_y-sin_a*cos_y, cos_a*cos_b ]
            ]
        );

        let p1_mat = self.p1.apply_projection(&rotation_matrix);
        let p2_mat = self.p2.apply_projection(&rotation_matrix);
        let p3_mat = self.p3.apply_projection(&rotation_matrix);
        
        Triangle {
            p1: Point::from_mat(p1_mat),
            p2: Point::from_mat(p2_mat),
            p3: Point::from_mat(p3_mat),
            color: self.color
        }
    }
}