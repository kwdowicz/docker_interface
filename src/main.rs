extern crate cursive_table_view;

use cursive::{align::HAlign, views::{Dialog, TextView}, Cursive};
use reqwest::Error;
mod models;
use models::container::{Containers, Container};
use cursive_table_view::{TableViewItem, TableColumn, TableView};
use cursive::traits::*;
mod views;
use views::container::ContainerColumn;
use tokio::sync::oneshot;

async fn fetch_containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let containers = reqwest::get(url)
    .await?
    .json::<Containers>()
    .await?;
    Ok(containers)
}

async fn stop_container(container_id: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:2375/containers/{}/stop", container_id);

    let res = client.post(url)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Container stopped successfully.");
    } else {
        println!("Failed to stop container: Status {}", res.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();
    
    let mut table = TableView::<Container, ContainerColumn>::new()
        .column(ContainerColumn::Id, "Id", |c| c.width_percent(20))
        .column(ContainerColumn::Command, "Command", |c| c.align(HAlign::Left));
        
    let mut container_rows = Vec::new();

    match fetch_containers().await {
        Ok(containers) => {
            for container in containers {
                container_rows.push(
                    container
                );
            }
            table.set_items(container_rows);
        },
        Err(e) => eprintln!("Failed to fetch containers: {}", e),
    }

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        
        let value = siv
            .call_on_name("containers_table", move |table: &mut TableView<Container, ContainerColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        println!("{:?}", value);

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row))
                .button("Close", move |s| {
                    s.call_on_name("containers_table", |table: &mut TableView<Container, ContainerColumn>| {
                        // Here I need to call stop_container
                        // If I get the result that container has stopped I need to update table and display message that
                        // it was indeed stopped
                        table.remove_item(index);
                    });
                    s.pop_layer();
                }),
        );
    });

    siv.add_layer(Dialog::around(table.with_name("containers_table").min_size((50, 20))).title("Containers").full_width());
    siv.run();

}
