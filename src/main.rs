use crossterm::terminal;
use crossterm::style;
use crossterm::cursor;
use crossterm::event;
use crossterm::{QueueableCommand, ExecutableCommand};
use std::io::{stdout, Write};


struct AppState {
    pub running: bool,
    pub frame_count: u16,
}

struct Buffer {
    pub size: u16,
    pub chars: Vec<Character>,
    pub curs_pos: u16,
    width: u16,
    height: u16,
}

impl Buffer {
    pub fn empty(width: u16, height: u16) -> Buffer {
        let buff_size: u16 = width * height;
        let mut chars = Vec::new();
        for _ in 0..width {
            for _ in 0..height {
                let c = Character {
                    character: b' ',
                    attributes: vec![],
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

        } } 

    pub fn insert_ch(&mut self, ch: Character) {
        self.chars[self.curs_pos as usize] = ch;
        self.increment_cursor();
    }

    pub fn insert_chars(&mut self, chars: Vec<Character>) {
        for ch in chars {
            self.chars[self.curs_pos as usize] = ch;
            self.increment_cursor();
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

    fn increment_cursor(&mut self) {
        if self.curs_pos >= self.size - 1 {
            self.curs_pos -= 1;
        }
        self.curs_pos += 1;
    }
}

#[derive(Clone, Debug)]
pub struct Character {
    character: u8,
    attributes: Vec<style::Attribute>,
}

impl Character {
    pub fn from_char(ch: char) -> Character {
        let character = Character {
            character: ch as u8,
            attributes: vec![],
        };

        return character;
    }
}

mod utils {
    pub fn linear_pos_from_x_y(x: u16, y: u16, width: u16) -> u16 {
        let adjusted_y = y * width;
        return x + adjusted_y;
    }

    pub fn char_vec_from_u16(num: u16) -> Vec<crate::Character> {
        let num_str = num.to_string();
        let mut result = vec![];
        for ch in num_str.chars() {
            result.push(crate::Character::from_char(ch));
        };
        return result;
    }
}

const FPS: u16 = 30;

fn main() {
    let mut state: AppState = AppState {
        running: true,
        frame_count: 0,
    };

    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (w_width, w_height) = terminal::size().unwrap();
    let mut buffer = Buffer::empty(w_width, w_height);
    while state.running {
        // Event
        if event::poll(std::time::Duration::from_millis((1000 / FPS) as u64)).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Char(ch) => {
                            let character = Character::from_char(ch);
                            buffer.insert_ch(character.clone());
                        },
                        event::KeyCode::Esc => { state.running = false },
                        _ => {}
                    }
                },
                _ => {}
            };
        };
        
        // Update
        state.frame_count += 1;
        let last_pos = buffer.curs_pos;
        buffer.move_cursor_to_x_y(0, buffer.height - 1);
        buffer.insert_chars(utils::char_vec_from_u16(state.frame_count));
        buffer.move_cursor_to(last_pos);

        // Draw
        stdout.queue(terminal::Clear(terminal::ClearType::Purge)).unwrap();
        

        for i in 0..buffer.chars.len() {
            let character = &buffer.chars[i];
            let linear_pos = i;
            let x = linear_pos % w_width as usize;
            let y = linear_pos / w_width as usize;
             
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                .queue(style::Print(format!("{}", character.character as char))).unwrap();
        }

        stdout.flush().unwrap();
    }
    
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap(); 
} 
