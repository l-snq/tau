use gtk4::{gdk, Application, ApplicationWindow, gio::AppInfo, Shortcut, SearchBar, EventControllerKey};

pub fn input_handling(application: &Application) {
    // select specific app, then fetch the exec
    let event_controller = EventControllerKey::new();
    //let window = ApplicationWindow::new(application);
    
}

