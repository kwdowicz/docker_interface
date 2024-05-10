use crate::views::text::Text;
use cursive::Cursive;
use cursive::{
    traits::{Nameable as _, Resizable as _},
    view::View as _,
    views::{FixedLayout, Layer, OnLayoutView, TextView},
    {Rect, Vec2},
};

use super::text::Status;

pub fn update_status_bar(siv: &mut Cursive, status: Status) {
    siv.call_on_name("status_bar", |text: &mut TextView| text.set_content(status.text()));
}

pub fn add_info_bar(siv: &mut Cursive) {
    siv.screen_mut().add_transparent_layer(
        OnLayoutView::new(
            FixedLayout::new().child(
                Rect::from_point(Vec2::zero()),
                Layer::new(TextView::new(Text::BottomBar.text()).with_name("info_bar")).full_width(),
            ),
            |layout, size| {
                layout.set_child_position(0, Rect::from_size((0, size.y - 1), (size.x, 1)));
                layout.layout(size);
            },
        )
        .full_screen(),
    );
}

pub fn add_status_bar(siv: &mut Cursive) {
    siv.screen_mut().add_transparent_layer(
        OnLayoutView::new(
            FixedLayout::new().child(
                Rect::from_point(Vec2::zero()),
                Layer::new(TextView::new("Status: Loaded").with_name("status_bar")).full_width(),
            ),
            |layout, size| {
                // We could also keep the status bar at the top instead.
                layout.set_child_position(0, Rect::from_size((0, size.y - 2), (size.x, 1)));
                layout.layout(size);
            },
        )
        .full_screen(),
    );
}
