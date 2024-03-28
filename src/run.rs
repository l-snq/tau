use gtk4 as gtk;
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};
use gtk::{
    prelude::*, 
    Application,
    ApplicationWindow,
    Image, 
    SearchBar,
    SearchEntry,
    gio,
    gdk,
    glib::{self, clone}
};
use std::collections::HashMap;
use crate::actions::on_app_activate;

pub fn draw_ui(application: &Application) {
   
   let draw_window = ApplicationWindow::builder()
        .default_width(700)
        .default_height(300)
        .application(application)
        .title("Tau")
        .build();

   // set up layer shell
   LayerShell::init_layer_shell(&draw_window);
   LayerShell::set_layer(&draw_window, Layer::Overlay);
   LayerShell::set_keyboard_mode(&draw_window, KeyboardMode::OnDemand);
   LayerShell::auto_exclusive_zone_enable(&draw_window);

   let list_box = gtk::ListBox::builder().name("who up listin they box rn").build();

   let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

   let mut hash = HashMap::new();

   let apps = gio::AppInfo::all(); 

   let bar = SearchBar::builder()
       .valign(gtk::Align::Start)
       .key_capture_widget(&draw_window)
       .build();
   let entry = SearchEntry::new();
   entry.set_hexpand(true);
   bar.set_child(Some(&entry));

   for app in apps {
       let icon_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       let app_name = app.display_name().to_string();
       icon_box.grab_focus();
       icon_box.set_widget_name(&app_name);
       let image_icon_setup = Image::builder()
            .pixel_size(50)
            .build();
       let title = gtk::Label::new(Some(&app_name));

       if let Some(gtk_icon_name) = app.icon() {
            image_icon_setup.set_from_gicon(&gtk_icon_name);
            icon_box.prepend(&title);
            icon_box.append(&image_icon_setup);
            list_box.append(&icon_box);
       } else {
           println!("the app has no icon {:?}", &app.display_name());
       }
      
       hash.insert(icon_box.clone(), app.clone()); // clone isn't really the best way to do this i
      // think?

   }

   scrolled_window.set_child(Some(&list_box));

   // continue some search entry logic here
   entry.connect_search_started(clone!(@weak list_box => move |_| {
       list_box.hide();
   }));

   // THIS IS FOR THE KEY EVENTS
   let event_controller = gtk::EventControllerKey::new();

   event_controller.connect_key_pressed(move |_, key, _, _| {
      if let Some(row) = list_box.selected_row() {
            match key {
               gdk::Key::Escape => {
                  std::process::exit(0);
               },
               gdk::Key::Return if row.has_focus() => {
                  if let Some(specific_row_child) = row.child() {
                     // get the hash map that corresponds with the widget name of the child
                     let query_child = specific_row_child;
                     let hashed_child = hash.contains_key(&query_child);
                     let captured_app = hash.get(&query_child).unwrap();
                     let launch_app = gio::AppInfo::launch(&captured_app, &[], gio::AppLaunchContext::NONE);
                  }
                  std::process::exit(0); 
               },
               _ => (),
         }
      }
      glib::Propagation::Proceed
   });

   draw_window.add_controller(event_controller);

   on_app_activate(&application);
   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
