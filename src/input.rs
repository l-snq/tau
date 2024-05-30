use gtk4::{
    gdk, gio::AppInfo, Application, ApplicationWindow, EventControllerKey, SearchBar, Shortcut,
};

pub fn input_handling(application: &Application) {
    // select specific app, then fetch the exec
    let event_controller = EventControllerKey::new();
    //let window = ApplicationWindow::new(application);
}
