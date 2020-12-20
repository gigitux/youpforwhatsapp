use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Settings, WindowPosition};

use crate::models::applications::CustomWebView;

use crate::models::{applications::ApplicationWithSettings, constants};

use gio::SettingsExt as GSettingsExt;

use super::{headbar::create_headbar, webview::create_webview};

fn create_application_with_settings(application: &Application) -> ApplicationWithSettings {
    let window = ApplicationWindow::new(application);

    let custom_settings = gio::Settings::new("com.gigitux.gtkwhats");

    window.set_title(constants::TITLE);
    window.set_position(WindowPosition::Center);
    window.set_default_size(
        custom_settings.get_int("persist-window-geometry-width"),
        custom_settings.get_int("persist-window-geometry-height"),
    );
    ApplicationWithSettings {
        application: window,
        custom_settings,
    }
}

pub fn build_ui(application: &Application) {
    let application_with_settings = create_application_with_settings(application);
    let application = application_with_settings.application;
    let settings = application_with_settings.custom_settings;

    let general_settings = get_settings();
    let CustomWebView { webview } = create_webview(&general_settings, &settings);
    let headbar = create_headbar(&general_settings, &settings, &webview);

    application.set_titlebar(Some(&headbar.container));

    application.add(&headbar.container);
    application.add(&webview);
    application.show_all();
}

fn get_settings() -> Settings {
    return Settings::get_default().unwrap();
}
