extern crate cursive_table_view;

use cursive::{event::Key, Cursive};
use reqwest::blocking;
use reqwest::Error;
mod models;
use cursive_table_view::TableView;
use models::container::{Container, Containers};
mod views;
use views::container::{containers_view, ContainerColumn};

fn fetch_containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let response = blocking::get(url)?;
    let containers = response.json::<Containers>()?;
    Ok(containers)
}

fn update_containers_view(siv: &mut Cursive) {
    let mut container_rows = Vec::new();

    match fetch_containers() {
        Ok(containers) => {
            for container in containers {
                container_rows.push(container);
            }
            siv.call_on_name(
                "containers_table",
                |table: &mut TableView<Container, ContainerColumn>| table.set_items(container_rows),
            );
        }
        Err(e) => eprintln!("Failed to fetch containers: {}", e),
    }
}

fn stop_container(container_id: &String) -> Result<(), Error> {
    let client = blocking::Client::new();
    let url = format!("http://localhost:2375/containers/{}/stop", container_id);

    let res = client.post(url).send()?;
    if res.status().is_success() {
        println!("Container stopped successfully.");
    } else {
        println!("Failed to stop container: Status {}", res.status());
    }
    Ok(())
}

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(containers_view());
    update_containers_view(&mut siv);
    siv.add_global_callback(Key::F5, |s| update_containers_view(s));
    siv.run();
}
