use gtk::{prelude::*, IconLookupFlags};
use gtk::{Application, ApplicationWindow, SearchBar, Image, IconTheme };

use crate::input::{self, compare_inputs};

pub fn draw_ui(app: &Application) {

    let apps = gio::AppInfo::all(); 
    let hbox = gtk::ListBox::new();

    let draw_window = ApplicationWindow::new(app);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();
    let search_bar = SearchBar::new();  //TODO! Need to add input box. Styling is broken.
    search_bar.set_margin(10);

    // handle search event. Searchbar.handle_event(); takes 1 arg

    let text = gtk::Label::new(None);     

    for app in apps {
       let icon_theme = IconTheme::default().unwrap();
       let icon_with_name = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       let name = app.display_name();

       let icon = app.icon(); // TODO! Fetching the actual icon from gio is out of the scope of
        // gio. You need to use something else. This isn't even being used anyways
       let image_container = Image::from_icon_name(Some(&name), gtk::IconSize::Menu);

       let title = gtk::Label::new(Some(&name));

       icon_with_name.pack_start(&title, false, false, 0);
       icon_with_name.pack_end(&image_container, false, true, 0);
       hbox.add(&icon_with_name);
    }

    hbox.add(&text);
    hbox.add(&search_bar); 

    scrolled_window.add(&hbox);

    draw_window.add(&scrolled_window);
    draw_window.set_size_request(100, 400);
    draw_window.set_keep_above(true);
    draw_window.show_all();
}
