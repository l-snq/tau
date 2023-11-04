use gtk4 as gtk;
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};
use gtk::{
    prelude::*, 
    Application,
    ApplicationWindow,
    Image, 
    gio,
    SearchBar,
    SearchEntry 
};
use std::collections::HashMap;
use crate::input::input_handling;
use crate::utils::AppField;
use std::process::Command; // FOR FUTURE USE

pub fn draw_ui(application: &Application) {

   
   let draw_window = ApplicationWindow::builder()
        .default_width(300)
        .default_height(300)
        .application(application)
        .title("Tau")
        .build();

   // set up layer shell
   LayerShell::init_layer_shell(&draw_window);
   LayerShell::set_layer(&draw_window, Layer::Overlay);
   LayerShell::set_keyboard_mode(&draw_window, KeyboardMode::OnDemand);
   LayerShell::auto_exclusive_zone_enable(&draw_window);

   let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

   let text = gtk::Label::new(None);     

   let mut captured_app = AppField::new();

   let mut hash = HashMap::new();

   let apps = gio::AppInfo::all(); 
   let hbox = gtk::ListBox::new();

   for app in apps {

       let icon_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);

       let app_name = app.display_name().to_string();

       let image_icon_setup = Image::builder()
            .pixel_size(50)
            .build();
        
       let title = gtk::Label::new(Some(&app_name));

       if let Some(gtk_icon_name) = app.icon() {
            image_icon_setup.set_from_gicon(&gtk_icon_name);
       }
       hash.insert(app_name.clone(), icon_box.clone()); // THIS IS EXPENSIVE, Consider alternatives to
       // using clone()

       captured_app.update_fields(app_name.clone(), app_name.clone()); // Still using clone :|

       icon_box.prepend(&title);
       icon_box.append(&image_icon_setup);
       hbox.append(&icon_box);
   }

   let search_label = gtk::Label::builder()
      .label("Type to search for an app")
      .vexpand(true)
      .halign(gtk::Align::Center)
      .valign(gtk::Align::Center)
      .build();

   // search bar setup
   let search_bar = SearchBar::builder()
      .valign(gtk::Align::Start)
      .key_capture_widget(&draw_window)
      .css_classes(["search_box"])
      .build();
   let search_button = gtk::ToggleButton::new();
   search_button.set_icon_name("system-search-symbolic");
   let search_entry = SearchEntry::new();
   
   search_entry.set_hexpand(true);

   search_button
      .bind_property("active", &search_bar, "search-mode-enabled")
      .sync_create()
      .bidirectional()
      .build();

   search_bar.set_child(Some(&search_entry));
   let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 40);
   search_box.append(&search_bar);

   //hbox.append(&text);
   hbox.append(&search_box); 

   scrolled_window.set_child(Some(&hbox));
   input_handling(&application, &draw_window);

   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
