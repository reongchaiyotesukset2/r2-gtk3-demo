use std::cell::Cell;
use std::cell::OnceCell;
use gtk::{
    gio,
    glib::{self,clone},
    subclass::prelude::*,
    prelude::*,
};
use crate::{
    application::Application,config,
};
use crate::models::{Provider, ProvidersModel};

mod imp {

    use super::*;
    use glib::Properties;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/org/example/myapp/providers_dialog.ui")]
    #[properties(wrapper_type = super::ProvidersDialog)]

    pub struct ProvidersDialog {
        #[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,
        pub actions: gio::SimpleActionGroup,
        #[property(get, set, construct)]
        pub has_set_password: Cell<bool>,

    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProvidersDialog {

        const NAME: &'static str = "ProvidersDialog";
        type Type = super::ProvidersDialog;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {

            obj.init_template();

        }

    }


    #[glib::derived_properties]
    impl ObjectImpl for ProvidersDialog {

        fn constructed(&self) {
            self.parent_constructed();
            let win = self.obj();
            win.set_icon_name(Some(config::APP_ID));
        }

    }
    impl  WidgetImpl for ProvidersDialog {}
    impl  WindowImpl for ProvidersDialog {}
    impl  BinImpl  for ProvidersDialog {}
    impl  ApplicationWindowImpl for ProvidersDialog {}
    impl ContainerImpl for ProvidersDialog {}
    impl InitableImpl for ProvidersDialog {
        fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {

            let _win = self.obj();

            Ok(())
        }
    }

}

glib::wrapper! {
    pub struct ProvidersDialog(ObjectSubclass<imp::ProvidersDialog>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap;
}

impl ProvidersDialog {

    pub fn new(model: &ProvidersModel) -> Self {
        glib::Object::builder().property("model", model).build()
    }

}


