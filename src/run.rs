use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{Application, ApplicationWindow, SearchEntry, Button, Fixed};

pub fn draw_ui(app: &Application) {
    let draw_window = ApplicationWindow::new(app);
    let search_thingy = SearchEntry::new();
    search_thingy.set_margin(20);

    draw_window.add(&search_thingy);
    draw_window.show_all();
    
}

pub fn run_rs() {
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
