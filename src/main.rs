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

    application.connect_activate(|application| application.hold());

    let res = {
        match gio::Resource::load("data/com.gigitux.youp.gresource") {
            Ok(resource) => resource,
            Err(_) => gio::Resource::load("/app/share/com.gigitux.youp/com.gigitux.youp.gresource")
                .unwrap(),
        }
    };

    gio::resources_register(&res);

    application.connect_activate(|app| {
        ui::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
