Regarding customization, I really don't know how I should approach the problem.

***Restructure with App Structs that contain ID, display name, exec, and AppInfo ***
Do this so it becomes easier to look up during the search w/ regex
Hash doesn't store enough info that we would need.

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

