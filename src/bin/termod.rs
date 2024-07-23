use std::io::{stdout, Stdout, Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal;
use crossterm::style;
use crossterm::event;
use crossterm::cursor;

use termod::buffer::Buffer;
use termod::character::Character;
use termod::widget::{self, WidgetTrait};
use termod::{colours, dashboard_widget, todo_widget};

fn main() {
    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let (width, height) = terminal::size().unwrap();

    let mut main_buffer= Buffer::new(width, height);
    let mut prev_buffer = Buffer::new(width, height);
    widget::add_buffer_border(&mut main_buffer, colours::LIGHT_GREY);
    
    let mut dashboard_widget = dashboard_widget::new(width-2, height-2, 0, 0);
    dashboard_widget.init();

    let mut todo_widget = todo_widget::new(width-2, height-2, 0, 0);
    todo_widget.init();

    let mut widgets: Vec<Box<dyn WidgetTrait>> = vec![
        Box::new(todo_widget),
        Box::new(dashboard_widget),
    ];
    let mut active_widget_index = 0;
    let mut active_widget_changed= true;

    loop {
        // event
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            let event = event::read().unwrap();
            widgets[active_widget_index].handle_event(&event);
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => { break },
                        event::KeyCode::Tab => { 
                            active_widget_index += 1; 
                            active_widget_index = active_widget_index % widgets.len();
                            active_widget_changed = true;
                        }
                        _ => {}
                    }
                },
                _ => {}
            };
        };

        // update
        widgets[active_widget_index].update();

        // draw
        widgets[active_widget_index].draw();
        let active_widget_buffer = widgets[active_widget_index].generate_buffer();
        let insert_pos = width + 1;
        main_buffer = main_buffer.merge(insert_pos as usize, &active_widget_buffer).unwrap();

        if active_widget_changed {
            let mut title_str_pos = 2;
            for i in 0..widgets.len() {
                let title_str = widgets[i].get_title();
                let mut title_chars = Character::vec_from_string(title_str, None, None);
                if i == active_widget_index {
                    for c in 0..title_chars.len() {
                        title_chars[c].highlight();
                    }
                }

                for j in 0..title_chars.len() {
                    main_buffer[title_str_pos + j] = title_chars[j].clone();
                }

                title_str_pos += title_chars.len() + 1;
            }
            active_widget_changed = false;
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
