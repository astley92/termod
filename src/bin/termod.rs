use std::io::{stdout, Stdout, Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal;
use crossterm::style;
use crossterm::event;
use crossterm::cursor;

use termod::buffer::Buffer;
use termod::character::Character;
use termod::widget::WidgetTrait;
use termod::{character, colours, dashboard_widget, git_widget, widget};

fn main() {
    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (width, height) = terminal::size().unwrap();

    let mut main_buffer= Buffer::new(width, height);
    let mut prev_buffer = Buffer::new(width, height);
    widget::add_buffer_border(&mut main_buffer, colours::GREY);
    
    let mut dashboard_widget = dashboard_widget::new(width-2, height-2, 0, 0);
    dashboard_widget.init();

    let mut git_widget = git_widget::new(width-2, height-2, 0, 0);
    git_widget.init();

    let mut widgets: Vec<Box<dyn WidgetTrait>> = vec![
        Box::new(dashboard_widget),
        Box::new(git_widget),
    ];
    let mut active_widget = 0;

    loop {
        // event
        if event::poll(std::time::Duration::from_millis(30)).unwrap() {
            let event = event::read().unwrap();
            widgets[active_widget].handle_event(&event);
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => { break },
                        event::KeyCode::Tab => { active_widget += 1; active_widget = active_widget % widgets.len() }
                        _ => {}
                    }
                },
                _ => {}
            };
        };

        // update
        widgets[active_widget].update();

        // draw
        widgets[active_widget].draw();
        let dashboard_buffer = widgets[active_widget].generate_buffer();
        let insert_pos = width + 1;
        main_buffer = main_buffer.merge(insert_pos as usize, &dashboard_buffer).unwrap();
        let mut title_str_pos = 2;
        for i in 0..widgets.len() {
            let title_str = widgets[i].get_title();
            let mut title_chars = Character::vec_from_string(title_str);
            if i == active_widget {
                for c in 0..title_chars.len() {
                    title_chars[c].highlight();
                }
            }

            for j in 0..title_chars.len() {
                main_buffer[title_str_pos + j] = title_chars[j].clone();
            }

            title_str_pos += title_chars.len() + 1;
        }

        stdout
            .queue(terminal::Clear(terminal::ClearType::Purge)).unwrap()
            .queue(cursor::MoveTo(0,0)).unwrap();

        for i in 0..main_buffer.len() {
            let character = &main_buffer[i];
            let prev_buffer_char = &prev_buffer[i];

            if character == prev_buffer_char {
                continue;
            };

            let x = i % width as usize;
            let y = i / width as usize;
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                .queue(style::SetAttributes(character.attributes)).unwrap()
                .queue(style::SetForegroundColor(character.colour)).unwrap()
                .queue(style::Print(character.c)).unwrap()
                .queue(style::ResetColor).unwrap()
                .queue(style::SetAttribute(style::Attribute::Reset)).unwrap();
        };

        stdout.flush().unwrap();
        prev_buffer = main_buffer.clone();
    };

    // clean up
    stdout.flush().unwrap();
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}
