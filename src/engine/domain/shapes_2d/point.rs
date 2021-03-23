use super::super::frame::{ Color, Pixel };
use super::shape::{ Coord, Shape };

pub struct Point {
    pub coord: Coord,
    pub color: Color
}

impl Shape for Point {
    fn to_pixels(&self) -> Vec<Pixel> {
        if self.coord.x < 0 || self.coord.y < 0 {
            vec![]
        } else {
            vec![Pixel::new(self.coord.x as usize, self.coord.y as usize, self.color, self.coord.depth)]
        }
    }
}