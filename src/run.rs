use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{Application, ApplicationWindow, SearchEntry};

pub fn draw_ui(app: &Application) {

    let hbox = gtk::ListBox::new();
    let data: Vec<u8>;
    let draw_window = ApplicationWindow::new(app);
    let search_thingy = SearchEntry::new();
    search_thingy.set_margin(400);
    let list_something = gtk::ListBoxRow::new();

    hbox.add(&search_thingy);
    // i have to use a list box otherwise 
    // it won't let me render multiple widgets
    hbox.add(&list_something);

    //draw_window.add(&search_thingy);
    draw_window.add(&hbox);

    draw_window.show_all();
}

pub fn run_rs() {
    // get gio shit here 
    let applications = gio::AppInfo::all();
    let file_name = gio::DesktopAppInfo::from_filename("firefox");

    if file_name.is_some() {
        println!("nice. it's firefox");
    } else {
        println!("i think something went wrong {:?}", file_name);
    }

    for app in applications { 
        app.should_show();
        println!("{}", app.display_name().to_string()); 
        // this is how you fetch all app display names
    }

}
