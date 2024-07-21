use std::io::{stdout, Stdout, Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal;
use crossterm::style;
use crossterm::event;
use crossterm::cursor;
use rand::rngs::ThreadRng;
use rand::Rng;

use termod::character::Character;
use termod::buffer::Buffer;

fn main() {
    // setup
    // - raw mode
    // - alternate screen

    let mut rng: ThreadRng = rand::thread_rng();
    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    // build a block of random chars
    // - random chars
    // - random attrs
    // - random colours
    let (width, height) = terminal::size().unwrap();

    let mut main_buffer: Buffer;
    let mut prev_buffer = Buffer::new(width, height, 0, 0);

    let mut bg_buffer= Buffer::new(width, height, 0, 0);
    let mut input_buffer = Buffer::new(width / 2, height / 2, 10, 5);
    let mut debug_buffer= Buffer::new(20,10,0, 0);
    // run a loop
    //   - queue clearing the screen
    //   - queue printing each of those chars
    //   - flush stdout

    let mut frame_count: u32 = 0;
    loop {
        // event
        if event::poll(std::time::Duration::from_millis(30)).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => { break },
                        event::KeyCode::Left => { input_buffer.x -= 1 },
                        event::KeyCode::Right => { input_buffer.x += 1 },
                        event::KeyCode::Up => { input_buffer.y -= 1 },
                        event::KeyCode::Down => { input_buffer.y += 1 },
                        _ => {}
                    }
                },
                _ => {}
            };
        };

        // update
        for i in 0..bg_buffer.len() {
            let choice = rng.gen_range(0..1000);

            if choice == 0 {
                bg_buffer[i] = Character::random(&mut rng);
            };
        };
        let fps_chars = Character::vec_from_string(&format!("Frame Count: {}", frame_count));
        debug_buffer.insert_char_slice(0, &fps_chars);

        // draw
        input_buffer = input_buffer.merge(&debug_buffer).unwrap();
        main_buffer = bg_buffer.merge(&input_buffer).unwrap();
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
        frame_count += 1;
    };

    // clean up
    stdout.flush().unwrap();
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}
