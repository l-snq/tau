use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{Application, ApplicationWindow, SearchEntry};

pub fn draw_ui(app: &Application) {

    let items = gio::AppInfo::all();
    let mut item_to_string: String;

    let hbox = gtk::ListBox::new();
    let draw_window = ApplicationWindow::new(app);
    let search_thingy = SearchEntry::new();
    search_thingy.set_margin(400);
    let list_something = gtk::ListBoxRow::new();

    let mut text_view = gtk::Label::new(None);

    for item in items {
        item_to_string = item.display_name().to_string();

        text_view.set_markup(&item_to_string)
    }

    // i have to use a list box otherwise 
    // it won't let me render multiple widgets
    hbox.add(&search_thingy);
    hbox.add(&text_view);

    draw_window.add(&hbox);
    draw_window.show_all();
}

pub fn run_rs() {
    // get gio shit here 
    let applications = gio::AppInfo::all();

}
