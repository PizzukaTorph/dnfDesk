use adw::prelude::*;
use gtk::gio;

use crate::ui::window::build_main_window;

pub fn build_app() -> adw::Application {
    let app = adw::Application::builder()
        .application_id("com.dnfdesk.app")
        .flags(gio::ApplicationFlags::empty())
        .build();

    app.connect_activate(|app| {
        let window = build_main_window(app);
        window.present();
    });

    app
}