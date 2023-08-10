use gio::AppInfo;
use gtk::{Application, ApplicationWindow, SearchBar};

pub fn compare_inputs(label: &str, input: &str) {
    // pass in label here and compare it to user input

    if label == input {
        return println!("your items match {:?}, {:?}", label, input); 
    }
}
