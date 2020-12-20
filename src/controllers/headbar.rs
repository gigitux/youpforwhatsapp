// aliases trait not work :(
use gio::SettingsExt as GSettingsExt;
use gtk::{
    prelude::SwitchExtManual, Align, Box, ContainerExt, GtkWindowExt, Label, Orientation, Settings,
    SettingsExt, Switch, SwitchExt, WidgetExt, Window, WindowPosition, WindowType,
};
use webkit2gtk::{WebView, WebViewExt};

use crate::models::{
    applications::GSettings,
    constants::{self, FULL_SCREEN},
};

use super::settings::toggle_full_screen;

pub fn toogle_theme(
    general_settings: &Settings,
    custom_settings: &GSettings,
    web_view: &WebView,
    is_dark_mode_enabled: bool,
) {
    set_theme(
        web_view,
        general_settings,
        custom_settings,
        is_dark_mode_enabled,
    )
}

fn set_theme(
    web_view: &WebView,
    general_settings: &Settings,
    custom_settings: &GSettings,
    is_dark_mode_enabled: bool,
) {
    if is_dark_mode_enabled {
        match custom_settings.set_boolean("dark-theme", true) {
            Ok(_) => {
                general_settings.set_property_gtk_application_prefer_dark_theme(true);
                web_view.run_javascript(
                    "document.body.classList.add('dark')",
                    None::<&gio::Cancellable>,
                    |_result| {},
                );
            }
            Err(_) => {}
        }
    } else {
        match custom_settings.set_boolean("dark-theme", false) {
            Ok(_) => {
                general_settings.set_property_gtk_application_prefer_dark_theme(false);
                web_view.run_javascript(
                    "document.body.classList.remove('dark')",
                    None::<&gio::Cancellable>,
                    |_result| {},
                );
            }
            Err(_) => {}
        }
    }
}

pub fn open_dialog(web_view: &WebView, custom_settings: &GSettings) {
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

    let text = Label::new(Some(FULL_SCREEN));
    vbox.add(&text);
    vbox.add(&switch_full_screen);
    window.add(&vbox);

    window.show_all();
    window.set_resizable(false);
    window.set_default_size(100, 100);
}
