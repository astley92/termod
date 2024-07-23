use crate::buffer::Buffer;
use crate::character;
use crate::widget::{Widget, WidgetTrait};
use crossterm::{event, style};

struct TodoItem {
    title: String,
}

pub struct TodoState {
    main_buffer: Buffer,
    todo_items: Vec<TodoItem>,
    highlighted_item: usize,
}

impl WidgetTrait for Widget<TodoState> {
    fn init(&mut self) {self.init()}
    fn handle_event(&mut self, event_to_handle: &event::Event) {self.handle_event(event_to_handle)}
    fn update(&mut self) {self.update()}
    fn draw(&mut self) {self.draw()}
    fn generate_buffer(&mut self) -> Buffer {self.generate_buffer()}
    fn get_title(&self) -> &String {&self.title}
}

fn init(myself: &mut Widget<TodoState>) {
    myself.state.todo_items = vec![
        TodoItem { title: "Buy groceries".to_string() },
        TodoItem { title: "Answer emails".to_string() },
    ]
}

fn event(myself: &mut Widget<TodoState>, event_to_handle: &event::Event) {
    match event_to_handle {
        event::Event::Key(event) => {
            match event.code {
                event::KeyCode::Up => {
                    if myself.state.highlighted_item > 0 {
                        myself.state.highlighted_item -= 1;
                    }
                },
                event::KeyCode::Down => { 
                    myself.state.highlighted_item = (myself.state.highlighted_item + 1).min(myself.state.todo_items.len()-1)
                },
                _ => {}
            }
        },
        _ => {}
    }
}

fn update(myself: &mut Widget<TodoState>) {
}

fn draw(myself: &mut Widget<TodoState>) {
    for i in 0..myself.state.todo_items.len() {
        let attributes= if i == myself.state.highlighted_item as usize { 
            Some(style::Attributes::from(style::Attribute::Reverse)) 
        } else { 
            None 
        };

        let title_chars = character::Character::vec_from_string(
            &myself.state.todo_items[i].title, 
            None, 
            attributes
        );
        myself.state.main_buffer.insert_char_slice(i * myself.width as usize, &title_chars);
    }
}

fn generate_buffer(myself: &mut Widget<TodoState>) -> Buffer {
    return myself.state.main_buffer.clone();
}

pub fn new(width: u16, height: u16, x: usize, y: usize) -> Widget<TodoState> {
    return Widget {
        width,
        height,
        title: "Todo".to_string(),
        init_fn: init,
        event_fn: event,
        update_fn: update,
        draw_fn: draw,
        generate_buffer_fn: generate_buffer,
        state: TodoState { 
            main_buffer: Buffer::new(width, height),
            todo_items: vec![],
            highlighted_item: 0,
        }
    };
}
