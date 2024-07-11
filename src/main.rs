use crossterm::terminal;
use crossterm::style;
use crossterm::cursor;
use crossterm::event;
use crossterm::{QueueableCommand, ExecutableCommand};
use std::io::{stdout, Write};

struct AppState {
    pub running: bool,
    pub frame_count: u32,
}

struct Buffer {
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
    }

    pub fn increment_cursor(&mut self) {
        if self.curs_pos >= self.size - 1 {
            self.curs_pos -= 1;
        }
        self.curs_pos += 1;
    }
}

#[derive(Clone, Debug)]
struct Character {
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

const FPS: u16 = 30;

fn main() {
    let mut state: AppState = AppState {
        running: true,
        frame_count: 0,
    };

    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide);

    let (w_width, w_height) = terminal::size().unwrap();
    let mut buffer = Buffer::empty(w_width, w_height);
    while state.running {
        // Event
        if event::poll(std::time::Duration::from_millis((1000 / FPS) as u64)).unwrap() {
            let event = event::read().unwrap();
            
            if let event::Event::Key(event::KeyEvent{
                code: event::KeyCode::Char(ch),
                kind: event::KeyEventKind::Press,
                modifiers: _,
                state: _,
            }) = event {
                if ch == 'q' {
                    state.running = false;
                }
                let character = Character::from_char(ch);
                buffer.insert_ch(character.clone());
                buffer.increment_cursor();
            };
        };
        
        // Update
        state.frame_count += 1;

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
    
    stdout.execute(cursor::Show);
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap(); 
} 
