use crossterm::event;
use crate::buffer::Buffer;

pub struct Widget<T> {
    pub width: u16,
    pub height: u16,
    pub title: String,
    pub init_fn: fn(&mut Widget<T>),
    pub event_fn: fn(&mut Widget<T>, &event::Event),
    pub update_fn: fn(&mut Widget<T>),
    pub draw_fn: fn(&mut Widget<T>),
    pub generate_buffer_fn: fn(&mut Widget<T>) -> Buffer,
    pub state: T,
}


impl<T> Widget<T> {
    pub fn init(&mut self) {
        (self.init_fn)(self);
    }
    
    pub fn handle_event(&mut self, event_to_handle: &event::Event) {
        (self.event_fn)(self, event_to_handle);
    }

    pub fn update(&mut self) {
        (self.update_fn)(self);
    }
    
    pub fn draw(&mut self) {
        (self.draw_fn)(self);
    }

    pub fn generate_buffer(&mut self) -> Buffer {
        return (self.generate_buffer_fn)(self);
    }
    
}
