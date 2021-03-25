use super::super::frame::{ Color, Pixel };
use super::shape::{ Coord, Shape };
use super::line::Line;

// ---------------------------------- Unfilled Triangle ---------------------------------- //

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

// ---------------------------------- Filled Triangle ---------------------------------- //

pub struct FillTriangle {
    pub p1: Coord,
    pub p2: Coord,
    pub p3: Coord,
    pub color: Color
}

impl FillTriangle {
    fn fill_bottom_triangle(v1: Coord, v2: Coord, v3: Coord, color: Color) -> Vec<Pixel> {
        let invslope_1 = (v2.x - v1.x) as f64 / (v2.y - v1.y) as f64;
        let invslope_2 = (v3.x - v1.x) as f64 / (v3.y - v1.y) as f64;
        
        let mut curx1 = v1.x as f64;
        let mut curx2 = v1.x as f64;

        let mut depth_1 = v1.depth;
        let mut depth_2 = v1.depth;
        let depth_step_1 = (v1.depth - v2.depth) / (v1.y - v2.y) as f64;
        let depth_step_2 = (v1.depth - v3.depth) / (v1.y - v3.y) as f64;

        let mut lines = vec![];
        for scanline_y in v1.y .. v2.y {
            let start = Coord { x: curx1 as i32, y: scanline_y, depth: depth_1 };
            let end = Coord { x: curx2 as i32, y: scanline_y, depth: depth_2 };
            lines.extend(Line { start: start, end: end, color: color }.to_pixels());

            curx1 += invslope_1;
            curx2 += invslope_2;
            depth_1 += depth_step_1;
            depth_2 += depth_step_2;
        }

        lines
    }

    fn fill_top_triangle(v1: Coord, v2: Coord, v3: Coord, color: Color) -> Vec<Pixel> {
        let invslope_1 = (v3.x - v1.x) as f64 / (v3.y - v1.y) as f64;
        let invslope_2 = (v3.x - v2.x) as f64 / (v3.y - v2.y) as f64;
        
        let mut curx1 = v3.x as f64;
        let mut curx2 = v3.x as f64;

        let mut depth_1 = v3.depth;
        let mut depth_2 = v3.depth;
        let depth_step_1 = (v3.depth - v2.depth) / (v3.y - v2.y) as f64;
        let depth_step_2 = (v3.depth - v1.depth) / (v3.y - v1.y) as f64;

        let mut lines = vec![];
        for scanline_y in (v1.y .. v3.y).rev() {
            let start = Coord { x: curx1 as i32, y: scanline_y, depth: depth_1 };
            let end = Coord { x: curx2 as i32, y: scanline_y, depth: depth_2 };
            lines.extend(Line { start: start, end: end, color: color }.to_pixels());

            curx1 -= invslope_1;
            curx2 -= invslope_2;
            depth_1 -= depth_step_1;
            depth_2 -= depth_step_2;
        }

        lines
    }
}

impl Shape for FillTriangle {
    fn to_pixels(&self) -> Vec<Pixel> {
        let mut points = vec![self.p1, self.p2, self.p3];
        points.sort_by(|a, b| a.y.cmp(&b.y));

        let v1 = points[0];
        let v2 = points[1];
        let v3 = points[2];

        if v2.y == v3.y {
            FillTriangle::fill_bottom_triangle(v1, v2, v3, self.color)
        } else if v1.y == v2.y {
            FillTriangle::fill_top_triangle(v1, v2, v3, self.color)
        } else {
            let item_1 = (v2.y - v1.y) as f64 / (v3.y - v1.y) as f64;
            let item_2 = (v2.y - v1.y) as f64 * (v3.depth - v1.depth);

            let v4 = Coord {
                x: (v1.x as f64 + item_1 * (v3.x - v1.x) as f64) as i32,
                y: v2.y,
                depth: v1.depth + item_2 / (v3.y - v1.y) as f64
            };

            let bottom = FillTriangle::fill_bottom_triangle(v1, v2, v4, self.color);
            let top = FillTriangle::fill_top_triangle(v2, v4, v3, self.color);
            
            let mut result = vec![];
            result.extend(bottom);
            result.extend(top);

            result
        }
    }
}