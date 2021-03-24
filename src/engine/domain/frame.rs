use super::shapes_2d::shape::Shape;

// ----------------------- Pixel ----------------------- //
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    depth: f64
}

impl Pixel {
    pub fn empty() -> Pixel {
        Pixel { x: 0, y: 0, color: Color { r: 0, g: 0, b: 0 }, depth: f64::NEG_INFINITY }
    }

    pub fn new(x: usize, y: usize, color: Color, depth: f64) -> Pixel {
        Pixel { x: x, y: y, color: color, depth: depth }
    }
}

// ----------------------- Frame ----------------------- //
pub struct Frame {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame { pixels: vec![Pixel::empty(); width * height], width: width, height: height }
    }

    pub fn draw_shape(&mut self, shape: &dyn Shape) {
        let pixels = shape.to_pixels();
        for pixel in pixels {
            self.draw_raw_pixel(pixel);
        }
    }

    pub fn as_buffer<T>(&self, transform: &dyn Fn(&Pixel) -> T) -> Vec<T> {
        self.pixels.iter().map(transform).collect()
    }

    // Private helper functions

    fn draw_raw_pixel(&mut self, pixel: Pixel) {
        if pixel.x < self.width && pixel.y < self.height {
            if pixel.depth > self.pixels[pixel.y * self.width + pixel.x].depth {
                self.pixels[pixel.y * self.width + pixel.x] = pixel
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame() {
        // Given, When, Then
        let _frame = Frame::new(640, 480);
    }
}