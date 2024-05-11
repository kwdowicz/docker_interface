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
use log::info;

extern crate fern;
extern crate log;
extern crate chrono;

fn main() {
    setup_logging().expect("Failed to initialize logging.");
    info!("Started DockerFace");
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

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log")?)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
