use gtk4 as gtk;
use gtk::{gdk, Application, ApplicationWindow, gio::AppInfo, Shortcut, SearchBar, EventControllerKey, glib::Cast};

pub fn input_handling(application: &Application) {
    // select specific app, then fetch the exec
    let event_controller = EventControllerKey::new();
    //let window = ApplicationWindow::new(application);
    
}

