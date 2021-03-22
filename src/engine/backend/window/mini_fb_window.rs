use minifb::{ Window, WindowOptions };

use super::window_backend::WindowBackend;
use crate::engine::domain::frame::Frame;

pub struct MiniFbWindow {
    window: Window
}

impl WindowBackend for MiniFbWindow {

    fn new(window_name: &str, width: usize, height: usize) -> MiniFbWindow {
        let window = match Window::new(
            window_name,
            width,
            height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            }
        ) {
            Ok(win) => win,
            Err(err) =>  panic!("Unable to create window {}", err)
        };

        MiniFbWindow { window }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        self.window.get_size()
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn draw(&mut self, frame: Frame) {
        let as_backend_buffer: Vec<u32> = frame.as_buffer(
            &|pixel| { ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32) }
        );
        
        let (width, height) = self.get_dimensions();
        self.window.update_with_buffer(&as_backend_buffer, width, height).unwrap();
    }
}