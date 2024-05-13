use gtk4 as gtk;
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};
use gtk::{
    gdk, gio, glib::{self, clone}, prelude::*, Application, ApplicationWindow, 
    IconLookupFlags, IconTheme, Image, SearchBar, SearchEntry, TextDirection
};
use std::{ascii::AsciiExt, cell::RefCell, collections::HashMap, rc::Rc};
use crate::{actions::on_app_activate, 
    utils::{
        hash_match_and_launch_app, 
        prepend_box_if_matches,
    }
};

pub fn draw_ui(application: &Application) {
   
   let draw_window = ApplicationWindow::builder()
        .default_width(300)
        .default_height(300)
        .application(application)
        .title("Tau")
        .build();

   // set up layer shell
   LayerShell::init_layer_shell(&draw_window);
   LayerShell::set_layer(&draw_window, Layer::Top);
   LayerShell::set_keyboard_mode(&draw_window, KeyboardMode::OnDemand);
   LayerShell::auto_exclusive_zone_enable(&draw_window);

   let list_box = gtk::ListBox::builder()
           .name("who up listin they box rn")
           .build();

   let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

   let mut hash = HashMap::new();
   let mut app_name_hash = HashMap::new();

   let apps = gio::AppInfo::all(); 

   let bar = SearchBar::builder()
       .valign(gtk::Align::Start)
       .show_close_button(true)
       .build();
   let entry = SearchEntry::new();
   entry.set_hexpand(true);
   entry.set_widget_name("entry");
   bar.connect_entry(&entry);
   bar.set_key_capture_widget(Some(&entry));

   let icon_theme = IconTheme::default();

   let parent_box = gtk::Box::new(gtk::Orientation::Vertical, 20);
   parent_box.append(&list_box);

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
            let some_icon_theme = Some(icon_theme.lookup_by_gicon(
                    &gtk_icon_name, 
                    16, 
                    16, 
                    TextDirection::None, 
                    IconLookupFlags::FORCE_REGULAR));

            if some_icon_theme.is_some() {
                image_icon_setup.set_from_gicon(&gtk_icon_name);
                /* icon_box.prepend(&title);
                icon_box.append(&image_icon_setup);
                list_box.append(&icon_box); */
            } 
           }        
       let str_app_name = app.display_name().to_string();

       app_name_hash.insert(str_app_name.clone(), app.clone());
       hash.insert(icon_box.clone(), app.clone()); 
    }
   parent_box.prepend(&entry);
   
    // continue some search entry logic here
   entry.connect_search_changed(clone!(@weak list_box => move |entry| {
       println!("{}", entry.text());
       let relevant_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       relevant_box.set_focusable(true);
       let user_text = entry.text().to_string();
       let apps = gio::AppInfo::all();
       for app in apps {
           let app_name = app.display_name().to_string();
           prepend_box_if_matches(
               user_text.clone(), 
               app_name, 
               &relevant_box, 
               &list_box
           );
       }
   })); 

   entry.connect_activate(clone!(@weak list_box => move |entry| {
       println!("your value: {}", entry.text());
       let user_string = entry.text().to_string();
       let apps = gio::AppInfo::all();
       for app in apps {
           let app_name = app.display_name().to_string();
           if app_name.eq_ignore_ascii_case(&user_string) {
               let launch_command = gio::AppInfo::launch(
                   &app, 
                   &[], 
                   gio::AppLaunchContext::NONE);
               std::process::exit(0);
           };

       }
   }));
   entry.connect_stop_search(clone!(@weak list_box => move |entry|{
       //list_box.remove(&relevant_box);

   }));
   scrolled_window.set_child(Some(&parent_box));

   // THIS IS FOR THE KEY EVENTS 
   // Do we even fucking need this??? 
   // nope c:

   // DELETE THIS FUCKING CODE IT DOESN'T WORK
   let event_controller = gtk::EventControllerKey::new();

   event_controller.connect_key_pressed(move |_, key, _, _| {
      if let Some(row) = list_box.selected_row() {
            if row.is_focusable() {
                row.grab_focus();
            }
            match key {
               gdk::Key::Return if row.has_focus() => {
                  // specify if the row is the search bar, 
                  // and if it is, set the search mode on?
                  if let Some(specific_row_child) = row.child() {
                     hash_match_and_launch_app(specific_row_child, &hash);
                  } else {
                      println!("uh oh, theres no match from your query");
                  }
               },
               _ => (),
         }
      }

      match key {
          gdk::Key::Escape => std::process::exit(0),
          _ => ()
      }

      glib::Propagation::Proceed
   });

   //draw_window.add_controller(event_controller);

   on_app_activate(&application);
   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
