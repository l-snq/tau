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
   let mut pattern = r"(i?)".to_owned();
   let dfa = dense::Builder::new().anchored(true).build(&pattern).unwrap();
   let lev = Levenshtein::new(&user_text, 3)?;

   println!("{}", pattern);
   lb.remove_all();
   let some_entry = Some(s_ent);
   let stream = fst_set.search(lev).into_stream();
   let keys = stream.into_strs().unwrap_or_default(); // this returns a Vec<String>

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
