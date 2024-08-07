use gtk4::{glib, prelude::*, Application};

pub mod actions;
pub mod run;
pub mod utils;

const APP_ID: &str = "org.tau.Main";

fn main() -> glib::ExitCode {
    // TODO!

    let app = Application::builder().application_id(APP_ID).build();

    //run::run_rs();

    app.connect_activate(run::draw_ui);
    app.run()
}
