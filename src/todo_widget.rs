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
    added_at: chrono::DateTime<chrono::Utc>,
    working: bool,
}

impl TodoItem {
    pub fn calculate_age(&self) -> chrono::TimeDelta {
        return chrono::Utc::now() - self.added_at;
    }
}

struct TodoItems {
    active_section: Section,
    todo_items: Vec<TodoItem>,
    done_items: Vec<TodoItem>,
    removed_items: Vec<TodoItem>,
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
            removed_items: vec![],
            highlighted_todo_item: 0,
            highlighted_done_item: 0,
        }
    }

    pub fn empty() -> TodoItems {
        return TodoItems {
            active_section: Section::TODO,
            todo_items: vec![],
            done_items: vec![],
            removed_items: vec![],
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

    pub fn remove_selected_item(&mut self) {
        if self.active_section == Section::TODO && self.todo_items.len() > 0 {
            let mut item = self.todo_items.remove(self.highlighted_todo_item);
            item.working = false;
            self.removed_items.push(item);
            if self.todo_items.len() > 0 && self.highlighted_todo_item > self.todo_items.len()-1 {
                self.highlighted_todo_item -= 1;
            }
        } else if self.active_section == Section::DONE && self.done_items.len() > 0 {
            let item = self.done_items.remove(self.highlighted_done_item);
            self.removed_items.push(item);
            if self.done_items.len() > 0 && self.highlighted_done_item > self.done_items.len()-1 {
                self.highlighted_done_item -= 1;
            }
        }
    }

    pub fn toggle_selected_item_completeness(&mut self) {
        if self.active_section == Section::TODO && self.todo_items.len() > 0 {
            let mut item = self.todo_items.remove(self.highlighted_todo_item);
            item.working = false;
            item.completed_at = Some(chrono::Utc::now());
            self.done_items.insert(0, item);
            if self.todo_items.len() > 0 && self.highlighted_todo_item > self.todo_items.len()-1 {
                self.highlighted_todo_item -= 1;
            }
        } else if self.active_section == Section::DONE && self.done_items.len() > 0 {
            let mut item = self.done_items.remove(self.highlighted_done_item);
            item.completed_at = None;
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

    pub fn undo_last_remove(&mut self) {
        if self.removed_items.len() < 1 {
            return;
        };

        let item = self.removed_items.pop().unwrap();
        if item.completed_at == None {
            self.todo_items.push(item);
        } else {
            self.done_items.push(item);
        }
    }
}

pub struct TodoState {
    main_buffer: Buffer,
    update_main_buffer: bool,
    items: TodoItems,
    item_seperator_height: usize,
    ages_last_calculated_at: std::time::Instant,
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
        TodoItem { 
            title: "Buy groceries".to_string(),
            added_at: chrono::Utc::now(),
            completed_at: None,
            working: false
        },
        TodoItem { 
            title: "Answer emails".to_string(), 
            added_at: chrono::DateTime::parse_from_rfc3339("2024-04-01T13:39:57Z").unwrap().to_utc(),
            completed_at: None,
            working: false
        },
        TodoItem { 
            title: "Write code".to_string(), 
            added_at: chrono::DateTime::parse_from_rfc3339("2023-04-01T13:39:57Z").unwrap().to_utc(),
            completed_at: Some(chrono::Utc::now() - std::time::Duration::from_secs(100)), 
            working: false 
        },
        TodoItem { 
            title: "Feed dogs".to_string(), 
            added_at: chrono::DateTime::parse_from_rfc3339("2024-04-01T13:39:57Z").unwrap().to_utc(),
            completed_at: Some(chrono::Utc::now()), 
            working: false 
        },
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
                event::KeyCode::Char('d') => { myself.state.items.remove_selected_item() },
                event::KeyCode::Char('u') => { myself.state.items.undo_last_remove() },
                _ => {}
            }
        },
        _ => {}
    }
}

fn update(myself: &mut Widget<TodoState>) {
    if std::time::Instant::now() - myself.state.ages_last_calculated_at > std::time::Duration::from_secs(1) {
        myself.state.update_main_buffer = true;
        myself.state.ages_last_calculated_at = std::time::Instant::now();
    };
}

fn draw(myself: &mut Widget<TodoState>) {
    if !myself.state.update_main_buffer { return };
   
    let all_items: &TodoItems = &myself.state.items;
    myself.state.main_buffer.clear();
    // Draw todo items
    let mut i = 0;
    for item in &all_items.todo_items {
        let attributes= if all_items.active_section == Section::TODO && all_items.highlighted_todo_item == i { 
            Some(style::Attributes::from(style::Attribute::Reverse)) 
        } else { 
            None 
        };

        let age: chrono::TimeDelta = item.calculate_age();
        let age_days= age.num_days();
        let age_hours= (age - chrono::TimeDelta::days(age_days)).num_hours();
        let age_minutes= (age - chrono::TimeDelta::days(age_days) - chrono::TimeDelta::hours(age_hours)).num_minutes();
        let age_seconds = (age - chrono::TimeDelta::days(age_days) - chrono::TimeDelta::hours(age_hours) - chrono::TimeDelta::minutes(age_minutes)).num_seconds();
        let mut title_prefix_chars= character::Character::vec_from_string(
            &format!("{:0>3}:{:0>2}:{:0>2}:{:0>2} - ", age_days, age_hours, age_minutes, age_seconds), 
            Some(colours::GREY), 
            None,
        );
        let mut title_chars = character::Character::vec_from_string(
            &item.title, 
            if item.working { Some(colours::ORANGE) } else { None }, 
            attributes
        );
        title_prefix_chars.append(&mut title_chars);
        myself.state.main_buffer.insert_char_slice(i * myself.width as usize, &title_prefix_chars);
        i += 1;
    };

    let seperator_string = "-".repeat(myself.width as usize);
    let seperator_chars = character::Character::vec_from_string(&seperator_string, None, None);
    myself.state.main_buffer.insert_char_slice(myself.state.item_seperator_height * myself.width as usize, &seperator_chars);

    // Draw done items
    let mut i = 0;
    for item in &all_items.done_items {
        let attributes= if all_items.active_section == Section::DONE && all_items.highlighted_done_item == i { 
            Some(style::Attributes::from(style::Attribute::Reverse)) 
        } else { 
            None 
        };

        let ttc: chrono::TimeDelta = item.completed_at.unwrap() - item.added_at;
        let ttc_days= ttc.num_days();
        let ttc_hours= (ttc- chrono::TimeDelta::days(ttc_days)).num_hours();
        let ttc_minutes= (ttc- chrono::TimeDelta::days(ttc_days) - chrono::TimeDelta::hours(ttc_hours)).num_minutes();
        let ttc_seconds = (ttc- chrono::TimeDelta::days(ttc_days) - chrono::TimeDelta::hours(ttc_hours) - chrono::TimeDelta::minutes(ttc_minutes)).num_seconds();
        
        let mut title_prefix_chars= character::Character::vec_from_string(
            &format!("{:0>3}:{:0>2}:{:0>2}:{:0>2} - ", ttc_days, ttc_hours, ttc_minutes, ttc_seconds), 
            Some(colours::GREY), 
            None,
        );
        let mut title_chars = character::Character::vec_from_string(
            &item.title,
            None, 
            attributes
        );

        title_prefix_chars.append(&mut title_chars);

        let line_number = i + myself.state.item_seperator_height + 1;
        if line_number < myself.height as usize {
            myself.state.main_buffer.insert_char_slice(line_number * myself.width as usize, &title_prefix_chars);
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
            ages_last_calculated_at: std::time::Instant::now() - std::time::Duration::from_secs(10),
        }
    };
}
