App Field {
    name string
    app info option<app Info>
}

let mut appfieldvec: AppInfo

apps = gio::AppInfo::all()

for app in apps {

    let appinfostruct = App Field {
       name: app.displayname.toString() 
       app info: some(app)
    }

    appfieldvec.push(appinfostruct)
}

connect search changed {
    appfieldvec.filter().map(|| fuzzy search this shit)

}

connect search activate {
    appfieldvec.filter().map(|| fuzzy search this to get exact match)
    then activate under current selection in the row by calling launch on the exact App Info in the     struct field of App info.

    
}
