use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use super::{Provider};

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Default)]
    pub struct ProvidersModel(pub RefCell<Vec<Provider>>, pub Cell<bool>);

    #[glib::object_subclass]
    impl ObjectSubclass for ProvidersModel {
        const NAME: &'static str = "ProvidersModel";
        type Type = super::ProvidersModel;
        type Interfaces = (gio::ListModel,);
    }
    impl ObjectImpl for ProvidersModel {}
    impl ListModelImpl for ProvidersModel {
        fn item_type(&self) -> glib::Type {
            Provider::static_type()
        }
        fn n_items(&self) -> u32 {
            self.0.borrow().len() as u32
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            self.0
                .borrow()
                .get(position as usize)
                .map(|o| o.clone().upcast::<glib::Object>())
        }
    }
}

glib::wrapper! {
    pub struct ProvidersModel(ObjectSubclass<imp::ProvidersModel>)
        @implements gio::ListModel;
}

impl ProvidersModel {
    pub fn load(&self) {
         let providers = Provider::load();
         println!("load  ProvidersModel on imp ProvidersModel witt file providers.rs");
    }
}

impl Default for ProvidersModel {
    fn default() -> Self {
        glib::Object::new()
    }
}
