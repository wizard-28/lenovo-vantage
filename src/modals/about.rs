use gtk::prelude::GtkWindowExt;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::VERSION;
use crate::fl;

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = ();
    type Output = ();
    type Root = gtk::AboutDialog;
    type Widgets = gtk::AboutDialog;

    fn init_root() -> Self::Root {
        gtk::AboutDialog::builder()
            .logo_icon_name("com.github.wizard28.LenovoVantage")
            .license_type(gtk::License::Gpl30)
            .comments(&fl!("app-comments"))
            .website("https://github.com/wizard-28/lenovo-vantage/")
            .version(VERSION)
            // NOTE: Doesn't work with fluent. :(
            // .translator_credits("translator-credits")
            .modal(true)
            .authors(vec!["wizard-28 <wiz28@protonmail.com>".into()])
            .artists(vec!["wizard-28 <wiz28@protonmail.com>".into()])
            .copyright("Copyright © 2022-present Sourajyoti Basak <wiz28@protonmail.com>")
            .build()
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
