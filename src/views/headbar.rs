use gio::SettingsExt;
use glib::clone;
use gtk::{
    prelude::SwitchExtManual, Align, Box, Button, ButtonExt, ContainerExt, HeaderBar, HeaderBarExt,
    IconSize, Label, Orientation, Settings, Switch, SwitchExt, WidgetExt,
};
use webkit2gtk::WebView;

use crate::{
    controllers::{about::open_about_page, headbar::toogle_theme},
    models::{
        applications::{CustomHeader, GSettings},
        constants,
    },
};

use super::settings::create_settings_dialog;

pub fn create_headbar(
    general_settings: &Settings,
    custom_settings: &GSettings,
    web_view: &WebView,
) -> CustomHeader {
    let vbox = Box::new(Orientation::Horizontal, 10);

    vbox.set_valign(Align::Center);
    vbox.set_halign(Align::Center);
    let switch_dark_mode = Switch::new();

    switch_dark_mode.set_state(custom_settings.get_boolean("dark-theme"));

    switch_dark_mode.connect_changed_active(
        clone!(@strong general_settings, @strong custom_settings, @strong web_view => move |value| {
            toogle_theme(&general_settings, &custom_settings, &web_view, value.get_state())
        }),
    );

    let label_dark_mode = Label::new(Some(constants::HEADER_LABEL));
    vbox.add(&label_dark_mode);
    vbox.add(&switch_dark_mode);
    let settings_button = Button::from_icon_name(Some("emblem-system-symbolic"), IconSize::Menu);

    let about_button = Button::from_icon_name(Some("dialog-information-symbolic"), IconSize::Menu);
    about_button.connect_clicked(|_| open_about_page());

    let headbar = HeaderBar::new();

    headbar.pack_end(&vbox);
    headbar.pack_end(&settings_button);
    headbar.set_title(Some(constants::TITLE));

    settings_button.connect_clicked(
        clone!(@strong web_view, @strong custom_settings => move |_| create_settings_dialog(&web_view, &custom_settings)),
    );

    headbar.pack_end(&about_button);

    headbar.set_show_close_button(true);

    CustomHeader {
        container: headbar,
        switch_dark_mode,
        settings_button,
    }
}
