use webkit2gtk::{WebView, WebViewExt};

pub fn set_status_indicator(
    web_view: &WebView,
    app_indicator: &std::cell::RefCell<libappindicator::AppIndicator>,
) {
    let title_tab = web_view
        .get_title()
        .map(move |title| title.to_string())
        .unwrap_or_else(String::new);

    match title_tab.chars().any(|char| char.is_numeric()) {
        true => app_indicator
            .borrow_mut()
            .set_icon("user-status-pending-symbolic"),
        false => app_indicator
            .borrow_mut()
            .set_icon("user-available-symbolic"),
    }
}
