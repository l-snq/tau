#[derive(Debug, Clone)]
pub struct AppField {
    app_name: String,
    exec: String,
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

