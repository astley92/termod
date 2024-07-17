use std::io::{stdout, Stdout, Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal;
use crossterm::style;
use crossterm::cursor;
use rand::Rng;

#[derive(Clone, PartialEq)]
struct Character {
    c: char,
    attributes: Vec<style::Attribute>,
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
        return Character {
            c: ' ',
            colour: style::Color::Rgb { r: 0, g: 0, b: 0 },
            attributes: vec![],
        }
    }
    
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Character {
        let rand_char_code: u8 = rand::thread_rng().gen_range(32..=126);
        let rand_char = rand_char_code as char;
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let colour = style::Color::Rgb {r: r, g: g, b: b};
        let attr_index = rng.gen_range(0..Self::ATTRIBUTES.len());
        let attr = Self::ATTRIBUTES[attr_index]; 
        return Character {
            c: rand_char,
            attributes: vec![attr],
            colour: colour,
        };
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
    let mut state: Vec<Character> = vec![];
    let mut prev_state: Vec<Character> = vec![];
    for _ in 0..width {
        for _ in 0..height {
            prev_state.push(Character::blank());
            state.push(Character::random(&mut rng));
        }
    }
    // run a loop
    //   - queue clearing the screen
    //   - queue printing each of those chars
    //   - flush stdout

    const DESIRED_FRAME_COUNT: usize = 1000;
    let mut times_taken: [f64; DESIRED_FRAME_COUNT] = [0.0; DESIRED_FRAME_COUNT];
    let mut count = 0;
    while count < DESIRED_FRAME_COUNT {
        let start_time = std::time::Instant::now();

        for i in 0..state.len() {
            let choice = rng.gen_range(0..1000);
            if choice < 2 {
                state[i] = Character::random(&mut rng);
            };
        };

        stdout
            .queue(terminal::Clear(terminal::ClearType::Purge)).unwrap()
            .queue(cursor::MoveTo(0,0)).unwrap();

        for i in 0..state.len() {
            let character = &state[i];
            let prev_state_char = &prev_state[i];

            if character == prev_state_char {
                continue;
            };

            let x = i % width as usize;
            let y = i / width as usize;
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                .queue(style::SetAttribute(character.attributes[0])).unwrap()
                .queue(style::SetForegroundColor(character.colour)).unwrap()
                .queue(style::Print(character.c)).unwrap()
                .queue(style::ResetColor).unwrap()
                .queue(style::SetAttribute(style::Attribute::Reset)).unwrap();
        };

        stdout.flush().unwrap();
        prev_state = state.clone();

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
    println!("Width:{}\nHeight:{}\nTotal Char Count:{}\nTotal time taken: {}\nAverage frame time:{}\nAverage FPS:{}", width, height, width*height,total_time_taken, average_frame_time, average_fps);
}
