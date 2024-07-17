use crossterm::event;
use crate::buffer::Buffer;

pub trait Widget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self where Self: Sized;

    fn handle_event(&mut self, event_to_handle: event::Event);
    fn update(&mut self);
    fn draw(&mut self);
    
    fn resize(&mut self, new_width: u16, new_height: u16);
    fn move_to(&mut self, new_x: u16, new_y: u16);
    fn reset_buffer(&mut self);

    fn title(&self) -> String {
        return "".to_string();
    }
    
    fn get_buffer(&mut self) -> Option<&mut Buffer> {
        return None;
    }
}
