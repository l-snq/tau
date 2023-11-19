use gtk4::{ListBox, Box};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct AppField {
    pub app_name: String,
    pub exec: String,
}

impl AppField {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            exec: String::new(),
        }
    }

    pub fn update_fields(&mut self, app_name: String, exec: String) {
        self.app_name = app_name;
        self.exec = exec;
    }
}

pub fn selection_process(ibox: &Box, app_hashmap: &HashMap<String, &Box>) {
    // compare ibox name with hashmap.keys(), right?
}
