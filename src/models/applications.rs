use gio::Settings as GioSettings;
use gtk::{ApplicationWindow, Button, HeaderBar, Switch};
use webkit2gtk::WebView;

pub struct CustomWebView {
    pub webview: WebView,
}

pub struct CustomHeader {
    pub container: HeaderBar,
    pub switch_dark_mode: Switch,
    pub settings_button: Button,
}

pub struct ApplicationWithSettings {
    pub application: ApplicationWindow,
    pub custom_settings: GSettings,
}

pub type GSettings = GioSettings;
