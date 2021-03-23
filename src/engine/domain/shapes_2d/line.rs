use std::ops::Range;

use super::super::frame::{ Color, Pixel };
use super::shape::{ Coord, Shape };
use super::point::Point;

pub struct Line {
    pub start: Coord,
    pub end: Coord,
    pub color: Color
}

impl Line {
    fn get_pixel(coord: Coord, color: Color) -> Vec<Pixel> {
        Point { coord: coord, color: color }.to_pixels()
    }

    fn depth_step(start: i32, end: i32, start_depth: f64, end_depth: f64) -> f64 {
        let steps = start - end;
        if steps == 0 {
            0.0
        } else {
            (start_depth - end_depth) / (steps as f64)
        }
    }

    fn draw_line_low(point1: Coord, point2: Coord, color: Color) -> Vec<Pixel> {
        let dx = point2.x as f64 - point1.x as f64;
        let temp_dy = point2.y as f64 - point1.y as f64;
        let depth_s = Line::depth_step(point1.x, point2.x, point1.depth, point2.depth);

        let (y_step, dy) = if temp_dy < 0.0 {
            (-1, -temp_dy)
        } else {
            (1, temp_dy)
        };

        let x_range = Range { start: point1.x, end: point2.x };

        let mut depth = point1.depth;
        let mut curr_y = point1.y;
        let mut curr_delta_err = 2.0 * dy - dx;
        let mut pixels = vec![];

        for x in x_range {
            pixels.extend(Line::get_pixel(Coord { x: x, y: curr_y, depth: depth }, color));
            depth += depth_s;

            if curr_delta_err > 0.0 {
                curr_y += y_step;
                curr_delta_err += 2.0 * (dy - dx);
            } else {
                curr_delta_err += 2.0 * dy;
            }
        }

        pixels
    }

    fn draw_line_high(point1: Coord, point2: Coord, color: Color) -> Vec<Pixel> {
        let temp_dx = point2.x as f64 - point1.x as f64;
        let dy = point2.y as f64 - point1.y as f64;
        let depth_s = Line::depth_step(point1.y, point2.y, point1.depth, point2.depth);
        
        let (x_step, dx) = if temp_dx < 0.0 {
            (-1, -temp_dx)
        } else {
            (1, temp_dx)
        };

        let y_range = Range { start: point1.y, end: point2.y };
        
        let mut depth = point1.depth;
        let mut curr_x = point1.x;
        let mut curr_delta_err = 2.0 * dx - dy;
        let mut pixels = vec![];

        for y in y_range {
            pixels.extend(Line::get_pixel(Coord { x: curr_x, y: y, depth: depth }, color));
            depth += depth_s;

            if curr_delta_err > 0.0 {
                curr_x += x_step;
                curr_delta_err += 2.0 * (dx - dy);
            } else {
                curr_delta_err += 2.0 * dx;
            }
        }

        pixels
    }
}

impl Shape for Line {

    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn to_pixels(&self) -> Vec<Pixel> {
        let point1 = self.start;
        let point2 = self.end;

        if (point2.y - point1.y).abs() < (point2.x - point1.x).abs() {
            if point1.x > point2.x {
                Line::draw_line_low(point2, point1, self.color)
            } else {
                Line::draw_line_low(point1, point2, self.color)
            }
        } else {
            if point1.y > point2.y {
                Line::draw_line_high(point2, point1, self.color)
            } else {
                Line::draw_line_high(point1, point2, self.color)
            }
        }
    }
}