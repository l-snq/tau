use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{Application, ApplicationWindow, SearchEntry};

pub fn draw_ui(app: &Application) {

    let apps = gio::AppInfo::all();
    let mut item_to_string: String;

    let hbox = gtk::ListBox::new();
    let draw_window = ApplicationWindow::new(app);
    let search_thingy = SearchEntry::new();
    search_thingy.set_margin(400);
    let text = gtk::Label::new(None); // TODO! This needs to be figured out. Myabe not use a label
    // at all.

    let text_view = gtk::TextView::new();
    // text_view.set_wrap_mode(gtk::WrapMode::Word);
    // text_view.set_cursor_visible(false); is this even necessary ?

    fn get_application_info() {
        let apps = gio::AppInfo::all();

        for app in apps {
            app.display_name().map(|i| i.to_string());
            println!("*******This is my app description {:?}", app.description());
        }
    }

    // i have to use a list box otherwise 
    // it won't let me render multiple widgets
    // hbox.add(&search_thingy);

    get_application_info();
    hbox.add(&text_view);

    draw_window.add(&hbox);
    draw_window.show_all();
}

pub fn run_rs() {
    // get gio shit here 
    let _applications = gio::AppInfo::all();
}
