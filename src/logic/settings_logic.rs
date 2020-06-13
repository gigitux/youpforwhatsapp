use gio::{Settings as GSettings, SettingsExt as GSettingsExt};
use gtk::{Settings, SettingsExt};

use webkit2gtk::{WebView, WebViewExt};

pub fn set_full_screen(
    web_view: &WebView,
    custom_settings: &GSettings,
    is_full_screen_enabled: bool,
) {
    if is_full_screen_enabled {
        match custom_settings.set_boolean("full-screen", true) {
            Ok(_) => {
                web_view.run_javascript(
                    "
                    var head = document.head || document.getElementsByTagName('head')[0];
                    var style = document.createElement('style');            
                    style.type = 'text/css';
                    style.id = 'gtkwhats';
                    var css = '.h70RQ {width: 100vw !important}';
                    // Append the css rules to the style node
                    style.appendChild(document.createTextNode(css));
                    
                    // Append the style node to the head of the page
                    head.appendChild(style); 
                    ",
                    None::<&gio::Cancellable>,
                    |_result| {},
                );
            }
            Err(_) => {}
        }
    } else {
        match custom_settings.set_boolean("full-screen", false) {
            Ok(_) => {
                web_view.run_javascript(
                    r##"
                    var elms = document.querySelectorAll('[id="gtkwhats"]');
                    elms.forEach((e) => e.remove());
                   "##,
                    None::<&gio::Cancellable>,
                    |_result| {},
                );
            }
            Err(_) => {}
        }
    }
}

pub fn set_theme(
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
