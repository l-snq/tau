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
    glib
};
use std::collections::HashMap;
use crate::utils::AppField;
use crate::actions::on_app_activate;
use std::process::Command; // FOR FUTURE USE

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
   //println!("******{:?}", list_box.select_row(Some(0)));
   if list_box.has_focus() {
      println!("list_box has focus")
   };

   let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

   let text = gtk::Label::new(None);     

   let mut captured_app = AppField::new();

   let mut hash = HashMap::new();

   let apps = gio::AppInfo::all(); 

   // refactor this whole fucking thing dude. This is a mess. Coordinate the hashmap with the UI
   // properly.
   for app in apps {

       let icon_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       let app_name = app.display_name().to_string();
       icon_box.grab_focus();
       icon_box.set_widget_name(&app_name);
       let image_icon_setup = Image::builder()
            .pixel_size(50)
            .build();
       let title = gtk::Label::new(Some(&app_name));

       //let list_row = gtk::ListBoxRow::builder().name(&app_name).build();

       if let Some(gtk_icon_name) = app.icon() {
            image_icon_setup.set_from_gicon(&gtk_icon_name);
       }
       hash.insert(app_name.clone(), icon_box.clone()); // THIS IS EXPENSIVE, Consider alternatives to
       // using clone()

       captured_app.update_fields(app_name.clone(), app_name.clone()); // Still using clone :|

       //println!("{:?}", icon_box.widget_name());
       icon_box.prepend(&title);
       icon_box.append(&image_icon_setup);
       list_box.append(&icon_box);
       
   }

   let search_label = gtk::Label::builder()
      .label("Type to search for an app")
      .vexpand(true)
      .halign(gtk::Align::Center)
      .valign(gtk::Align::Center)
      .build();

   scrolled_window.set_child(Some(&list_box));

   // THIS IS FOR THE KEY EVENTS
   let event_controller = gtk::EventControllerKey::new();

   event_controller.connect_key_pressed(move |_, key, _, _| {
      if let Some(row) = list_box.selected_row() {
         match key {
            gdk::Key::Escape => {
               std::process::exit(0);
            },
            gdk::Key::Return if row.has_focus() => {
               println!("this row {:?} has focus", &row);
               // do your magic here  
            },
            _ => (),
      }
   }
      glib::Propagation::Proceed
   });

   draw_window.add_controller(event_controller);
   //input_handling(&application, &draw_window);

   on_app_activate(&application);
   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
