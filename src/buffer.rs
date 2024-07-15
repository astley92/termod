use crate::utils;
use crate::character::Character;

pub struct Buffer {
    pub size: u16,
    pub chars: Vec<Character>,
    pub curs_pos: u16,
    pub width: u16,
    pub height: u16,
}

impl Buffer {
    pub fn empty(width: u16, height: u16) -> Buffer {
        let buff_size: u16 = width * height;
        let mut chars = Vec::new();
        for _ in 0..width {
            for _ in 0..height {
                let c = Character {
                    character: ' ',
                    attributes: vec![],
                    fg_colour: None,
                };
                chars.push(c);
            }
        }
        return Buffer {
            size: buff_size,
            width: width,
            height: height,
            chars: chars,
            curs_pos: 0,

        } 
    }

    pub fn insert_ch(&mut self, ch: &Character) {
        self.chars[self.curs_pos as usize] = ch.clone();
        self.increment_cursor();
    }

    pub fn insert_chars(&mut self, chars: &Vec<Character>) {
        for ch in chars {
            self.insert_ch(ch);
        };
    }

    pub fn move_cursor_to_x_y(&mut self, x: u16, y: u16) -> u16 {
        if x >= self.width { 
            panic!("Cursor moved outside of buffer width given x {} buffer width {}", x, self.width);
        } else if y >= self.height {
            panic!("Cursor moved outside of buffer height");
        };

        let result = utils::linear_pos_from_x_y(x, y, self.width);
        self.curs_pos = result;
        return result;
    }

    pub fn move_cursor_to(&mut self, pos: u16) {
        if pos >= self.size {
            panic!("Cursor moved outside buffer range");
        };

        self.curs_pos = pos;
    }

    pub fn insert_buffer(&mut self, x_offset: u16, y_offset: u16, other_buffer: &Buffer) {
        if !self.can_consume(x_offset, y_offset, other_buffer) {
            panic!("Trying to insert a buffer that doesn't fit {},{},{},{} into {},{}", x_offset, y_offset, other_buffer.width, other_buffer.height, self.width, self.height);
        }

        for other_x in 0..other_buffer.width {
            for other_y in 0..other_buffer.height {
                let other_position = utils::linear_pos_from_x_y(other_x, other_y, other_buffer.width);
                let x = other_x + x_offset;
                let y = other_y + y_offset;

                self.move_cursor_to_x_y(x, y);
                self.insert_ch(&other_buffer.chars[other_position as usize]);
            }
        }
    }

    fn can_consume(&self, x_offset: u16, y_offset: u16, other_buffer: &Buffer) -> bool {
        let other_max_width_within_self = x_offset + other_buffer.width; 
        if other_max_width_within_self > self.width {
            return false;
        };

        let other_max_height_within_self = y_offset + other_buffer.height; 
        if other_max_height_within_self > self.height {
            return false;
        };

        return true;
    }

    fn increment_cursor(&mut self) {
        if self.curs_pos >= self.size - 1 {
            self.curs_pos -= 1;
        }
        self.curs_pos += 1;
    }
}
