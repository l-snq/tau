use gtk4 as gtk;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use gtk::{
    prelude::*, 
    IconLookupFlags, 
    Application,
    ApplicationWindow,
    Image, 
    IconTheme,
    gio,
    glib,
};
use crate::input::{self, compare_inputs};

pub fn draw_ui(app: &Application) {

    let apps = gio::AppInfo::all(); 
    let hbox = gtk::ListBox::new();

    let draw_window = ApplicationWindow::builder()
        .default_width(300)
        .default_height(300)
        .application(app)
        .title("Tau")
        .build();

    // set up layer shell
    LayerShell::init_layer_shell(&draw_window);
    LayerShell::set_layer(&draw_window, Layer::Overlay);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

    let text = gtk::Label::new(None);     

    for app in apps {
       let icon_theme = gtk::IconTheme::default();
       let icon_with_name = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       let name = app.display_name();
       let icon = app.icon(); // TODO! Fetching the actual icon from gio is out of the scope of
        // gio. You need to use something else. This isn't even being used anyways
       let image_container = Image::from_icon_name(&name);

       let title = gtk::Label::new(Some(&name));

       icon_with_name.prepend(&title);
       icon_with_name.append(&image_container);
       hbox.append(&icon_with_name);
    }

    hbox.append(&text);
    // hbox.prepend(&search_bar); 

    scrolled_window.set_child(Some(&hbox));

    draw_window.set_child(Some(&scrolled_window));
    draw_window.set_size_request(100, 400);
    draw_window.show();
}
