mod app;
mod error;
mod dnf;
mod ui;

use app::build_app;

fn main() {
    let app = build_app();
    app.run();
}