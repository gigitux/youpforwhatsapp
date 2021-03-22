use std::cell::RefCell;

use glib::clone;
use gtk::{prelude::*, StateFlags};
use gtk::{Application, ApplicationWindow, Settings, WindowPosition};
use webkit2gtk::{NotificationExt, WebViewExt};

use crate::{controllers::indicator::set_status_indicator, models::applications::CustomWebView};

use crate::models::{applications::ApplicationWithSettings, constants};

use gio::SettingsExt as GSettingsExt;

use super::{headbar::create_headbar, indicator::create_indicator, webview::create_webview};

fn create_application_with_settings(application: &Application) -> ApplicationWithSettings {
    let window = ApplicationWindow::new(application);

    let custom_settings = gio::Settings::new("com.gigitux.youp");

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

    let indicator = create_indicator(&application);

    let indicator = RefCell::new(indicator);

    webview.connect_property_title_notify(move |webview| set_status_indicator(webview, &indicator));
    webview.connect_show_notification(clone!(@strong application => move |_, notification| {

        notification.connect_clicked(clone!(@strong application => move |_| {
            application.present();
        }));

        false
    }));

    application.add(&headbar.container);
    application.add(&webview);
    application.connect_delete_event(
        clone!(@strong settings => move |application, _| match settings.get_boolean("tray-icon") {
            true => {
                application.set_state_flags(StateFlags::ACTIVE, true);
                application.hide();
                Inhibit(true)
            }
            false => Inhibit(false),
        }),
    );

    application.show_all();
}

fn get_settings() -> Settings {
    Settings::get_default().unwrap()
}
