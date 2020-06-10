use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, HeaderBar, IconSize, Label, Orientation,
    Settings, SettingsExt, Switch, Window, WindowPosition, WindowType,
};

use webkit2gtk::{
    NotificationPermissionRequest, PermissionRequestExt, WebContext, WebView, WebViewExt,
};

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

pub fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    // #WIP
    // let settings = Settings::new_with_path("com.gigitux.gtkwhats", "/com/gigitux/gtkwhats/");
    // settings.set_property("is_dark_mode", &true);

    window.set_title(constants::TITLE);
    window.set_position(WindowPosition::Center);
    let vbox = Box::new(Orientation::Horizontal, 10);
    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);

    let switch_dark_mode = Switch::new();
    let label_dark_mode = Label::new(Some(constants::HEADER_LABEL));
    vbox.add(&label_dark_mode);
    vbox.add(&switch_dark_mode);
    let settings = Button::new_from_icon_name(Some("emblem-system-symbolic"), IconSize::Menu);

    let headbar = build_and_get_headbar();
    headbar.container.pack_end(&vbox);
    headbar.container.pack_end(&settings);
    window.set_titlebar(Some(&headbar.container));

    let custom_webview = build_and_get_webview();
    custom_webview.webview.load_uri(constants::URL);
    custom_webview
        .webview
        .connect_permission_request(move |_, perm_req| {
            if perm_req.is::<NotificationPermissionRequest>() {
                perm_req.allow();
                return true;
            }

            false
        });

    window.add(&custom_webview.webview);
    window.show_all();

    let cloned_custom_web_view = custom_webview.webview.clone();
    let info_settings = get_settings();
    switch_dark_mode
        .connect_changed_active(move |_| toogle_theme(&info_settings, &custom_webview.webview));
    settings.connect_clicked(move |_| open_dialog(&cloned_custom_web_view));
}

fn toogle_theme(settings: &Settings, web_view: &WebView) {
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

fn get_settings() -> Settings {
    return Settings::get_default().unwrap();
}

fn open_dialog(web_view: &WebView) {
    let window = Window::new(WindowType::Toplevel);
    window.set_title(constants::TITLE);
    window.set_position(WindowPosition::Center);
    window.show_all();
    let vbox = Box::new(Orientation::Horizontal, 50);
    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);

    let toggle = Switch::new();
    let cloned = web_view.clone();
    toggle.connect_changed_active(move |switch| toggle_full_screen(switch, &cloned));
    let text = Label::new(Some("Full Screen"));
    vbox.add(&text);
    vbox.add(&toggle);
    window.add(&vbox);

    window.show_all();
}

fn toggle_full_screen(switch: &Switch, web_view: &WebView) {
    if switch.get_active() {
        web_view.run_javascript(
            "document.getElementsByClassName('h70RQ')[0].style.width='100vw'",
            None::<&gio::Cancellable>,
            |_result| {},
        );
    } else {
        web_view.run_javascript(
            "document.getElementsByClassName('h70RQ')[0].style.width=''",
            None::<&gio::Cancellable>,
            |_result| {},
        );
    }
}
