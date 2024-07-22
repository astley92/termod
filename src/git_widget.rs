use crate::buffer::Buffer;
use crate::widget::{self, Widget, WidgetTrait};
use crossterm::event;

pub struct GitState {
}

impl WidgetTrait for Widget<GitState> {
    fn init(&mut self) {self.init()}
    fn handle_event(&mut self, event_to_handle: &event::Event) {self.handle_event(event_to_handle)}
    fn update(&mut self) {self.update()}
    fn draw(&mut self) {self.draw()}
    fn generate_buffer(&mut self) -> Buffer {self.generate_buffer()}
    fn get_title(&self) -> &String {&self.title}
}

fn init(myself: &mut Widget<GitState>) {
}

fn event(myself: &mut Widget<GitState>, event_to_handle: &event::Event) {
}

fn update(myself: &mut Widget<GitState>) {
}

fn draw(myself: &mut Widget<GitState>) {
}

fn generate_buffer(myself: &mut Widget<GitState>) -> Buffer {
    Buffer::new(7, 7)
}

pub fn new(width: u16, height: u16, x: usize, y: usize) -> Widget<GitState> {
    return Widget {
        width,
        height,
        title: "GIT".to_string(),
        init_fn: init,
        event_fn: event,
        update_fn: update,
        draw_fn: draw,
        generate_buffer_fn: generate_buffer,
        state: GitState { }
    };
}
