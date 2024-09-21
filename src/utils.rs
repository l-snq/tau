use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk4::{gio, prelude::AppInfoExt, prelude::{ListBoxRowExt, *}, Label, SearchEntry, ListBox, ListBoxRow};
use std::process::Command;
use fst::{IntoStreamer, automaton::Levenshtein, Set};
use regex_automata::dense;

#[derive(Debug, Clone)]
pub struct AppField {
    pub app_name: String,
    pub app_info: Option<gio::AppInfo>,
    pub id: Option<String>,
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
    let fms_str = &input.trim().to_lowercase();

    let echo_command = Command::new(&fms_str)
        .spawn()
        .expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}

pub fn fst(user_text: String, app_names_vec: Vec<String>, lb: ListBox, s_ent: &SearchEntry) -> Result<(), Box<dyn std::error::Error>> {
   let fst_set = Set::from_iter(app_names_vec.clone())?;
   let pattern = r"(i?)";
   let dfa = dense::Builder::new().anchored(true).build(&pattern).unwrap();
   let lev = Levenshtein::new(&user_text, 3)?;

   println!("{}", pattern);
   lb.remove_all();
   let some_entry = Some(s_ent);
   let stream = fst_set.search(lev).into_stream();
   let keys = stream.into_strs().unwrap_or_default(); 

   if some_entry.is_some() {
           for i in keys {
               let item = i.to_owned();
               let lbl = Label::new(Some(&item)); 
               let lbr = ListBoxRow::new();
               lbr.set_child(Some(&lbl));
               lb.prepend(&lbr);
           }
   }

   lb.select_row(lb.row_at_index(0).as_ref());

   if let Some(lb_row) = lb.row_at_index(0) {
       lb_row.changed();
   }

   Ok(())
}

pub fn sort_app_vec(search_text: String, app_vec: Vec<String> ) {
    // take all of the results in the app vec, and push them into a singular string
    // and clean them up 
    let mut app_string = String::new();
    for i in app_vec {
        app_string.push_str(&i);
        app_string.push_str("+");
    }
    let matcher = SkimMatcherV2::default();
    if let Some(fuzzy_value) = matcher.fuzzy_match(&app_string, &search_text) {
        println!("{:?}", fuzzy_value);
    }
}

pub fn sanitize_app_names(app_name_vector: Vec<String>) -> Vec<String> {
    let mut sanitized_names = app_name_vector.clone();
    sanitized_names.sort();
    sanitized_names.dedup();

    for i in &sanitized_names {
       i.replacen(" ", "+", 10);
    }
    println!("{:?}", sanitized_names);

    sanitized_names
}
