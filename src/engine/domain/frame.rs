// ----------------------- Pixel ----------------------- //
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    depth: f64
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { r: 0, g: 0, b: 0, depth: f64::NEG_INFINITY }
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
        Frame { pixels: vec![Pixel::new(); width * height], width: width, height: height }
    }

    pub fn as_buffer<T>(&self, transform: &dyn Fn(&Pixel) -> T) -> Vec<T> {
        self.pixels.iter().map(transform).collect()
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