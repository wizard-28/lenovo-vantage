#![allow(clippy::used_underscore_binding)]

mod app;
mod components;
mod config;
mod localize;
mod modals;
mod setup;

use app::App;
use gtk::prelude::ApplicationExt;
use relm4::{gtk, main_application, RelmApp};
use setup::setup;

relm4::new_action_group!(AppActionGroup, "app");

fn main() {
    // Enable logging
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    setup();

    let app = main_application();
    app.set_resource_base_path(Some("/com/github/wizard28/LenovoVantage/"));

    let app = RelmApp::with_app(app);

    app.run::<App>(());
}
