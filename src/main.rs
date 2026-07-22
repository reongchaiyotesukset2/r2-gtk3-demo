mod widgets;
mod application;
mod models;
mod config;
mod schema;

use crate::application::Application;

fn main() {
   gtk::init().expect("failed to init gtk");

   let res = gio::Resource::load("data/resources.gresource").expect("Could not load resources!");
   gio::resources_register(&res);

   glib::set_application_name("r2-gtk4");
   Application::run();   
}
