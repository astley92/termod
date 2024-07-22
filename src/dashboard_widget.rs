use crate::buffer::Buffer;
use crate::character::Character;
use crate::colours;
use crate::widget::{self, Widget, WidgetTrait};
use crossterm::event;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct DashboardState {
    pub frame_count: usize,
    pub debug_buffer: Buffer,
    pub bg_buffer: Buffer,
    pub debug_x: usize,
    pub debug_y: usize,
}

impl WidgetTrait for Widget<DashboardState> {
    fn init(&mut self) {self.init()}
    fn handle_event(&mut self, event_to_handle: &event::Event) {self.handle_event(event_to_handle)}
    fn update(&mut self) {self.update()}
    fn draw(&mut self) {self.draw()}
    fn generate_buffer(&mut self) -> Buffer {self.generate_buffer()}
    fn get_title(&self) -> &String {&self.title}
}

fn dashboard_init(myself: &mut Widget<DashboardState>) {
    let mut debug_buffer = Buffer::new(20, 10);
    widget::add_buffer_border(&mut debug_buffer, colours::LIGHT_GREY);
    myself.state.debug_buffer = debug_buffer;
    myself.state.bg_buffer = Buffer::new(myself.width, myself.height);
}

fn dashboard_event(myself: &mut Widget<DashboardState>, event_to_handle: &event::Event) {
    match event_to_handle {
        event::Event::Key(event) => {
            match event.code {
                event::KeyCode::Left => { myself.state.debug_x -= 1 },
                event::KeyCode::Right => { myself.state.debug_x += 1 },
                event::KeyCode::Up => { myself.state.debug_y -= 1 },
                event::KeyCode::Down => { myself.state.debug_y += 1 },
                _ => {}
            }
        },
        _ => {}
    }
}

fn dashboard_update(myself: &mut Widget<DashboardState>) {
    // update
    myself.state.frame_count += 1;
    let mut rng: ThreadRng = rand::thread_rng();
    let mut bg_buffer = myself.state.bg_buffer.clone();
    for i in 0..bg_buffer.len() {
        let choice = rng.gen_range(0..1000);

        if choice == 0 {
            bg_buffer[i] = Character::random(&mut rng);
        };
    };
    myself.state.bg_buffer = bg_buffer;
}

fn dashboard_draw(myself: &mut Widget<DashboardState>) {
    let mut debug_buffer = myself.state.debug_buffer.clone();
    let fps_chars = Character::vec_from_string(&format!("Frame Count: {}", myself.state.frame_count), None);
    debug_buffer.insert_char_slice(0, &fps_chars);
    myself.state.debug_buffer = debug_buffer;
}

fn dashboard_generate_buffer(myself: &mut Widget<DashboardState>) -> Buffer {
    let bg_buffer = &myself.state.bg_buffer;
    let pos_to_insert = myself.state.debug_x + myself.state.debug_y * myself.width as usize;
    let debug_buffer = &myself.state.debug_buffer;
    bg_buffer.merge(pos_to_insert, &debug_buffer).unwrap()
}

pub fn new(width: u16, height: u16, x: usize, y: usize) -> Widget<DashboardState> {
    return Widget {
        width,
        height,
        title: "Dashboard".to_string(),
        init_fn: dashboard_init,
        event_fn: dashboard_event,
        update_fn: dashboard_update,
        draw_fn: dashboard_draw,
        generate_buffer_fn: dashboard_generate_buffer,
        state: DashboardState { 
            frame_count: 0, 
            debug_x: x, 
            debug_y: y,
            bg_buffer: Buffer::new(0, 0),
            debug_buffer: Buffer::new(0, 0),
        }
    };
}
