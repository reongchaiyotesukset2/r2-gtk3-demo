use gtk::{

    glib::{self},
    subclass::prelude::*,
};
use diesel::prelude::*;
use crate::{
    models::{database},
    schema::providers,
};
pub struct ProviderPatch {
    pub name: String,
    pub website: Option<String>,
    pub help_url: Option<String>,
    pub image_uri: Option<String>,
    pub period: i32,
    pub digits: i32,
    pub default_counter: i32,
    pub algorithm: String,
    pub method: String,
    pub is_backup_restore: bool,
}

#[derive(Insertable)]
#[diesel(table_name = providers)]
struct NewProvider {
    pub name: String,
    pub website: Option<String>,
    pub help_url: Option<String>,
    pub image_uri: Option<String>,
    pub period: i32,
    pub digits: i32,
    pub default_counter: i32,
    pub algorithm: String,
    pub method: String,
}
mod imp {

    use super::*;

     #[derive(Default)]
    pub struct Provider {
       
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "Provider";
        type Type = super::Provider;
    }

    impl ObjectImpl for Provider {
        
    }

    impl Provider {

    }
}

glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>);
}

impl Provider {
    pub fn load()-> Result<(), Box<dyn std::error::Error>> {
        println!("load on provider on provider.rs");

        use crate::schema::providers::dsl::*;
        let db = database::connection();
        let mut _conn = db.get()?;
        Ok(())
    }
}
