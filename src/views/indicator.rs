use glib::clone;
use gtk::{
    get_current_event_time, ApplicationWindow, GtkMenuItemExt, GtkWindowExt, MenuShellExt,
    WidgetExt,
};
use libappindicator::{AppIndicator, AppIndicatorStatus};

pub fn create_indicator(application: &ApplicationWindow) -> AppIndicator {
    let mut app_indicator = AppIndicator::new("youp", "user-invisible-symbolic");

    app_indicator.set_icon("user-available-symbolic");
    app_indicator.set_status(AppIndicatorStatus::Active);
    let mut m = gtk::Menu::new();
    let timestamp = get_current_event_time();

    let open_item = gtk::MenuItem::with_label("Open app");
    open_item.connect_activate(clone!(@strong application, @strong timestamp => move |_| {
        application.present_with_time(timestamp);
    }));

    let quit_item = gtk::MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| {
        gtk::main_quit();
    });

    m.append(&open_item);
    m.append(&quit_item);

    app_indicator.set_menu(&mut m);

    m.show_all();

    app_indicator
}
