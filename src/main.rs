mod actions;
mod models;
mod views;

extern crate cursive_table_view;
use crate::views::bars;
use crate::views::container;
use cursive::event::Key;
use views::bars::update_status_bar;
use views::text::Status;
use views::text::Text;

fn main() {
    let mut siv = cursive::default();
    siv.set_window_title(Text::AppTitle.text());
    bars::add_status_bar(&mut siv);
    bars::add_info_bar(&mut siv);
    siv.add_layer(container::containers_view());
    container::update_containers_view(&mut siv);
    update_status_bar(&mut siv, Status::Ready);
    siv.add_global_callback(Key::F5, |s| container::update_containers_view(s));
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
