use crate::utils;
use crate::character::Character;
use crate::buffer::Buffer;
use crate::widget::Widget;
use crate::habit_track_widget::HabitTrackWidget;
use crossterm::event;

pub struct MainWidget {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub buffer: Buffer,
    sub_widgets: Vec<Box<dyn Widget>>,
    active_subwidget_index: usize,
}

impl MainWidget {
    fn draw_box(&mut self) {
        let line_string = "-".repeat(self.width as usize - 2);
        let horizontal_string = format!(" {} ", line_string);
        self.buffer.move_cursor_to(0);
        let chars = utils::char_vec_from_string(horizontal_string);
        self.buffer.insert_chars(&chars);
        self.buffer.move_cursor_to_x_y(0, self.buffer.height - 1);
        self.buffer.insert_chars(&chars);
        for y in 1..self.height-1 {
            self.buffer.move_cursor_to_x_y(0, y);
            self.buffer.insert_ch(&Character::from_char('|'));
            self.buffer.move_cursor_to_x_y(self.width-1, y);
            self.buffer.insert_ch(&Character::from_char('|'));
        }
    }
}

impl Widget for MainWidget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> MainWidget {
        MainWidget {
            x: x,
            y: y,
            width: width,
            height: height,
            buffer: Buffer::empty(width, height),
            sub_widgets: vec![
                Box::new(HabitTrackWidget::new(1, 1, width - 2, height - 2)),
            ],
            active_subwidget_index: 0,
        }
    }

    fn handle_event(&mut self, event_to_handle: event::Event) {
        match event_to_handle {
            event::Event::Key(event_to_handle) => {
                match event_to_handle.code {
                    event::KeyCode::Char(ch) => {
                        let character = Character::from_char(ch);
                        self.buffer.insert_ch(&character);
                    },
                    event::KeyCode::Tab => {
                        self.active_subwidget_index = (self.active_subwidget_index + 1) % self.sub_widgets.len();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
        self.buffer = Buffer::empty(new_width, new_height);
    }

    fn reset_buffer(&mut self) {
        self.buffer = Buffer::empty(self.width, self.height);
    }

    fn update(&mut self) {}
    
    fn draw(&mut self) {
        self.draw_box();
        let mut title_offset: usize = 2;
        for i in 0..self.sub_widgets.len() {
            let sub_widget = &mut self.sub_widgets[i];
            self.buffer.move_cursor_to_x_y(title_offset as u16, 0);
            let mut chars = utils::char_vec_from_string(sub_widget.title());
            title_offset += chars.len() + 1;
            if i == self.active_subwidget_index {
                for char in &mut chars {
                    char.toggle_reverse();
                }
            }
            sub_widget.update(); 
            self.buffer.insert_chars(&chars);
        };

        let sub_widget = &mut self.sub_widgets[self.active_subwidget_index];
        sub_widget.draw();
        let sub_buffer = sub_widget.get_buffer();
        match sub_buffer {
            Some(buffer) => { self.buffer.insert_buffer(1, 1, buffer) },
            _ => {}
        }
    }
    
    fn move_to(&mut self, new_x: u16, new_y: u16) {
        self.x = new_x;
        self.y = new_y;
    }
}
