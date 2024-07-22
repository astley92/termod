use crossterm::{event, style};
use crate::buffer::Buffer;
use crate::character::{self, Character};

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

pub trait WidgetTrait {
    fn init(&mut self);
    fn handle_event(&mut self, event_to_handle: &event::Event);
    fn update(&mut self);
    fn draw(&mut self);
    fn generate_buffer(&mut self) -> Buffer;
    fn get_title(&self) -> &String; 
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

pub fn add_buffer_border(buffer: &mut Buffer, colour: style::Color) {
    let buffer_width = buffer.width as usize;
    let top_bottom_str = &"-".repeat(buffer_width - 2).to_string();
    let top_bottom_line_chars = character::Character::vec_from_string(top_bottom_str);
    for y in 0..buffer.height {
        if y == 0 || y == buffer.height-1 {
            // Top or bottom row
            buffer.insert_char_slice(y as usize * buffer.width as usize + 1, &top_bottom_line_chars);
        } else {
            // Middle rows

            let character = Character {
                c: '|',
                attributes: character::empty_attr_set(),
                colour,
            };
            buffer[buffer_width as usize * y as usize] = character.clone();
            buffer[buffer_width as usize * y as usize + buffer_width-1] = character.clone();
        }
    }
}
