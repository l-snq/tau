use gio::ffi::g_input_stream_has_pending;
use uuid::Uuid;
use gtk4::{EventControllerKey, ListBox, Box};
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
    let mut echo_command = Command::new(&fms_str).output().expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}
