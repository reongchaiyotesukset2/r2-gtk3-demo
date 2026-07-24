use std::cell::OnceCell;
use gettextrs::gettext;
use anyhow::Result;


use gtk::{
    gio,
    glib::{self},
    subclass::prelude::*,
    prelude::*,
};

use crate::{
    application::Application,config,
    models::{ProvidersModel,OTPUri},
};
mod imp {


    use super::*;


    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/org/example/myapp/preferences.ui")]
    #[properties(wrapper_type = super::PreferencesWindow)]

    pub struct PreferencesWindow {
        #[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,
        #[template_child]
        pub button_clicked1 : TemplateChild<gtk::Button>,
    }



    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {

        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {

            obj.init_template();
        }

    }


    #[glib::derived_properties]
    impl ObjectImpl for PreferencesWindow {

        fn constructed(&self) {
            self.parent_constructed();
            let win = self.obj();
            win.set_icon_name(Some(config::APP_ID));
        }

    }

    impl  WidgetImpl for PreferencesWindow {}
    impl  WindowImpl for PreferencesWindow {

        fn enable_debugging(&self, toggle: bool) -> bool {
            if config::PROFILE != "Devel" {

                false
            } else {
                self.parent_enable_debugging(toggle)
            }
        }



    }
    impl  ApplicationWindowImpl for PreferencesWindow {}
    impl  BinImpl for PreferencesWindow {}
    impl ContainerImpl for PreferencesWindow {}

    impl InitableImpl for PreferencesWindow {
        fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {

            let _win = self.obj();

            Ok(())
        }
    }

}


glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::Initable,gio::ActionMap, gio::ActionGroup;
}

impl PreferencesWindow {
    pub fn new(model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
        .property("application", app)
        .property("model", model)
        .build(gio::Cancellable::NONE)
        .unwrap()
    }

}
