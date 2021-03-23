use super::super::frame::Pixel;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub depth: f64
}

pub trait Shape {
    /**
     * Convert any shape into a vector of pixels to be drawn to the frame
     */
    fn to_pixels(&self) -> Vec<Pixel>;
}