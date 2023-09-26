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

the other aspect of this project is maybe using gtk-layer-shell? The reason why, is because rust's winit crate isn't the best for its wayland support. 
Maybe using gtk I can access .desktop files?

instead, i think i could use gio which is a library used in turn with gtk to query desktop files

So i want to be able to provide a list of the appInfo stuff. Go into run.rs, and figure out how to render a label as an individual UI element. Then do a forloop to render each of those items for the label.

Also, at some point, i really need to transfer from gtk3 to gtk4.

so, i added in a label that would iterate through your
apps and display those apps to a label within a list
box. nice. next i want to do proper styling by adding
it into a table. OR I can just use labels. Time to try and implement icons now though

## TODO! I need to upgrade to gtk4.
1. I named the branch wrong, however, I need to upgrade to gtk4 because
   layer_shell is not properly working with gtk3.
   But, gtk layer shell is not possible with gtk4. I need to figure out what to do fully.
   See: https://github.com/wmww/gtk-layer-shell/issues/37
2. The issue seems interesting. It's a problem with the Gtk surfaces attaches
   to an xdg_surface, which is not good since i'm using wayland. I need to get
   rid of this somehow
3. Another option, still is to use the auto generated bindings this guy did for the gtk4, to support layer shell.
wmww gtk4-layer-shell might be the go to.

## Suggestions: 
1. Use gtk's menu, and menu items. Ex: 
https://github.com/gtk-rs/gtk3-rs/blob/master/examples/menu_bar/

## Regarding customization, I really don't know how I should approach the problem.

