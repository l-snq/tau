use std::fs::File;
use std::io::{Error, Write};
use xdg;
use gtk::prelude::*;
use gtk::{glib, Application};

pub mod run;
pub mod input;

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
