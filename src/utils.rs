use uuid::Uuid;
use gtk4 as gtk;
use gtk::{EventControllerKey, ListBox, Box, gio, prelude::WidgetExt};
use std::{collections::HashMap, process::Command};
#[derive(Debug, Clone)]
pub struct AppField {
    pub app_name: String,
    pub exec: String,
    pub id: Uuid,
}

impl AppField {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            exec: String::new(),
            id: Uuid::new_v4(),
        }
    }

    pub fn update_fields(&mut self, app_name: String, exec: String) {
        self.app_name = app_name;
        self.exec = exec;
        self.id = Uuid::new_v4();
    }
}

pub fn string_to_command(input: &str) {
    // this will take the string, to lowercase, and then remove any spaces with split(' ')
    let fms_str = &input.trim().to_lowercase();

    println!("the string that is formatted= {:?}", &fms_str);
    let mut echo_command = Command::new(&fms_str).spawn().expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}

pub fn widget_comparison(widget1: &gtk::Widget, widget2: &gtk::Widget) -> bool {
    // wrapping widget1 and widget2 in Some() to match types
    if let (Some(name1), Some(name2)) = (
        Some(widget1.widget_name().to_string()), 
        Some(widget2.widget_name().to_string())
    ) {
        return name1 == name2;
    };
    false
}
