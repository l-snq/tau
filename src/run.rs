use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{ApplicationWindow, Button, Fixed};
use gio::AppInfo;
use glib::prelude::*;

pub fn RunRs() {
    // get gio shit here 
    let app = gio::AppInfo::all();
    println!("your stuff {:?}", app);
}
