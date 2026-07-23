
use gtk::{
    gio,
    glib::{self},
    subclass::prelude::*,
    prelude::*,
};
use crate::{
    widgets::{Window,PreferencesWindow},
    models::{ProvidersModel},
};
use crate::config;
use gtk::subclass::prelude::DerivedObjectProperties;
use std::cell::{Cell,RefCell};
 
  
mod imp {
  
  use super::*;
  #[derive(Default,glib::Properties)]
  #[properties(wrapper_type = super::Application)]
  
  pub struct Application {
       pub window: RefCell<Option<glib::WeakRef<Window>>>,
        pub model: ProvidersModel,
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
   
    }
  
  
  #[glib::object_subclass]
   impl ObjectSubclass for Application {
   
         const NAME: &'static str = "Application";
         type Type = super::Application;
         type ParentType = gtk::Application;
         type Interfaces = ();
   }
     #[glib::derived_properties]
   impl ObjectImpl for Application { }
   
   impl ApplicationImpl for Application {
       
            fn startup(&self) {
            
             println!("startup");
             
                self.parent_startup();

                let app = self.obj();


                let button1_action = gio::ActionEntry::builder("button1")
                .activate(|app: &Self::Type, _, _| {

                  let window = app.active_window();
                  let preferences = PreferencesWindow::default();
                  preferences.present();

                }).build();


                    let quit_action = gio::ActionEntry::builder("quit")
                  .activate(|app: &Self::Type, _, _| {
                    app.quit()  
                  })
                  .build();

                    app.add_action_entries([
                        quit_action,
                        button1_action,
                    ]);
            }
            
            fn activate(&self) {
                println!("activate");
                let app = self.obj();
                let window = Window::new(&self.model, &app);
                window.present();
                self.window.replace(Some(window.downgrade()));

            }
            fn open(&self, _files: &[gio::File], _hint: &str) 
            {

               self.activate();
             }
            
          
   }
      impl GtkApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
       @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    
    pub fn run() -> glib::ExitCode  {
     println!("run!!!!!!!!!!!!");
    
     tracing::info!("Authenticator ({})", config::APP_ID);
     tracing::info!("Version: {} ({})", config::VERSION, config::PROFILE);


        let app = glib::Object::builder::<Application>()
            .property("application-id", config::APP_ID)
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build();
            app.imp().model.load();
        app.run()
      
    }

  pub fn active_window(&self) -> Window {
        self.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }
}
