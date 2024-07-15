use crossterm::terminal;
use crossterm::style;
use crossterm::cursor;
use crossterm::event;
use crossterm::{QueueableCommand, ExecutableCommand};
use std::io::{stdout, Write};

mod buffer;
mod character;
mod widget;
mod main_widget;
mod debug_widget;
mod utils;
mod habit_track_widget;

use buffer::Buffer;
use widget::Widget;
use main_widget::MainWidget;
use debug_widget::DebugWidget;

fn main() {
    let mut running = true;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (mut w_width, mut w_height) = terminal::size().unwrap();
    let mut main_widget = MainWidget::new(0, 0, w_width, w_height);
    let mut debug_widget = DebugWidget::new(1, w_height-11, w_width-2, 10);

    while running {
        main_widget.reset_buffer();
        debug_widget.reset_buffer();

        // Event
        if event::poll(std::time::Duration::ZERO).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => { running = false },
                        _ => { main_widget.handle_event(crossterm::event::Event::Key(event)) }
                    }
                },
                event::Event::Resize(new_width, new_height) => {
                    w_width = new_width;
                    w_height = new_height;
                    main_widget.resize(new_width, new_height);
                    debug_widget.resize(new_width-2, 10);
                    debug_widget.move_to(1, new_height-11);
                },
                _ => {}
            };
        };

        // Update
        main_widget.update();
        debug_widget.update();

        // Draw
        main_widget.draw();
        debug_widget.draw();
        stdout.queue(terminal::Clear(terminal::ClearType::Purge)).unwrap();

        let mut buffer = Buffer::empty(w_width, w_height);
        buffer.insert_buffer(main_widget.x, main_widget.y, &main_widget.buffer);
        buffer.insert_buffer(debug_widget.x, debug_widget.y, &debug_widget.buffer);

        for i in 0..buffer.chars.len() {
            let character = &buffer.chars[i];
            let linear_pos = i;
            let x = linear_pos % buffer.width as usize;
            let y = linear_pos / buffer.width as usize;
             
            for attr in &character.attributes {
                stdout.queue(style::SetAttribute(*attr)).unwrap();
            };
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                .queue(style::Print(format!("{}", character.character as char))).unwrap()
                .queue(style::SetAttribute(style::Attribute::Reset)).unwrap();
        }

        stdout.flush().unwrap();
    }
    
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap(); 
}

