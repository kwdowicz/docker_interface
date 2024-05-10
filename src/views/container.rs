use crate::models::container::Container;
use cursive::{align::HAlign, views::{Dialog, ResizedView, TextView}, Cursive};
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;
use cursive::traits::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ContainerColumn {
    Id,
    Image,
    Command,
}

impl TableViewItem<ContainerColumn> for Container {
    fn to_column(&self, column: ContainerColumn) -> String {
        match column {
            ContainerColumn::Id => self.id.clone().unwrap_or("NO_ID".to_string()),
            ContainerColumn::Image => self.image.clone().unwrap_or("NO_IMAGE".to_string()),
            ContainerColumn::Command => self.command.clone().unwrap_or("NO_COMMAND".to_string()),
        }
    }

    fn cmp(&self, other: &Self, column: ContainerColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            ContainerColumn::Id => self
                .id
                .as_ref()
                .unwrap_or(&"NO_ID".to_string())
                .cmp(&other.id.as_ref().unwrap_or(&"NO_ID".to_string())),
            ContainerColumn::Image => self
                .image
                .as_ref()
                .unwrap_or(&"NO_IMAGE".to_string())
                .cmp(&other.image.as_ref().unwrap_or(&"NO_IMAGE".to_string())),
            ContainerColumn::Command => self
                .command
                .as_ref()
                .unwrap_or(&"NO_COMMAND".to_string())
                .cmp(&other.command.as_ref().unwrap_or(&"NO_COMMAND".to_string())),
        }
    }
}

pub fn containers_view() -> ResizedView<Dialog> {
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
                            // match stop_container(&container_id.unwrap()) {
                            //     Ok(_) => {
                            //         table.remove_item(index);
                            //     }
                            //     Err(e) => eprintln!("Failed to stop container: {}", e),
                            // }
                        },
                    );
                    s.pop_layer();
                }),
        );
    });

    Dialog::around(table.with_name("containers_table").min_size((50, 20)))
            .title("Containers")
            .full_width()
}
