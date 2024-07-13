use crossterm::event;
use crate::buffer::Buffer;
use crate::widget::Widget;
use crate::utils;

pub struct DebugState {
    frame_count: u16,
    achieved_fps: u32,
    prev_update_time: std::time::Instant,
    millis_elapsed: u32,
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
            state: DebugState { 
                frame_count: 0, 
                achieved_fps: 0, 
                prev_update_time: std::time::Instant::now() - std::time::Duration::from_secs(2), 
                millis_elapsed: 1000,
            },
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
        let elapsed_duration = self.state.prev_update_time.elapsed();
        let time_delta = elapsed_duration.subsec_millis() + 1;

        self.state.achieved_fps = 1000 / time_delta;
        self.state.millis_elapsed = time_delta;

        self.buffer.curs_pos = 0;
        self.state.frame_count += 1;
        self.state.prev_update_time = std::time::Instant::now();
    }

    fn draw(&mut self) {
        let line_string = "-".repeat(self.width as usize);
        let chars = utils::char_vec_from_string(line_string);
        self.buffer.move_cursor_to(0);
        self.buffer.insert_chars(chars.clone());
        
        self.buffer.move_cursor_to_x_y(1, 1);
        let string = format!("Frame Count: {}", self.state.frame_count);
        self.add_string(string);
        
        self.buffer.move_cursor_to_x_y(1, 2);
        let string = format!("Achieved FPS: {}", self.state.achieved_fps);
        self.add_string(string);
        
        self.buffer.move_cursor_to_x_y(1, 3);
        let string = format!("Millis Elapsed: {}", self.state.millis_elapsed);
        self.add_string(string);
    }

    fn move_to(&mut self, new_x: u16, new_y: u16) {
        self.x = new_x;
        self.y = new_y;
    }
}
