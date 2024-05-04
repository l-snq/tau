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

**** OUTLINE ****
Pseudo code
```
fn run() {
    set_up_window() // arbitrary code here for setting up window & layer
    let list_box // declared
    let parent_box // declared
    for app in apps {
        hash.insert(gtk::Box(app box technically), app.clone())
    }
    
    let entry
    entry.connect_search_changed(|list_box, entry|{
        let EVC(move |list_box| {
            let cloned_entry = entry.clone()
            let app_name_hash
            app in apps {
                app_name_hash.insert(app_name, app.clone())
            }
            if app_name_hash.contains(entry.text().to_string()) {
                
                
            }

        })

    })
}

fn main() {
    run()
} 
```

