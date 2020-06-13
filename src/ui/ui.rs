
use gio::{ApplicationExt, Settings as GSettings, SettingsExt as GSettingsExt};
use gtk::prelude::*;
use gtk::{
    AboutDialog, Align, Application, ApplicationWindow, Box, Button, HeaderBar, IconSize,
    Label, Orientation, Settings, Switch, Window, WindowPosition, WindowType,
};

use webkit2gtk::{
    NotificationPermissionRequest, PermissionRequestExt, WebContext, WebView, WebViewExt,
};

use super::models;
use crate::constants::*;
use crate::logic::settings_logic::{set_full_screen, set_theme};

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

    let custom_settings = gio::Settings::new("com.gigitux.gtkwhats");
    let general_settings = get_settings();

    window.set_title(constants::TITLE);
    window.set_position(WindowPosition::Center);
    window.set_default_size(
        custom_settings.get_int("persist-window-geometry-width"),
        custom_settings.get_int("persist-window-geometry-height"),
    );
    let application_clone = application.clone();

    let custom_settings_clone = custom_settings.clone();
    window.connect_delete_event(move |window, _| {
        let window_geometry = window.get_size();
        custom_settings_clone
            .set_int("persist-window-geometry-width", window_geometry.0)
            .unwrap();

        custom_settings_clone
            .set_int("persist-window-geometry-height", window_geometry.1)
            .unwrap();
        application_clone.quit();
        Inhibit(false)
    });

    let vbox = Box::new(Orientation::Horizontal, 10);
    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);
    let switch_dark_mode = Switch::new();
    let switch_dark_mode_clone = switch_dark_mode.clone();

    let general_settings_clone = get_settings().clone();
    let custom_settings_clone = custom_settings.clone();
    let custom_webview = build_and_get_webview();
    custom_webview
        .webview
        .connect_load_changed(move |web_view: &WebView, _| {
            toogle_theme(
                &general_settings_clone,
                &custom_settings_clone,
                &web_view,
                &switch_dark_mode_clone,
            );

            let is_full_screen_enabled = custom_settings_clone.get_boolean("full-screen");

            set_full_screen(&web_view, &custom_settings_clone, is_full_screen_enabled);
        });
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

    let is_dark_mode_enabled = custom_settings.get_boolean("dark-theme");

    switch_dark_mode.set_active(is_dark_mode_enabled);

    let label_dark_mode = Label::new(Some(constants::HEADER_LABEL));
    vbox.add(&label_dark_mode);
    vbox.add(&switch_dark_mode);
    let settings_button =
        Button::new_from_icon_name(Some("emblem-system-symbolic"), IconSize::Menu);

    let about_button =
        Button::new_from_icon_name(Some("dialog-information-symbolic"), IconSize::Menu);
    about_button.connect_clicked(|_| open_about_page());

    let headbar = build_and_get_headbar();
    headbar.container.pack_end(&vbox);
    headbar.container.pack_end(&settings_button);
    headbar.container.pack_end(&about_button);
    window.set_titlebar(Some(&headbar.container));

    window.add(&custom_webview.webview);
    window.show_all();

    let cloned_custom_web_view = custom_webview.webview.clone();
    let custom_settings_clone = custom_settings.clone();
    switch_dark_mode.connect_changed_active(move |value| {
        toogle_theme(
            &general_settings,
            &custom_settings_clone,
            &custom_webview.webview,
            value,
        )
    });

    let custom_settings_clone = custom_settings.clone();
    settings_button
        .connect_clicked(move |_| open_dialog(&cloned_custom_web_view, &custom_settings_clone));
}

fn get_settings() -> Settings {
    return Settings::get_default().unwrap();
}

fn open_dialog(web_view: &WebView, custom_settings: &GSettings) {
    let window = Window::new(WindowType::Toplevel);
    window.set_title(constants::TITLE);
    window.set_position(WindowPosition::CenterAlways);
    window.show_all();
    let vbox = Box::new(Orientation::Horizontal, 50);
    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);

    let switch_full_screen = Switch::new();
    let is_full_screen_enabled = custom_settings.get_boolean("full-screen");

    switch_full_screen.set_active(is_full_screen_enabled);

    let web_view_cloned = web_view.clone();
    let custom_settings_cloned = custom_settings.clone();
    switch_full_screen.connect_changed_active(move |switch| {
        toggle_full_screen(switch, &web_view_cloned, &custom_settings_cloned)
    });

    let text = Label::new(Some("Full Screen"));
    vbox.add(&text);
    vbox.add(&switch_full_screen);
    window.add(&vbox);

    window.show_all();
    window.set_resizable(false);
    window.set_default_size(100, 100);
}

fn toggle_full_screen(switch: &Switch, web_view: &WebView, custom_settings: &GSettings) {
    set_full_screen(web_view, custom_settings, switch.get_state())
}

fn open_about_page() {
    let window = AboutDialog::new();
    window.set_authors(&["<Luigi Teschio> gigitux@gmail.com"]);
    window.set_version(Some("1.0"));
    window.set_website(Some("https://github.com/gigitux/gtkwhats"));
    window.show_all();
}

fn toogle_theme(
    general_settings: &Settings,
    custom_settings: &GSettings,
    web_view: &WebView,
    value: &Switch,
) {
    set_theme(
        web_view,
        general_settings,
        custom_settings,
        value.get_state(),
    )
}
