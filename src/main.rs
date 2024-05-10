mod actions;
mod models;
mod views;

extern crate cursive_table_view;
use crate::views::container;
use cursive::event::Key;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(container::containers_view());
    container::update_containers_view(&mut siv);
    siv.add_global_callback(Key::F5, |s| container::update_containers_view(s));
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
