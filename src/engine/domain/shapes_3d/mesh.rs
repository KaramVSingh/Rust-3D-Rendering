use super::triangle::Triangle;
use super::super::shapes_2d::triangle::Triangle as Triangle2D;

pub struct Mesh {
    pub triangles: Vec<Triangle>
}

impl Mesh {
    pub fn project_to_screenspace(&self, f_near: f64, f_far: f64, f_fov: f64, width: f64, height: f64) -> Vec<Triangle2D> {
        self
            .triangles
            .iter()
            .map(|triangle| { triangle.project_to_screenspace(f_near, f_far, f_fov, width, height) })
            .flatten()
            .collect()
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Mesh {
        Mesh {
            triangles: self.triangles.iter().map(|triangle| { triangle.translate(x, y, z) }).collect()
        }
    }

    pub fn rotate(&self, rx: f64, ry: f64, rz: f64) -> Mesh {
        Mesh {
            triangles: self.triangles.iter().map(|triangle| { triangle.rotate(rx, ry, rz) }).collect()
        }
    }
}