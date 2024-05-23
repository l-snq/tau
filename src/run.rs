use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};
use gtk::{
    gio, glib::{self, clone}, prelude::*, Application, ApplicationWindow, IconLookupFlags, IconTheme, Image, SearchBar, SearchEntry, TextDirection
};
use std::collections::{HashMap, HashSet};
use crate::{actions::on_app_activate, utils::AppField};

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

   let list_box = gtk::ListBox::new();
   let scrolled_window = gtk::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

   let mut hash = HashMap::new();

   let apps = gio::AppInfo::all(); 

   let bar = SearchBar::builder()
       .valign(gtk::Align::Start)
       .show_close_button(true)
       .build();
   let entry = SearchEntry::new();
   entry.set_hexpand(true);
   entry.set_widget_name("entry");
   entry.set_placeholder_text(Some("Start typing something..."));
   entry.set_key_capture_widget(Some(&list_box));

   bar.connect_entry(&entry);
   bar.set_search_mode(true);
   bar.set_key_capture_widget(Some(&entry));

   let icon_theme = IconTheme::default();

   let parent_box = gtk::Box::new(gtk::Orientation::Vertical, 20);
   parent_box.append(&list_box);

   for app in apps {
       let app_name = app.display_name().to_string();

       let icon_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       icon_box.grab_focus();
       icon_box.set_widget_name(&app_name);
       let image_icon_setup = Image::builder()
            .pixel_size(50)
            .build();
       let title = gtk::Label::new(Some(&app_name));

       // rendering out all the results in appInfo
       let lbr = gtk::ListBoxRow::new();
       let lbr_label = gtk::Label::new(Some(&app_name));
       lbr.set_child(Some(&lbr_label));
       list_box.append(&lbr);

       if let Some(gtk_icon_name) = app.icon() {
            let some_icon_theme = Some(icon_theme.lookup_by_gicon(
                    &gtk_icon_name, 
                    16, 
                    16, 
                    TextDirection::None, 
                    IconLookupFlags::FORCE_REGULAR)
            );

            if some_icon_theme.is_some() {
                image_icon_setup.set_from_gicon(&gtk_icon_name);
                // move this into entry.search_changed ?
            } 
       }        

       hash.insert(icon_box.clone(), app.clone()); 

       let app_id = app.id().unwrap().to_string();
       let contained_app = AppField {
           app_name: app_name.clone(),
           app_info: Some(app.clone()),
           id: Some(app_id), 
       };

       contained_app.update_fields(); 
    }
   parent_box.prepend(&entry);
   
    // continue some search entry logic here
   entry.connect_search_changed(clone!(@weak list_box => move |entry| {
       let relevant_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
       relevant_box.set_focusable(true);
       let user_text = entry
           .text()
           .to_string()
           .to_lowercase();
       let apps = gio::AppInfo::all();
       for app in apps {
           let hash_set: HashSet<&gtk::Label> = HashSet::new();
           let app_name = app
               .display_name()
               .to_string()
               .to_lowercase();
           let app_title = app
               .display_name()
               .to_string(); // adding this so the titles when rendered aren't all in lowercase
           let app_id = app
               .id()
               .unwrap()
               .to_string();
           let matcher = SkimMatcherV2::default();
           let contained_app = AppField {
               app_name,
               app_info: Some(app),
               id: Some(app_id),
           }; // i know this is ugly as shit. deal with it later

           if matcher.fuzzy_match(
               contained_app.app_name.as_str(), 
               user_text.clone().as_str()
           ).is_some() {
               list_box.remove_all();
           }
       }
   })); 

   entry.connect_activate(clone!(@weak list_box => move |entry| {
       println!("your value: {}", entry.text());
       let user_string = entry.text().to_string();
       let apps = gio::AppInfo::all();
       for app in apps {
           let app_name = app.display_name().to_string();
           let app_id = app.id().unwrap().to_string();
           let app_struct = AppField {
               app_name,
               app_info: Some(app.clone()),
               id: Some(app_id),
           };
           let matcher = SkimMatcherV2::default();
           if matcher.fuzzy_match(app_struct.app_name.as_str(), user_string.clone().as_str()).is_some() {
               let launch_command = gio::AppInfo::launch(
                   &app, 
                   &[], 
                   gio::AppLaunchContext::NONE);
               std::process::exit(0);
           } 
       }
   }));
   entry.connect_stop_search(clone!(@weak list_box => move |entry|{
       //list_box.remove(&relevant_box);

   }));
   scrolled_window.set_child(Some(&parent_box));

   on_app_activate(&application);
   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
