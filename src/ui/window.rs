use adw::prelude::*;
use gtk::{
    Align, Box, Entry, Label, ListBox, Orientation, ScrolledWindow,
};

pub fn build_main_window(app: &adw::Application) -> adw::ApplicationWindow {
    let header = adw::HeaderBar::new();

    let title = Label::builder()
        .label("DnfDesk")
        .halign(Align::Start)
        .build();

    let search = Entry::builder()
        .placeholder_text("Search packages...")
        .build();

    let list = ListBox::new();

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .child(&list)
        .build();

    let content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    content.append(&title);
    content.append(&search);
    content.append(&scroll);

    adw::ApplicationWindow::builder()
        .application(app)
        .title("DnfDesk")
        .default_width(900)
        .default_height(600)
        .content(&content)
        .build()
}