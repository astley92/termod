use crossterm::event;
use crate::buffer::Buffer;
use crate::widget::Widget;
use crate::utils;
use crate::colours;

struct HabitState {
    pub habits: Vec<String>,
    pub selected_habit: usize,
}

pub struct HabitTrackWidget {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub buffer: Buffer,
    state: HabitState,
}

impl HabitTrackWidget {
    pub fn add_string(&mut self, str_to_add: String) {
        let chars = utils::char_vec_from_string(str_to_add);
        self.buffer.insert_chars(chars);
    }
}

impl Widget for HabitTrackWidget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> HabitTrackWidget {
        HabitTrackWidget {
            x: x,
            y: y,
            width: width,
            height: height,
            buffer: Buffer::empty(width, height),
            state: HabitState { 
                habits: vec![
                    "No Smoking".to_string(),
                    "No Drinking".to_string(),
                ],
                selected_habit: 0,
            }
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
    }

    fn draw(&mut self) {
        for i in 0..self.state.habits.len() {
            self.buffer.move_cursor_to_x_y(1, i as u16);
            let mut chars = utils::char_vec_from_string(self.state.habits[i].clone());
            if i == self.state.selected_habit {
                for ch in &mut chars {
                    ch.set_fg_colour(colours::DIMMED_GREEN);
                };
            }
            self.buffer.insert_chars(chars);
        };
    }

    fn move_to(&mut self, new_x: u16, new_y: u16) {
        self.x = new_x;
        self.y = new_y;
    }
    
    fn title(&self) -> String {
        return "Habits".to_string();
    }

    fn get_buffer(&mut self) -> Option<&mut Buffer> {
        return Some(&mut self.buffer);
    }
}
