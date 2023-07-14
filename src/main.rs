use std::fs::File;
use std::io::{Error, Write};
use xdg;
use gtk::prelude::*;

fn build_gui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tau");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("click this damn button");
    window.add(&button);

    window.show_all();

}

fn main() {
    // TODO! shit

    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), 
        Default::default());

    application.connect_activate(build_gui);

    application.run();

    println!("yo this is tau");

}
