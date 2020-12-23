use gdk::Screen;
use gio::SettingsExt;
use glib::{clone, Cast, UserDirectory};
use gtk::{show_uri, Settings};
use webkit2gtk::{
    NavigationPolicyDecision, NavigationPolicyDecisionExt, PolicyDecisionType, SecurityOrigin,
    URIRequestExt, WebContext, WebContextExt, WebView, WebViewExt,
};

use crate::{
    controllers::{headbar::toogle_theme, settings::toggle_full_screen},
    models::{
        applications::{CustomWebView, GSettings},
        constants::{self, URL},
    },
};

pub fn create_webview(general_settings: &Settings, custom_settings: &GSettings) -> CustomWebView {
    let context = WebContext::get_default().unwrap();

    context.set_spell_checking_enabled(true);
    let origin = SecurityOrigin::new_for_uri(URL);

    context.initialize_notification_permissions(&[&origin], &[]);

    let webview = WebView::with_context(&context);
    let is_full_screen_enabled = custom_settings.get_boolean("full-screen");
    let is_dark_mode_enabled = custom_settings.get_boolean("dark-theme");

    webview.load_uri(constants::URL);

    let download_folder = glib::get_user_special_dir(UserDirectory::Downloads)
        .map(|directory| directory.as_path().display().to_string());
    webview.download_uri(&download_folder.unwrap());
    webview.load_uri(constants::URL);

    webview.connect_decide_policy(
        move |_, policy, policy_decison_type| match policy_decison_type {
            PolicyDecisionType::NewWindowAction => {
                let policy = policy
                    .clone()
                    .downcast::<NavigationPolicyDecision>()
                    .expect("Unable to cast policy");
                let url = policy.get_request().unwrap().get_uri().unwrap();

                match show_uri(Screen::get_default().as_ref(), url.as_str(), 0) {
                    Ok(action) => action,
                    Err(_) => panic!("Error to open this url: {:?}", url),
                }
                return true;
            }

            PolicyDecisionType::NavigationAction => true,
            PolicyDecisionType::Response => true,
            PolicyDecisionType::__Unknown(_) => true,
            _ => true,
        },
    );

    webview.connect_load_changed(
        clone!(@strong custom_settings, @strong is_full_screen_enabled => move |webview, _| {
            toggle_full_screen(webview, &custom_settings, is_full_screen_enabled)
        }),
    );

    toogle_theme(
        &general_settings,
        &custom_settings,
        &webview,
        is_dark_mode_enabled,
    );

    CustomWebView { webview: webview }
}
