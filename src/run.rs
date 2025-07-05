use crate::{
    actions::on_app_activate,
    utils::{applist_search_iter, sanitize_app_names, APPINFO}
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk4::{
    gdk::Display, glib::clone, prelude::*, style_context_add_provider_for_display, STYLE_PROVIDER_PRIORITY_APPLICATION, Application, ApplicationWindow, CssProvider, IconLookupFlags, IconTheme, Image, SearchBar, SearchEntry, TextDirection};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::rc::Rc;

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

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
    list_box.invalidate_sort();
    list_box.invalidate_filter();
    list_box.add_css_class("container");
    let scrolled_window = gtk4::ScrolledWindow::builder()
        .name("scrollable window")
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();

    let apps = gio::AppInfo::all();
    let apps_rc = Rc::new(apps);
    let mut app_list_vec: Vec<APPINFO> = Vec::new();
    let matcher = Rc::new(SkimMatcherV2::default());

    let bar = SearchBar::builder()
        .valign(gtk4::Align::Start)
        .show_close_button(true)
        .build();
    bar.add_css_class("barContainer");
    let entry = SearchEntry::new();
    entry.set_hexpand(false);
    entry.set_vexpand(false);
    entry.add_css_class("entrybar");
    entry.set_widget_name("entry");
    entry.set_placeholder_text(Some("Start typing something..."));
    entry.set_key_capture_widget(Some(&list_box));

    // set the css styling for bar first.
    // In hiearchy from root to child It goes like: 
    // bar
    // entry
    // listbox
    // list row
    bar.connect_entry(&entry);
    bar.set_search_mode(true);
    bar.set_key_capture_widget(Some(&entry));

    let icon_theme = IconTheme::default();

    let parent_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
    parent_box.append(&list_box);

    let mut app_names_vec: Vec<String> = vec![];

    for app in apps_rc.iter() {
        let appinfo_init = APPINFO {
            name: app.display_name().to_string(),
            app_info: Some(app.clone()),
        };
        app_list_vec.push(appinfo_init);

        let app_name = app.display_name().to_string();

        let icon_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
        icon_box.grab_focus();
        icon_box.set_widget_name(&app_name);
        let image_icon_setup = Image::builder().pixel_size(50).build();

        // rendering out all the results in appInfo
        let lbr = gtk4::ListBoxRow::new();
        lbr.add_css_class("list_box_rows");
        lbr.set_widget_name(&app_name);
        let lbr_label = gtk4::Label::new(Some(&app_name));
        lbr.set_child(Some(&lbr_label));
        list_box.prepend(&lbr);

        // icon stuff
        if let Some(gtk_icon_name) = app.icon() {
            let some_icon_theme = Some(icon_theme.lookup_by_gicon(
                &gtk_icon_name,
                16,
                16,
                TextDirection::None,
                IconLookupFlags::FORCE_REGULAR,
            ));

            if some_icon_theme.is_some() {
                image_icon_setup.set_from_gicon(&gtk_icon_name);
            }
        }

        app_names_vec.push(app_name.to_lowercase());
    }
    app_names_vec.sort();
    parent_box.prepend(&entry);

    list_box.set_focusable(true);

    // continue some search entry logic here
    entry.connect_search_changed(clone!(
           @weak list_box, 
           @strong app_list_vec,
           @strong matcher
           => move |entry| {

       let user_text = entry
           .text()
           .to_string()
           .to_lowercase();
       sanitize_app_names(app_names_vec.clone());
       applist_search_iter(user_text.clone(), app_list_vec.clone(), list_box).expect("something went wrong when trying to match results");
    }));

    entry.connect_activate(clone!(@weak list_box, @strong apps_rc, @strong matcher => move |entry| {
       let user_string = entry.text().to_string();

       for app in apps_rc.iter() {
           let app_name = app.display_name().to_string();
           if matcher.fuzzy_match(app_name.as_str(), user_string.as_str()).is_some() {
               let _launch_command = gio::AppInfo::launch(
                   &app, 
                   &[], 
                   gio::AppLaunchContext::NONE);
               std::process::exit(0);
           } 
       }
   }));

   entry.connect_stop_search(clone!(@weak list_box,=> move |_|{
        std::process::exit(0);
   }));

    scrolled_window.set_child(Some(&parent_box));

    on_app_activate(&application);
    draw_window.set_child(Some(&scrolled_window));
    draw_window.set_size_request(100, 400);
    draw_window.set_visible(true);
}
