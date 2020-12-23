use gio::SettingsExt;
use glib::clone;
use gtk::{
    prelude::SwitchExtManual, BaselinePosition, Box, BoxExt, ContainerExt, Dialog, DialogExt,
    GtkWindowExt, Label, LabelExt, ListBox, ListBoxExt, ListBoxRow, Orientation, Switch, SwitchExt,
    WidgetExt,
};
use webkit2gtk::WebView;

use crate::{
    controllers::settings::{toggle_full_screen, toggle_tray_icon},
    models::{
        applications::GSettings,
        constants::{FULL_SCREEN, TRAY_ICON},
    },
};

const DEFAULT_ALIGN: f32 = 0.0;

pub fn create_settings_dialog(web_view: &WebView, custom_settings: &GSettings) {
    let dialog = Dialog::new();
    dialog.set_title("Settings");
    dialog.set_resizable(false);

    let full_screen_box = create_full_screen_switch(web_view, custom_settings);
    let tray_icon_box = create_sys_tray_icon_switch(custom_settings);

    let content_area = dialog.get_content_area();

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk::SelectionMode::None);
    let row = ListBoxRow::new();
    let row1 = ListBoxRow::new();

    row.add(&full_screen_box);
    row1.add(&tray_icon_box);

    listbox.add(&row);
    listbox.add(&row1);

    content_area.add(&listbox);
    dialog.show_all();

    dialog.set_default_size(200, 0);
}

fn create_full_screen_switch(web_view: &WebView, custom_settings: &GSettings) -> Box {
    let full_screen_box = Box::new(Orientation::Horizontal, 0);
    let full_screen_switch = Switch::new();
    let is_full_screen_enabled = custom_settings.get_boolean("full-screen");

    full_screen_switch.set_state(is_full_screen_enabled);

    let web_view_cloned = web_view.clone();
    let custom_settings_cloned = custom_settings.clone();
    full_screen_switch.connect_changed_active(move |switch| {
        toggle_full_screen(
            &web_view_cloned,
            &custom_settings_cloned,
            switch.get_state(),
        )
    });

    let full_screen_label = Label::new(Some(FULL_SCREEN));
    full_screen_label.set_xalign(DEFAULT_ALIGN);
    full_screen_box.pack_start(&full_screen_label, true, true, 0);
    full_screen_box.pack_start(&full_screen_switch, false, true, 0);

    full_screen_box
}

fn create_sys_tray_icon_switch(custom_settings: &GSettings) -> Box {
    let tray_icon_box = Box::new(Orientation::Horizontal, 0);
    tray_icon_box.set_baseline_position(BaselinePosition::Center);
    let tray_icon_switch = Switch::new();
    let is_tray_icon_enabled = custom_settings.get_boolean("tray-icon");

    tray_icon_switch.set_state(is_tray_icon_enabled);

    tray_icon_switch.connect_changed_active(clone!(@strong custom_settings =>  move |switch| {
        toggle_tray_icon(switch.get_state(), &custom_settings);
    }));

    let tray_icon_label = Label::new(Some(TRAY_ICON));
    tray_icon_label.set_xalign(DEFAULT_ALIGN);
    tray_icon_box.pack_start(&tray_icon_label, true, true, 0);
    tray_icon_box.pack_end(&tray_icon_switch, false, true, 0);

    tray_icon_box
}
