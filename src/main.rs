use std::fs::File;
use std::io::{Error, Write};
use xdg;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn build_gui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("tau")
        .build();

    window.present();
}

const APP_ID: &str = "org.tau.Main";

fn main() -> glib::ExitCode {
    // TODO! 

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_gui);
    app.run()
}
