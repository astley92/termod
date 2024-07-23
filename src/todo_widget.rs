use crate::buffer::Buffer;
use crate::{character, colours};
use crate::widget::{Widget, WidgetTrait};
use crossterm::{event, style};
use chrono;

#[derive(PartialEq)]
enum Section {
    TODO, 
    DONE,
}

struct TodoItem {
    title: String,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    working: bool,
}

struct TodoItems {
    active_section: Section,
    todo_items: Vec<TodoItem>,
    done_items: Vec<TodoItem>,
    highlighted_todo_item: usize,
    highlighted_done_item: usize,
}

impl TodoItems {
    pub fn new(items: Vec<TodoItem>) -> TodoItems {
        let mut todo_items: Vec<TodoItem> = vec![];
        let mut done_items: Vec<TodoItem>= vec![];
        for item in items {
            if item.completed_at == None {
                todo_items.push(item);
            } else {
                done_items.push(item);
            }
        }

        return TodoItems {
            active_section: Section::TODO,
            todo_items,
            done_items,
            highlighted_todo_item: 0,
            highlighted_done_item: 0,
        }
    }

    pub fn empty() -> TodoItems {
        return TodoItems {
            active_section: Section::TODO,
            todo_items: vec![],
            done_items: vec![],
            highlighted_todo_item: 0,
            highlighted_done_item: 0,
        }
    }

    pub fn next_item(&mut self) {
        if self.active_section == Section::TODO {
            if self.todo_items.len() > 0 && self.highlighted_todo_item < self.todo_items.len()-1 {
                self.highlighted_todo_item += 1;
            };
        } else {
            if self.done_items.len() > 0 && self.highlighted_done_item < self.done_items.len()-1 {
                self.highlighted_done_item += 1;
            };
        }; 
    }

    pub fn prev_item(&mut self) {
        if self.active_section == Section::TODO {

            if self.highlighted_todo_item > 0 {
                self.highlighted_todo_item -= 1;
            };
        } else {
            if self.highlighted_done_item > 0 {
                self.highlighted_done_item -= 1;
            };
        }; 
    }

    pub fn toggle_section(&mut self) {
        if self.active_section == Section::TODO {
            self.active_section = Section::DONE;
        } else {
            self.active_section = Section::TODO;
        }
    }

    pub fn toggle_selected_item_completeness(&mut self) {
        if self.active_section == Section::TODO {
            let item = self.todo_items.remove(self.highlighted_todo_item);
            self.done_items.insert(0, item);
            if self.todo_items.len() > 0 && self.highlighted_todo_item > self.todo_items.len()-1 {
                self.highlighted_todo_item -= 1;
            }
        } else {
            let item = self.done_items.remove(self.highlighted_done_item);
            self.todo_items.push(item);
            if self.done_items.len() > 0 && self.highlighted_done_item > self.done_items.len()-1 {
                self.highlighted_done_item -= 1;
            }
        }
    }

    pub fn toggle_selected_item_working(&mut self) {
        if self.active_section == Section::TODO {
            let item: &mut TodoItem = &mut self.todo_items[self.highlighted_todo_item];
            item.working = !item.working;
        }
    }
}

pub struct TodoState {
    main_buffer: Buffer,
    update_main_buffer: bool,
    items: TodoItems,
    item_seperator_height: usize,
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
    myself.state.items = TodoItems::new(vec![
        TodoItem { title: "Buy groceries".to_string(), completed_at: None ,working: false},
        TodoItem { title: "Answer emails".to_string(), completed_at: None ,working: false},
        TodoItem { title: "Write code".to_string(), completed_at: Some(chrono::Utc::now()), working: false },
        TodoItem { title: "Feed dogs".to_string(), completed_at: Some(chrono::Utc::now()), working: false },
    ]);
}

fn event(myself: &mut Widget<TodoState>, event_to_handle: &event::Event) {
    myself.state.update_main_buffer = true;
    match event_to_handle {
        event::Event::Key(event) => {
            match event.code {
                event::KeyCode::Up => { myself.state.items.prev_item() },
                event::KeyCode::Down => { myself.state.items.next_item() },
                event::KeyCode::Right | event::KeyCode::Left => { 
                    myself.state.items.toggle_section();
                    myself.state.item_seperator_height = myself.height as usize - myself.state.item_seperator_height;
                },
                event::KeyCode::Enter => { myself.state.items.toggle_selected_item_completeness() },
                event::KeyCode::Char('w') => { myself.state.items.toggle_selected_item_working() },
                _ => {}
            }
        },
        _ => {}
    }
}

fn update(_myself: &mut Widget<TodoState>) {
}

fn draw(myself: &mut Widget<TodoState>) {
    if !myself.state.update_main_buffer { return };
   
    let all_items: &TodoItems = &myself.state.items;
    myself.state.main_buffer.clear();
    // Draw todo items
    let mut i = 0;
    for item in &all_items.todo_items {
        let attributes= if all_items.highlighted_todo_item == i { 
            Some(style::Attributes::from(style::Attribute::Reverse)) 
        } else { 
            None 
        };

        let title_chars = character::Character::vec_from_string(
            &item.title, 
            if item.working { Some(colours::ORANGE) } else { None }, 
            attributes
        );
        myself.state.main_buffer.insert_char_slice(i * myself.width as usize, &title_chars);
        i += 1;
    };

    let seperator_string = "-".repeat(myself.width as usize);
    let seperator_chars = character::Character::vec_from_string(&seperator_string, None, None);
    myself.state.main_buffer.insert_char_slice(myself.state.item_seperator_height * myself.width as usize, &seperator_chars);

    // Draw done items
    let mut i = 0;
    for item in &all_items.done_items {
        let attributes= if all_items.highlighted_done_item == i { 
            Some(style::Attributes::from(style::Attribute::Reverse)) 
        } else { 
            None 
        };

        let title_chars = character::Character::vec_from_string(
            &item.title,
            None, 
            attributes
        );
        let line_number = i + myself.state.item_seperator_height + 1;
        if line_number < myself.height as usize {
            myself.state.main_buffer.insert_char_slice(line_number * myself.width as usize, &title_chars);
        }
        i += 1;
    };

    myself.state.update_main_buffer = false;
}

fn generate_buffer(myself: &mut Widget<TodoState>) -> Buffer {
    return myself.state.main_buffer.clone();
}

pub fn new(width: u16, height: u16, _x: usize, _y: usize) -> Widget<TodoState> {
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
            update_main_buffer: true,
            item_seperator_height: height as usize - 10,
            items: TodoItems::empty(),
        }
    };
}
