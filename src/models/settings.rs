use std::ops::Deref;

use gtk::{
    gio,
  
};
use glib::thread_guard::ThreadGuard;

use crate::config;

pub struct Settings(ThreadGuard<gio::Settings>);

impl Settings {
    


}

impl Default for Settings {
    fn default() -> Self {
        Self(ThreadGuard::new(gio::Settings::new(config::APP_ID)))
    }
}

impl Deref for Settings {
    type Target = gio::Settings;

    fn deref(&self) -> &Self::Target {
        self.0.get_ref()
    }
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}
