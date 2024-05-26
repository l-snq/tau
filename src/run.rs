use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gio::AppInfo;
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};
use gtk4::{
    gio, glib::{self, clone}, prelude::*, Application, ApplicationWindow, CustomSorter, IconLookupFlags, IconTheme, Image, ListBoxRow, SearchBar, SearchEntry, TextDirection
};
use std::collections::{HashMap};
use crate::{actions::on_app_activate, utils::{AppField, sorting_function}};

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

   let list_box = gtk4::ListBox::new();
   let scrolled_window = gtk4::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();

   let mut hash = HashMap::new();

   let apps = gio::AppInfo::all(); 

   let bar = SearchBar::builder()
       .valign(gtk4::Align::Start)
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

   let parent_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
   parent_box.append(&list_box);

   for app in apps {
       let app_name = app.display_name().to_string();

       let icon_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
       icon_box.grab_focus();
       icon_box.set_widget_name(&app_name);
       let image_icon_setup = Image::builder()
            .pixel_size(50)
            .build();
       let title = gtk4::Label::new(Some(&app_name));

       // rendering out all the results in appInfo
       let lbr = gtk4::ListBoxRow::new();
       let lbr_label = gtk4::Label::new(Some(&app_name));
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
   

   let mut app_info_vector = vec![];
   // continue some search entry logic here
   entry.connect_search_changed(clone!(@weak list_box => move |entry| {
       let relevant_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
       relevant_box.set_focusable(true);
       let user_text = entry
           .text()
           .to_string()
           .to_lowercase();
       let apps = gio::AppInfo::all();
       for app in apps {
           let matcher = SkimMatcherV2::default();
          
           app_info_vector.push(app.clone());
           let mut app_vec_clone = app_info_vector.clone(); 
           app_vec_clone.sort_unstable(); 
           app_vec_clone.dedup();
           for i in app_vec_clone {
               let item_name = i.name();
               let label = gtk4::Label::new(Some(&item_name));
               let lbr = ListBoxRow::new();
               if matcher.fuzzy_match(&item_name, &user_text).is_some() {
                   lbr.set_child(Some(&label));
                   list_box.prepend(&lbr);
               }

        };
           // good god what have i done
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
   
   //list_box.set_filter_func(sorting_function(, user_text))
   scrolled_window.set_child(Some(&parent_box));

   on_app_activate(&application);
   draw_window.set_child(Some(&scrolled_window));
   draw_window.set_size_request(100, 400);
   draw_window.show();
}
