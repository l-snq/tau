use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk4::{gio, prelude::AppInfoExt, prelude::{ListBoxRowExt, *}, Label, SearchEntry, ListBox, ListBoxRow};
use std::process::Command;
use fst::{IntoStreamer, automaton::Levenshtein, Set};
use regex_automata::dense;

#[derive(Debug, Clone)]
pub struct APPINFO {
    pub name: String,
    pub app_info: Option<gio::AppInfo>,
}


pub fn string_to_command(input: &str) {
    let fms_str = &input.trim().to_lowercase();

    let echo_command = Command::new(&fms_str)
        .spawn()
        .expect("something went wrong trying to read the command");
    let hello = echo_command.stdout;
}

pub fn applist_search_iter(user_text: String, vec_appinfo: Vec<APPINFO>, lb: ListBox, s_ent: &SearchEntry) -> Result<(), Box<dyn std::error::Error>> {

   let mut app_list_iter: Vec<(usize, &APPINFO)> = vec_appinfo 
       .iter()
       .filter_map(|app| {
           let name_lower = app.name.to_lowercase();
           if let Some(pos) = name_lower.find(&user_text) {
               Some((pos, app))
           } else {
               None
           }
       })
       .collect();
   app_list_iter.sort_by_key(|&(pos, _)| pos);

    Ok(())
}

pub fn fst_worker(user_text: String, app_names_vec: Vec<String>, lb: ListBox, s_ent: &SearchEntry) -> Result<(), Box<dyn std::error::Error>> {
   let fst_set = Set::from_iter(app_names_vec.clone())?;
   let pattern = r"(i?)";
   let dfa = dense::Builder::new().anchored(true).build(&pattern).unwrap();
   let lev = Levenshtein::new(&user_text, 2)?;

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

pub fn sanitize_app_names(app_name_vector: Vec<String>) -> Vec<String> {
    let mut sanitized_names = app_name_vector.clone();
    sanitized_names.sort();
    sanitized_names.dedup();

    for i in &sanitized_names {
       let _ = i.replacen("s", "+", 90);
    }

    sanitized_names
}
