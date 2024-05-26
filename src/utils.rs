use gtk4::{gio, prelude::AppInfoExt};
use std::process::Command;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

 #[derive(Debug, Clone)]

pub struct AppField {
    pub app_name: String,
    pub app_info: Option<gio::AppInfo>,
    pub id: Option<String>, // this is annoying. You have to unwrap it, 
                            // then turn it into a string,
                            //AND THEN WRAP IT AGAIN
}

impl AppField {
    pub fn new(info: gio::AppInfo) -> Self {
        Self {
            app_name: String::new(),
            app_info: Some(info),
            id: Some(String::new()),
        }
    }

    pub fn update_fields(&self) {
        let _ = self.app_name.clone();
        let _ = self.app_info.clone();
        let _ = self.id.clone();
    }
}

pub fn string_to_command(input: &str) {
    // this will take the string, to lowercase, and then remove any spaces with split(' ')
    let fms_str = &input.trim().to_lowercase();

    println!("the string that is formatted= {:?}", &fms_str);
    let echo_command = Command::new(&fms_str)
        .spawn()
        .expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}

// this shouldn't be used!!!! but im stashing it c:
pub fn hash_match_and_launch_app(
    widget: gtk4::Widget, 
    hash: &std::collections::HashMap<gtk4::Box, gio::AppInfo>) {
     let query_child = &widget;
     let _hashed_child = hash.contains_key(query_child);
     let captured_app = hash.get(query_child).unwrap();
     let _launch_app = gio::AppInfo::launch(
         &captured_app, 
         &[], 
         gio::AppLaunchContext::NONE);
}

pub fn sorting_function(app_name: String, user_text: String,) {
    let matcher = SkimMatcherV2::default();

    if matcher.fuzzy_match(&app_name, &user_text).is_some() {
        println!("///////////////////theres a match");
    };
}
