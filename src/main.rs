use gtk4::{glib, prelude::*, Application};

pub mod actions;
pub mod run;
pub mod utils;

const APP_ID: &str = "org.tau.Main";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| run::load_css());
    app.connect_activate(run::draw_ui);
    app.run()
}
