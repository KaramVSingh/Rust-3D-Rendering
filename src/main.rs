mod engine;

use engine::backend::implementations::mini_fb::MiniFb;
use engine::backend::window_backend::WindowBackend;
use engine::domain::shapes_2d::shape::Coord;
use engine::domain::shapes_2d::triangle::Triangle;
use engine::domain::frame::Frame;
use engine::domain::frame::Color;

fn main() {
    let mut window = MiniFb::new("Hello World!", 640, 480);
    while window.is_open() {
        let (width, height) = window.get_dimensions();

        let mut frame = Frame::new(width, height);
        let some_triangle = Triangle {
            p1: Coord { x: 100, y: 100, depth: 1.0 },
            p2: Coord { x: 100, y: 300, depth: 1.0 },
            p3: Coord { x: 200, y: 200, depth: 0.0 },
            color: Color { r: 0, g: 255, b: 0 }
        };

        frame.draw_shape(&some_triangle);
        window.draw(frame);
    }
}
