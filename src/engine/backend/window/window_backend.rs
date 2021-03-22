use crate::engine::domain::frame::Frame;

pub trait WindowBackend {
    /**
     * Create a new window with the below parameters
     */
    fn new(window_name: &str, width: usize, height: usize) -> Self;

    /**
     * Return the (width, height) of the window in pixels
     */ 
    fn get_dimensions(&self) -> (usize, usize);

    /**
     * Return a boolean indicating if the frame is currently open
     */
    fn is_open(&self) -> bool;

    /**
     * Draw the passed in frame onto the window
     */
    fn draw(&mut self, frame: Frame);
}