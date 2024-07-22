use termod::{dashboard_widget, git_widget};
use termod::widget::WidgetTrait;

fn main() {
    let mut widgets: Vec<Box<dyn WidgetTrait>> = vec![];
    let dashboard_widget = dashboard_widget::new(10, 10, 0, 0);
    let second_widget= git_widget::new(10, 10, 0, 0);
   
    widgets.push(Box::new(dashboard_widget));
    widgets.push(Box::new(second_widget));

    widgets[0].init();
    println!("{}", widgets[0].get_title());
    println!("{}", widgets[1].get_title());
}
