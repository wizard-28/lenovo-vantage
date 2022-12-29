use gtk::{gdk, gio, glib};
use i18n_embed::DesktopLanguageRequester;
use relm4::{adw, gtk};

use crate::config::{APP_ID, RESOURCES_FILE};
use crate::fl;

pub fn setup() {
    adw::init().unwrap();

    setup_fluent();

    glib::set_application_name(&fl!("app-title"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    setup_css();

    gtk::Window::set_default_icon_name(APP_ID);
}

fn setup_fluent() {
    // Prepare i18n
    let localizer = crate::localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading languages for library_fluent {error}");
    }
}

fn setup_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/com/github/wizard28/LenovoVantage/style.css");
    if let Some(display) = gdk::Display::default() {
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
