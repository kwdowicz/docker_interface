use crate::actions::container::{fetch_containers, stop_container};
use crate::models::container::Container;
use crate::views::bars::update_status_bar;
use cursive::traits::*;
use cursive::{
    views::{Dialog, ResizedView, TextView},
    Cursive,
};
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;

use super::text::Status;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ContainerColumn {
    Id,
    Image,
    Command,
    Name,
}

impl TableViewItem<ContainerColumn> for Container {
    fn to_column(&self, column: ContainerColumn) -> String {
        match column {
            ContainerColumn::Id => self.id_short(),
            ContainerColumn::Image => self.image(),
            ContainerColumn::Command => self.command(),
            ContainerColumn::Name => self.first_name(),
        }
    }

    fn cmp(&self, other: &Self, column: ContainerColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            ContainerColumn::Id => self.id_short().cmp(&other.id_short()),
            ContainerColumn::Image => self.image().cmp(&other.image()),
            ContainerColumn::Command => self.command().cmp(&other.command()),
            ContainerColumn::Name => self.first_name().cmp(&other.first_name()),
        }
    }
}

pub fn update_containers_view(siv: &mut Cursive) {
    update_status_bar(siv, Status::Refreshing);
    let mut container_rows = Vec::new();
    match fetch_containers() {
        Ok(containers) => {
            update_status_bar(siv, Status::Ready);
            for container in containers {
                container_rows.push(container);
            }
            siv.call_on_name(
                "containers_table",
                |table: &mut TableView<Container, ContainerColumn>| table.set_items(container_rows),
            );
        }
        Err(_e) => {
            update_status_bar(siv, Status::NoDocker);
        }
    }
}

fn create_containers_table() -> TableView<Container, ContainerColumn> {
    let table = TableView::<Container, ContainerColumn>::new()
        .column(ContainerColumn::Id, "ID", |c| c.width(13))
        .column(ContainerColumn::Image, "IMAGE", |c| c.width_percent(20))
        .column(ContainerColumn::Command, "COMMAND", |c| c.width_percent(30))
        .column(ContainerColumn::Name, "NAME", |c| c.width_percent(20));
    table
}

fn handle_table_submission(siv: &mut Cursive, index: usize) {
    siv.set_user_data(index);
    let value = siv
        .call_on_name(
            "containers_table",
            move |table: &mut TableView<Container, ContainerColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            },
        )
        .unwrap();

    siv.add_layer(create_details_dialog(value));
}

fn create_details_dialog(details: String) -> Dialog {
    Dialog::around(TextView::new(details))
        .title("Container details")
        .button("Back", |s| {
            s.pop_layer();
        })
        .button("Stop Container", |s| handle_stop_container_action(s))
}

fn handle_stop_container_action(s: &mut Cursive) {
    let current_index = *s.user_data::<usize>().unwrap();

    update_status_bar(s, Status::StoppingContainer);
    let mut table = s
        .find_name::<TableView<Container, ContainerColumn>>("containers_table")
        .unwrap();
    let maybe_item = table.borrow_item(current_index);
    if let Some(item) = maybe_item {
        let container_id = item.id.clone();

        match stop_container(&container_id.unwrap()) {
            Ok(_) => {
                table.remove_item(current_index);
                s.pop_layer();
                update_status_bar(s, Status::ContainerStopped);
            }
            Err(e) => {
                update_status_bar(s, Status::ErrorStoppingContainer(e.to_string()));
            }
        }
    }
}

pub fn containers_view() -> ResizedView<Dialog> {
    let mut table = create_containers_table();
    table.set_on_submit(move |siv, _row, index| handle_table_submission(siv, index));

    Dialog::around(table.with_name("containers_table").min_size((50, 20)))
        .title("Containers")
        .full_width()
}
