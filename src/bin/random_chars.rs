use std::io::{stdout, Stdout, Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal;
use crossterm::style;
use crossterm::event;
use crossterm::cursor;
use rand::Rng;

#[derive(Clone, PartialEq)]
struct Character {
    c: char,
    attributes: style::Attributes,
    colour: style::Color,
}

impl Character {
    const ATTRIBUTES: [style::Attribute; 4] = [
        style::Attribute::Bold,
        style::Attribute::Dim,
        style::Attribute::Underlined,
        style::Attribute::Reverse,
    ];

    pub fn blank() -> Character {
        let mut attrs = style::Attributes::from(style::Attribute::Bold);
        attrs.unset(style::Attribute::Bold);
        return Character {
            c: ' ',
            colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
            attributes: attrs,
        }
    }
    
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Character {
        let rand_char_code: u8 = rand::thread_rng().gen_range(32..=126);
        if rand_char_code == 32 {
            return Character::blank();
        };
        let rand_char = rand_char_code as char;
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let colour = style::Color::Rgb {r: r, g: g, b: b};
        let attr_index = rng.gen_range(0..Self::ATTRIBUTES.len());
        let mut attrs = style::Attributes::from(style::Attribute::Bold);
        attrs.unset(style::Attribute::Bold);
        let attr = Self::ATTRIBUTES[attr_index];
        attrs.toggle(attr);
        return Character {
            c: rand_char,
            attributes: attrs,
            colour: colour,
        };
    }
}

#[derive(Clone)]
struct Buffer {
    width: u16,
    height: u16, 
    x: u16,
    y: u16,
    characters: Vec<Character>,
}

impl Buffer {
    pub fn new(width: u16, height: u16, x: u16, y: u16) -> Buffer {
        let mut chars = vec![];
        for _ in 0..width {
            for _ in 0..height {
                chars.push(Character::blank());
            }
        }
        return Buffer {
            width: width,
            height: height,
            x: x, 
            y: y,
            characters: chars,
        }
    }

    pub fn len(&self) -> usize {
        return self.characters.len();
    }
}

impl std::ops::Index<usize> for Buffer {
    type Output = Character;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.characters[i];
    }
}

impl std::ops::IndexMut<usize> for Buffer {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.characters[i];
    }
}

fn main() {
    // setup
    // - raw mode
    // - alternate screen

    let mut rng = rand::thread_rng();
    let mut stdout: Stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    // build a block of random chars
    // - random chars
    // - random attrs
    // - random colours
    let (width, height) = terminal::size().unwrap();
    
    let mut buffer = Buffer::new(width, height, 0, 0);
    let mut prev_buffer = Buffer::new(width, height, 0, 0);
    
    // run a loop
    //   - queue clearing the screen
    //   - queue printing each of those chars
    //   - flush stdout

    const DESIRED_FRAME_COUNT: usize = 1000;
    let mut times_taken: [f64; DESIRED_FRAME_COUNT] = [0.0; DESIRED_FRAME_COUNT];
    let mut count: usize = 0;
    while count < DESIRED_FRAME_COUNT {
        let start_time = std::time::Instant::now();

        // event
        if event::poll(std::time::Duration::ZERO).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => { break },
                        _ => {}
                    }
                },
                _ => {}
            };
        };

        // update
        for i in 0..buffer.len() {
            let choice = rng.gen_range(0..1000);

            if choice == 0 {
                buffer[i] = Character::random(&mut rng);
            };
        };

        // draw
        stdout
            .queue(terminal::Clear(terminal::ClearType::Purge)).unwrap()
            .queue(cursor::MoveTo(0,0)).unwrap();

        for i in 0..buffer.len() {
            let character = &buffer[i];
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
        prev_buffer = buffer.clone();

        let time_taken = start_time.elapsed().as_secs_f64();
        times_taken[count] = time_taken;
        count += 1;
    };

    // clean up
    stdout.flush().unwrap();
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();

    let total_time_taken: f64 = times_taken.iter().sum();
    let average_frame_time = total_time_taken / DESIRED_FRAME_COUNT as f64;
    let average_fps = 1 as f64 / average_frame_time;
    println!("{:?}", times_taken);
    println!("Width:{}\nHeight:{}\nTotal Char Count:{}\nTotal time taken: {}\nAverage frame time:{}\nAverage FPS:{}", width, height, width*height,total_time_taken, average_frame_time, average_fps);
}
