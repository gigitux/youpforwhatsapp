// aliases trait not work :(
use gio::SettingsExt as GSettingsExt;
use gtk::{Settings, SettingsExt};
use webkit2gtk::{WebView, WebViewExt};

use crate::models::applications::GSettings;

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
        if custom_settings.set_boolean("dark-theme", true).is_ok() {
            general_settings.set_property_gtk_application_prefer_dark_theme(true);
            web_view.run_javascript(
                "document.body.classList.add('dark')",
                None::<&gio::Cancellable>,
                |_result| {},
            );
        }
    } else if custom_settings.set_boolean("dark-theme", false).is_ok() {
        general_settings.set_property_gtk_application_prefer_dark_theme(false);
        web_view.run_javascript(
            "document.body.classList.remove('dark')",
            None::<&gio::Cancellable>,
            |_result| {},
        );
    }
}
