use adw::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation,
    Paned, ScrolledWindow, SelectionMode, TextBuffer, TextView,
};
use std::cell::RefCell;
use std::rc::Rc;

use dnfdesk_core::{DnfClient, Package};

fn main() {
    let app = Application::builder()
        .application_id("dev.pizzuka.dnfdesk")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    adw::init().expect("Failed to initialize libadwaita");

    let dnf = Rc::new(DnfClient::new());
    let selected_package: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let title = Label::builder()
        .label("DnfDesk")
        .xalign(0.0)
        .margin_top(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let subtitle = Label::builder()
        .label("Modern DNF5 frontend for Fedora")
        .xalign(0.0)
        .margin_start(16)
        .margin_end(16)
        .build();

    let search = Entry::builder()
        .placeholder_text("Search packages, e.g. git, firefox, neovim...")
        .margin_top(12)
        .margin_start(16)
        .margin_end(16)
        .build();

    let list = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    let list_scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&list)
        .build();

    let status = Label::builder()
        .label("Type at least 2 characters to search with dnf5 repoquery.")
        .xalign(0.0)
        .margin_start(16)
        .margin_end(16)
        .margin_bottom(8)
        .build();

    let left = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .build();

    left.append(&search);
    left.append(&list_scroll);
    left.append(&status);

    let details_title = Label::builder()
        .label("Select a package")
        .xalign(0.0)
        .margin_top(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let details_summary = Label::builder()
        .label("Search for a package and select a result.")
        .wrap(true)
        .xalign(0.0)
        .margin_start(16)
        .margin_end(16)
        .build();

    let command_label = Label::builder()
        .label("Equivalent command: —")
        .wrap(true)
        .xalign(0.0)
        .margin_start(16)
        .margin_end(16)
        .build();

    let preview_button = Button::builder()
        .label("Preview install")
        .sensitive(false)
        .margin_start(16)
        .margin_end(16)
        .build();

    let preview_buffer = TextBuffer::new(None);
    preview_buffer.set_text("Transaction preview will appear here.");

    let preview_output = TextView::builder()
        .buffer(&preview_buffer)
        .editable(false)
        .monospace(true)
        .vexpand(true)
        .hexpand(true)
        .build();

    let preview_scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .margin_start(16)
        .margin_end(16)
        .margin_bottom(16)
        .child(&preview_output)
        .build();

    let right = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();

    right.append(&details_title);
    right.append(&details_summary);
    right.append(&command_label);
    right.append(&preview_button);
    right.append(&preview_scroll);

    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&left)
        .end_child(&right)
        .resize_start_child(true)
        .shrink_start_child(false)
        .shrink_end_child(false)
        .build();

    let root = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .build();

    root.append(&title);
    root.append(&subtitle);
    root.append(&paned);

    {
        let list = list.clone();
        let status = status.clone();
        let dnf = dnf.clone();
        let preview_buffer = preview_buffer.clone();

        search.connect_activate(move |entry| {
            let query = entry.text().to_string();
            clear_list(&list);
            preview_buffer.set_text("Transaction preview will appear here.");

            if query.trim().len() < 2 {
                status.set_label("Type at least 2 characters.");
                return;
            }

            status.set_label("Searching with dnf5 repoquery...");

            match dnf.search_packages(&query) {
                Ok(packages) => {
                    let count = packages.len();
                    populate_list(&list, &packages);

                    if count == 0 {
                        status.set_label("No packages found.");
                    } else {
                        status.set_label(&format!("Found {} packages. Press Enter to search again.", count));
                    }
                }
                Err(error) => {
                    status.set_label("Search failed.");
                    preview_buffer.set_text(&format!("Search error:\n\n{}", error));
                }
            }
        });
    }

    {
        let details_title = details_title.clone();
        let details_summary = details_summary.clone();
        let command_label = command_label.clone();
        let preview_button = preview_button.clone();
        let selected_package = selected_package.clone();

        list.connect_row_selected(move |_list, row| {
            if let Some(row) = row {
                if let Some(name) = row.widget_name().strip_prefix("package:") {
                    *selected_package.borrow_mut() = Some(name.to_string());
                    details_title.set_label(name);
                    details_summary.set_label("Package selected from DNF5 repoquery results.");
                    command_label.set_label(&format!("Equivalent command: dnf5 install --assumeno {}", name));
                    preview_button.set_sensitive(true);
                }
            }
        });
    }

    {
        let dnf = dnf.clone();
        let selected_package = selected_package.clone();
        let preview_buffer = preview_buffer.clone();
        let status = status.clone();

        preview_button.connect_clicked(move |_| {
            let package_name = selected_package.borrow().clone();

            let Some(package_name) = package_name else {
                preview_buffer.set_text("No package selected.");
                return;
            };

            status.set_label("Running install preview...");
            preview_buffer.set_text(&format!("Running: dnf5 install --assumeno {}\n\nPlease wait...", package_name));

            match dnf.preview_install(&package_name) {
                Ok(preview) => {
                    status.set_label("Preview complete.");
                    preview_buffer.set_text(&format!("$ {}\n\n{}", preview.command, preview.output));
                }
                Err(error) => {
                    status.set_label("Preview failed.");
                    preview_buffer.set_text(&format!("Preview error:\n\n{}", error));
                }
            }
        });
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("DnfDesk")
        .default_width(1100)
        .default_height(720)
        .child(&root)
        .build();

    window.present();
}

fn populate_list(list: &ListBox, packages: &[Package]) {
    for package in packages {
        let row = build_package_row(package);
        list.append(&row);
    }
}

fn clear_list(list: &ListBox) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
}

fn build_package_row(package: &Package) -> ListBoxRow {
    let name = Label::builder()
        .label(&package.name)
        .xalign(0.0)
        .build();

    let summary_text = if package.summary.trim().is_empty() {
        format!("Version: {}", package.version)
    } else {
        format!("{} · {}", package.version, package.summary)
    };

    let summary = Label::builder()
        .label(&summary_text)
        .wrap(true)
        .xalign(0.0)
        .build();

    let content = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(12)
        .margin_end(12)
        .build();

    content.append(&name);
    content.append(&summary);

    let row = ListBoxRow::builder()
        .child(&content)
        .build();

    row.set_widget_name(&format!("package:{}", package.name));
    row
}
