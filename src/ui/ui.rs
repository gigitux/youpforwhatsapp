use gtk::prelude::*;
use gtk::{Button, HeaderBar, SettingsExt};
use webkit2gtk::{WebContext, WebView, WebViewExt};

use super::models;
use crate::constants::*;

fn build_and_get_headbar() -> models::CustomHeader {
    let container = HeaderBar::new();
    container.set_show_close_button(true);

    models::CustomHeader { container }
}

fn build_and_get_webview() -> models::CustomWebView {
    let context = WebContext::get_default().unwrap();

    models::CustomWebView {
        webview: WebView::new_with_context(&context),
    }
}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(constants::TITLE);
    window.set_position(gtk::WindowPosition::Center);
    let dark_mode = Button::new_with_label(constants::HEADER_LABEL);
    let headbar = build_and_get_headbar();
    headbar.container.pack_end(&dark_mode);
    window.set_titlebar(Some(&headbar.container));

    let custom_webview = build_and_get_webview();
    custom_webview.webview.load_uri(constants::URL);
    window.add(&custom_webview.webview);
    window.show_all();

    let settings = get_settings();
    dark_mode.connect_clicked(move |_| toogle_theme(&settings, &custom_webview.webview));
}

fn toogle_theme(settings: &gtk::Settings, web_view: &WebView) {
    if settings.get_property_gtk_application_prefer_dark_theme() {
        settings.set_property_gtk_application_prefer_dark_theme(false);
        web_view.run_javascript(
            "document.body.classList.remove('dark')",
            None::<&gio::Cancellable>,
            |_result| {},
        );
    } else {
        settings.set_property_gtk_application_prefer_dark_theme(true);
        web_view.run_javascript(
            "document.body.classList.add('dark')",
            None::<&gio::Cancellable>,
            |_result| {},
        );
    }
}

fn get_settings() -> gtk::Settings {
    return gtk::Settings::get_default().unwrap();
}
