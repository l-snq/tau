use gtk4::{gio, Application, ApplicationWindow};

pub fn on_app_activate(_app: &Application) {
    let _action = gio::ActionEntry::builder("close")
        .activate(|_window: &ApplicationWindow, _, _| {
        })
        .build();
}
