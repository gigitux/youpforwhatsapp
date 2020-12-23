mod controllers;
mod models;
mod views;

use gio::prelude::*;
use models::constants;
use std::env::args;
use views::ui;

fn main() {
    let application = gtk::Application::new(Some(constants::APPLICATION_NAME), Default::default())
        .expect("Initialization failed...");

    let res = gio::Resource::load("data/com.gigitux.youp.gresource").expect("Load not work");

    gio::resources_register(&res);

    application.connect_activate(|app| {
        ui::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
