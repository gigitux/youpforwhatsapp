use gio::prelude::*;
use gtkwhats::constants::*;
use gtkwhats::ui;
use std::env::args;
fn main() {
    let application = gtk::Application::new(Some(constants::APPLICATION_NAME), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        ui::ui::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
