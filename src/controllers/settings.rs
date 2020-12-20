use gio::SettingsExt;
use gtk::{Switch, SwitchExt};
use webkit2gtk::{WebView, WebViewExt};

use crate::models::applications::GSettings;

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
                    var css = '._36Q2N {width: 100vw !important}';
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

pub fn toggle_full_screen(switch: &Switch, web_view: &WebView, custom_settings: &GSettings) {
    set_full_screen(web_view, custom_settings, switch.get_state())
}
