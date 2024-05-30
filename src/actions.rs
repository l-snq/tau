use gtk4::{gio, Application, ApplicationWindow};

pub fn on_app_activate(app: &Application) {
    let action = gio::ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            // window.close();
        })
        .build();
}
