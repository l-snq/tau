## TODO! I need to upgrade to gtk4. ()COMPLETED !!!!!!!!
   layer_shell is not properly working with gtk3.
   But, gtk layer shell is not possible with gtk4. I need to figure out what to do fully.
   See: https://github.com/wmww/gtk-layer-shell/issues/37
2. The issue seems interesting. It's a problem with the Gtk surfaces attaches
   to an xdg_surface, which is not good since i'm using wayland. I need to get
   rid of this somehow
3. Another option, still is to use the auto generated bindings this guy did for the gtk4, to support layer shell.
wmww gtk4-layer-shell might be the go to.

## TRY TO FIGURE OUT HOW TO GET THE EXEC SPECS FOR EACH APP LAUNCHED
I did it so that I'm just comparing the widget from the hashmap to the specific_row_child widget. Pretty easy, might get rid of my comparison function since it's pointless now lol.

## Suggestions: 
1. Use gtk's menu, and menu items. Ex: 
https://github.com/gtk-rs/gtk3-rs/blob/master/examples/menu_bar/

## Regarding customization, I really don't know how I should approach the problem.

