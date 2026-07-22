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
    //use glib::prelude::*;


    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/org/example/myapp/window.ui")]
    #[properties(wrapper_type = super::Window)]

    pub struct Window {
        #[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,
        #[template_child]
        pub button_clicked1 : TemplateChild<gtk::Button>,
    }



    #[glib::object_subclass]
    impl ObjectSubclass for Window {

        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);

        fn class_init(klass: &mut Self::Class) {


        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {

            obj.init_template();

        }

    }


    #[glib::derived_properties]
    impl ObjectImpl for Window {

        fn constructed(&self) {
            self.parent_constructed();
            let win = self.obj();
            win.set_icon_name(Some(config::APP_ID));
        }

    }

    impl  WidgetImpl for Window {}
    impl  WindowImpl for Window {

        fn enable_debugging(&self, toggle: bool) -> bool {
            if config::PROFILE != "Devel" {

                false
            } else {
                self.parent_enable_debugging(toggle)
            }
        }



    }
    impl  ApplicationWindowImpl for Window {}
    impl  BinImpl for Window {}
    impl ContainerImpl for Window {}

    impl InitableImpl for Window {
        fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {

            let _win = self.obj();

            Ok(())
        }
    }

}


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::Initable,gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
        .property("application", app)
        .property("model", model)
        .build(gio::Cancellable::NONE)
        .unwrap()
    }

    fn on_search_btn_click(&self, _btn: &gtk::Button) {

        println!("on_search_btn_click!!");


    }


    async fn open_qr_code(&self) -> Result<()> {
         Ok(())
    }


}
