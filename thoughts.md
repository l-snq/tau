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


## Regarding customization, I really don't know how I should approach the problem.

