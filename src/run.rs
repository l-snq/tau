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
    let text = gtk::Label::new(None); // TODO!

    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);
    //text_view.add(&text);

    for item in items {
        item_to_string = item.display_name().to_string();
        text.set_markup(&item_to_string);
        text_view.add(&text);
    }

    // i have to use a list box otherwise 
    // it won't let me render multiple widgets
    // hbox.add(&search_thingy);
    hbox.add(&text_view);
    hbox.add(&text);

    draw_window.add(&hbox);
    draw_window.show_all();
}

pub fn run_rs() {
    // get gio shit here 
    let _applications = gio::AppInfo::all();
}
