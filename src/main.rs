extern crate cursive_table_view;

use cursive::{
    align::HAlign,
    event::Key,
    views::{Button, Dialog, TextView},
    CbSink, Cursive, CursiveRunnable,
};
use reqwest::blocking;
use reqwest::Error;
mod models;
use cursive::traits::*;
use cursive_table_view::{TableColumn, TableView, TableViewItem};
use models::container::{Container, Containers};
mod views;
use views::container::ContainerColumn;

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

    let mut table = TableView::<Container, ContainerColumn>::new()
        .column(ContainerColumn::Id, "Id", |c| c.width_percent(20))
        .column(ContainerColumn::Image, "Image", |c| c.width_percent(20))
        .column(ContainerColumn::Command, "Command", |c| {
            c.align(HAlign::Left)
        });

    table.set_on_submit(move |siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name(
                "containers_table",
                move |table: &mut TableView<Container, ContainerColumn>| {
                    format!("{:?}", table.borrow_item(index).unwrap())
                },
            )
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row))
                .button("Back", |s| {
                    s.pop_layer();
                })
                .button("Stop", move |s| {
                    s.call_on_name(
                        "containers_table",
                        |table: &mut TableView<Container, ContainerColumn>| {
                            let item = table.borrow_item(index).unwrap();
                            let container_id = item.id.clone();
                            match stop_container(&container_id.unwrap()) {
                                Ok(_) => {
                                    table.remove_item(index);
                                }
                                Err(e) => eprintln!("Failed to stop container: {}", e),
                            }
                        },
                    );
                    s.pop_layer();
                }),
        );
    });
    siv.add_layer(
        Dialog::around(table.with_name("containers_table").min_size((50, 20)))
            .title("Containers")
            .full_width(),
    );
    update_containers_view(&mut siv);

    siv.add_global_callback(Key::F5, |s| update_containers_view(s));
    siv.run();
}
