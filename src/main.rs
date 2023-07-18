use std::fs::File;
use std::io::{Error, Write};
use xdg;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, ListBox, SearchEntry };

pub mod run;

fn build_gui(app: &Application) {

    let window = ApplicationWindow::new(app);
    let button = Button::with_label("hey whats up");

    window.add(&button);

    window.show_all();
}

const APP_ID: &str = "org.tau.Main";

fn main() -> glib::ExitCode {
    // TODO! 

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    run::run_rs();    

    app.connect_activate(run::draw_ui);
    app.run()
}
