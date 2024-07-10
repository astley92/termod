use crossterm::terminal;
use crossterm::style;
use crossterm::cursor;
use crossterm::event;
use crossterm::{QueueableCommand, ExecutableCommand};
use std::io::{stdout, Write};

struct AppState {
    pub running: bool,
    pub last_char: char,
    pub frame_count: u32,
}

fn main() {
    let mut state: AppState = AppState {
        running: true,
        last_char: 'h',
        frame_count: 0,
    };

    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();

    while state.running {
        // Event
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            let event = event::read().unwrap();
            
            if let event::Event::Key(event::KeyEvent{
                code: event::KeyCode::Char(ch),
                kind: event::KeyEventKind::Press,
                modifiers: _,
                state: _,
            }) = event {
                state.last_char = ch;
            };
        };
        
        // Update
        if state.last_char == 'q' {
            state.running = false;
        }
        state.frame_count += 1;

        // Draw
        stdout
            .queue(terminal::Clear(terminal::ClearType::Purge)).unwrap()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(style::Print(format!("{}", state.frame_count))).unwrap()
            .queue(cursor::MoveTo(0, 5)).unwrap()
            .queue(style::Print(format!("{}", state.last_char))).unwrap()
            .flush().unwrap();
    }

    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap(); 
}   
