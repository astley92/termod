use crossterm::event;
use crate::buffer::Buffer;
use crate::widget::Widget;
use crate::utils;
pub struct DebugState {
    frame_count: u16,
}

pub struct DebugWidget {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub buffer: Buffer,
    state: DebugState,
}

impl DebugWidget {
    pub fn add_string(&mut self, str_to_add: String) {
        let chars = utils::char_vec_from_string(str_to_add);
        self.buffer.insert_chars(chars);
    }
}


impl Widget for DebugWidget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> DebugWidget {
        DebugWidget {
            x: x,
            y: y,
            width: width,
            height: height,
            buffer: Buffer::empty(width, height),
            state: DebugState { frame_count: 0 },
        }
    }
    
    fn handle_event(&mut self, _event_to_handle: event::Event) {}

    fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
        self.buffer = Buffer::empty(new_width, new_height);
    }

    fn reset_buffer(&mut self) {
        self.buffer = Buffer::empty(self.width, self.height); 
    }

    fn update(&mut self) {
        self.buffer.curs_pos = 0;
        self.state.frame_count += 1;
    }
    
    fn draw(&mut self) {
        self.buffer.move_cursor_to(0);
        let string = format!("Frame Count: {}", self.state.frame_count);
        self.add_string(string);
    }

    fn move_to(&mut self, new_x: u16, new_y: u16) {
        self.x = new_x;
        self.y = new_y;
    }
}
