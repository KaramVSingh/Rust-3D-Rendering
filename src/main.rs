mod engine;

use engine::backend::window::mini_fb_window::MiniFbWindow;
use engine::backend::window::window_backend::WindowBackend;
use engine::domain::frame::Frame;

fn main() {
    let mut window = MiniFbWindow::new("Hello World!", 640, 480);
    while window.is_open() {
        let (width, height) = window.get_dimensions();

        let frame = Frame::new(width, height);
        window.draw(frame);
    }
}
