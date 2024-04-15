Regarding customization, I really don't know how I should approach the problem.

let's write some pseudo code:

for app in app {
    hash.set(list_box, appinfo)

    // the whole logic for adding the UI elements will be moved
}

entry.connect_search_started(clone!(@weak => move|entry| {
    // do the whole logic for adding UI elements HERE

    let user_text = entry.text()
    let app_string_captured = appinfo::name(&user_text)
    let app_string = hash.get_value(&user_text) 
    let result_box = gtk::Box::new(gtk::Orientation::horizontal, 20)

    result_box.prepend(&app_string)
    // this will make the relevant apps show up.

    if user_text == app_string_captured {
       result_box.prepend(&app_string_captured);
    } else {
        println!("nothing to show, nothing has been relevantly matched")
    }
        
    // maybe use a loop to iterate through all apps?
}))
