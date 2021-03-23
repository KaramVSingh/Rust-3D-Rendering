use minifb::{ Window, WindowOptions };

use super::super::window_backend::WindowBackend;
use crate::engine::domain::frame::Frame;

pub struct MiniFb {
    window: Window
}

impl WindowBackend for MiniFb {

    fn new(window_name: &str, width: usize, height: usize) -> MiniFb {
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

        MiniFb { window }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        self.window.get_size()
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn draw(&mut self, frame: Frame) {
        let as_backend_buffer: Vec<u32> = frame.as_buffer(
            &|pixel| { ((pixel.color.r as u32) << 16) | ((pixel.color.g as u32) << 8) | (pixel.color.b as u32) }
        );
        
        let (width, height) = self.get_dimensions();
        self.window.update_with_buffer(&as_backend_buffer, width, height).unwrap();
    }
}