use gtk::{
    gio,
    glib::{self,clone},
    subclass::prelude::*,
    prelude::*,
};

use crate::{
    models::{ProvidersModel},
};

mod imp {
    use crate::config;
    use std::{
        cell::{Cell, OnceCell},
    };
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/org/example/myapp/preferences.ui")]
    #[properties(wrapper_type = super::PreferencesWindow)]

    pub struct PreferencesWindow {
        #[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,
        #[property(get, set, construct)]
        pub has_set_password: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {

        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = gtk::ApplicationWindow;
        //type Interfaces = (gio::Initable,);

        fn class_init(klass: &mut Self::Class) {



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
    impl  WindowImpl for PreferencesWindow {}
    impl  BinImpl  for PreferencesWindow {}
    impl  ApplicationWindowImpl for PreferencesWindow {}
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
    @implements gio::Initable, gio::ActionMap, gio::ActionGroup;
}

impl PreferencesWindow {

}
impl Default for PreferencesWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}

