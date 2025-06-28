use gtk4::{gio, prelude::ListBoxRowExt, Label, SearchEntry, ListBox, ListBoxRow};
use std::process::Command;

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
    let _hello = echo_command.stdout;
}

pub fn applist_search_iter(user_text: String, vec_appinfo: Vec<APPINFO>, lb: ListBox) -> Result<(), Box<dyn std::error::Error>> {

   lb.remove_all();
   let mut app_list_iter: Vec<(usize, &APPINFO)> = vec_appinfo 
       .iter()
       .filter_map(|app| {
           let name_lower = app.name.to_lowercase();
           if let Some(pos) = name_lower.find(&user_text) {
               let lbl = Label::new(Some(&name_lower));
               let lbr = ListBoxRow::new();
               lbr.set_child(Some(&lbl));
               lb.prepend(&lbr);
               Some((pos, app))
           } else {
               None
           }
       })
       .collect();
   app_list_iter.sort_by_key(|&(pos, _)| pos);

   lb.select_row(lb.row_at_index(0).as_ref());

   if let Some(lb_row) = lb.row_at_index(0) {
       lb_row.changed();
   }

    Ok(())
}

pub fn sanitize_app_names(mut app_name_vector: Vec<String>) -> Vec<String> {
    app_name_vector.sort();
    app_name_vector.dedup();
    app_name_vector
}
