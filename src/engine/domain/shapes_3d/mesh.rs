use super::triangle::{ Triangle, FillTriangle };
use super::super::shapes_2d::shape::Shape as Shape2D;

// ---------------------------------- Unfilled Mesh ---------------------------------- //

pub struct Mesh {
    triangles: Vec<Triangle>
}

impl Mesh {

    pub fn from_triangles(triangles: Vec<Triangle>) -> Mesh {
        Mesh { triangles: triangles }
    }

    pub fn project_to_screenspace(&self, f_near: f64, f_far: f64, f_fov: f64, width: f64, height: f64) -> Vec<Box<dyn Shape2D>> {
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

// ---------------------------------- Filled Mesh ---------------------------------- //

pub struct FillMesh {
    triangles: Vec<FillTriangle>
}

impl FillMesh {

    pub fn from_triangles(triangles: Vec<FillTriangle>) -> FillMesh {
        FillMesh { triangles: triangles }
    }

    pub fn project_to_screenspace(&self, f_near: f64, f_far: f64, f_fov: f64, width: f64, height: f64) -> Vec<Box<dyn Shape2D>> {
        self
            .triangles
            .iter()
            .map(|triangle| { triangle.project_to_screenspace(f_near, f_far, f_fov, width, height) })
            .flatten()
            .collect()
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> FillMesh {
        FillMesh {
            triangles: self.triangles.iter().map(|triangle| { triangle.translate(x, y, z) }).collect()
        }
    }

    pub fn rotate(&self, rx: f64, ry: f64, rz: f64) -> FillMesh {
        FillMesh {
            triangles: self.triangles.iter().map(|triangle| { triangle.rotate(rx, ry, rz) }).collect()
        }
    }
}