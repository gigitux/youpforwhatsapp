use gdk_pixbuf::{InterpType, Pixbuf};
use gtk::{AboutDialog, AboutDialogExt, WidgetExt};

pub fn open_about_page() {
    let window = AboutDialog::new();
    window.set_authors(&["<Luigi Teschio> gigitux@gmail.com"]);
    window.set_version(Some("2.0"));
    window.set_website(Some("https://luigitesch.io/youpforwhatsapp"));
    let icon = Pixbuf::from_resource("/com/gigitux/youp/logo")
        .ok()
        .and_then(|icon| icon.scale_simple(256, 256, InterpType::Hyper))
        .unwrap();

    window.set_logo(Some(&icon));
    window.show_all();
}
