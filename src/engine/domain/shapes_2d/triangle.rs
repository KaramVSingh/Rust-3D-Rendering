use super::super::frame::{ Color, Pixel };
use super::shape::{ Coord, Shape };
use super::line::Line;

pub struct Triangle {
    pub p1: Coord,
    pub p2: Coord,
    pub p3: Coord,
    pub color: Color
}

impl Shape for Triangle {
    fn to_pixels(&self) -> Vec<Pixel> {
        let line1 = Line { start: self.p1, end: self.p2, color: self.color }.to_pixels();
        let line2 = Line { start: self.p2, end: self.p3, color: self.color }.to_pixels();
        let line3 = Line { start: self.p3, end: self.p1, color: self.color }.to_pixels();

        let mut result = vec![];
        result.extend(line1);
        result.extend(line2);
        result.extend(line3);

        result
    }
}