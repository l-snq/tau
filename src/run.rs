use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{ApplicationWindow, Button, Fixed};
use glib::prelude::*;

pub fn RunRs() {
    // get gio shit here 
    let applications = gio::AppInfo::all();
    let file_name = gio::DesktopAppInfo::from_filename("firefox");

    if (file_name.is_some()) {
        println!("nice. it's firefox");
    } else {
        println!("i think something went wrong {:?}", file_name);
    }

    for app in applications { 
        app.should_show();
    }

}
