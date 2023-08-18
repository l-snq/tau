## What to do
There are a couple ideas I have. I really don't
know what i'm doing at all in this regard, i've
never written software for linux and this is my
first really "big" rust project. That being said, I
think i have an entry point: XDG. Specifically, autostart & .desktop files.

From my understanding, .desktop files are files
that contain information of your app, such as
icons, name and description. According to the xdg
specification:

`https://wiki.freedesktop.org/www/Specifications/`

but then there's another specification, called
`desktop basedir` which would be used to query
through all the .desktop files you have, maybe?

There's a repo called rust-xdg which provides
desktop basedir support, maybe I could fork it and
make it compatible with getting .desktop files
following the freedesktop specs?

the other aspect of this project is maybe using gtk-layer-shell? The reason why, is because rust's winit crate isn't the best for its wayland support. 
Maybe using gtk I can access .desktop files?

instead, i think i could use gio which is a library used in turn with gtk to query desktop files

So i want to be able to provide a list of the appInfo stuff. Go into run.rs, and figure out how to render a label as an individual UI element. Then do a forloop to render each of those items for the label.

Also, at some point, i really need to transfer from gtk3 to gtk4.

Let's do some pseudo code to verbalize everything here.
So I want to get the Exec aspect of every desktop file. For every app in Gio::AppInfo, I want to load those exec shortcuts
`
let name = gio::AppInfo

for app in apps {
  Command::new(&name) 
  .args()
  .output();
}
`

in Command::new(), which is a way to take commands and actually output them into terminal, i would want to grab the exec for each app. How do I do that? Maybe with gio::DesktopAppInfo ?
## Suggestions: 
1. Use gtk's menu, and menu items. Ex: 
https://github.com/gtk-rs/gtk3-rs/blob/master/examples/menu_bar/

## Regarding customization, I really don't know how I should approach the problem.

