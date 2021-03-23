mod engine;

use engine::backend::window::mini_fb_window::MiniFbWindow;
use engine::backend::window::window_backend::WindowBackend;
use engine::domain::frame::Frame;
use engine::domain::frame::Pixel;
use engine::domain::frame::Color;

fn main() {
    let mut window = MiniFbWindow::new("Hello World!", 640, 480);
    while window.is_open() {
        let (width, height) = window.get_dimensions();

        let mut frame = Frame::new(width, height);
        let some_pixel = Pixel::new(100, 100, Color { r: 255, g: 255, b: 255 }, 0.0);
        frame.draw_raw_pixel(some_pixel);
        
        window.draw(frame);
    }
}
