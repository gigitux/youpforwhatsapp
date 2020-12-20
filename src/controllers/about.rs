use gtk::AboutDialog;
use gtk::AboutDialogExt;
use gtk::WidgetExt;

pub fn open_about_page() {
    let window = AboutDialog::new();
    window.set_authors(&["<Luigi Teschio> gigitux@gmail.com"]);
    window.set_version(Some("1.0"));
    window.set_website(Some("https://github.com/gigitux/gtkwhats"));
    window.show_all();
}
