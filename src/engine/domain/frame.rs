// ----------------------- Pixel ----------------------- //
#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    depth: f64
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { r: 0, g: 0, b: 0, depth: f64::NEG_INFINITY }
    }
}

// ----------------------- Frame ----------------------- //
pub struct Frame {
    pixels: Vec<Pixel>
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame { pixels: vec![Pixel::new(); width * height] }
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