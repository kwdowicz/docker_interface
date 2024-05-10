use std::cmp::Ordering;
use cursive_table_view::TableViewItem;
use crate::models::container::Container;

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
            ContainerColumn::Id => self.id.as_ref().unwrap_or(&"NO_ID".to_string()).cmp(&other.id.as_ref().unwrap_or(&"NO_ID".to_string())),
            ContainerColumn::Image => self.image.as_ref().unwrap_or(&"NO_IMAGE".to_string()).cmp(&other.image.as_ref().unwrap_or(&"NO_IMAGE".to_string())),
            ContainerColumn::Command => self.command.as_ref().unwrap_or(&"NO_COMMAND".to_string()).cmp(&other.command.as_ref().unwrap_or(&"NO_COMMAND".to_string())),
        }
    }
}