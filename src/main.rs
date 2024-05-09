extern crate cursive_table_view;

use cursive::{align::HAlign, views::{Dialog, TextView}};
use reqwest::Error;
mod models;
use models::container::{Containers, Container};
use cursive_table_view::{TableViewItem, TableColumn, TableView};
use cursive::traits::*;
mod views;
use views::container::ContainerColumn;

async fn fetch_containers() -> Result<Containers, Error> {
    let url = "http://localhost:2375/containers/json";
    let containers = reqwest::get(url)
    .await?
    .json::<Containers>()
    .await?;
    Ok(containers)
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

    siv.add_layer(Dialog::around(table.with_name("containers_table").min_size((50, 20))).title("Containers").full_width());
    siv.run();

}
