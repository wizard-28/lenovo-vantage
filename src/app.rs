use adw::prelude::*;
use gtk::{gio, glib};
use relm4::actions::{ActionGroupName, RelmAction, RelmActionGroup};
use relm4::{
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, SimpleComponent,
};

use crate::components::preferences::Model;
use crate::config::{APP_ID, PROFILE};
use crate::modals::about::AboutDialog;

pub(super) struct App {
    preferences: Controller<Model>,
    about_dialog: Controller<AboutDialog>,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_About Lenovo Vantage" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                gtk::Inhibit(true)
            },

            add_css_class?: if PROFILE == "Devel" {
                Some("devel")
            } else {
                None
            },


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

               append: model.preferences.widget()
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(root)
            .launch(())
            .detach();

        let model = Self {
            preferences: Model::builder().launch(()).detach(),
            about_dialog,
        };

        let widgets = view_output!();

        let actions = RelmActionGroup::<WindowActionGroup>::new();

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(());
            })
        };

        actions.add_action(&about_action);

        widgets
            .main_window
            .insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
