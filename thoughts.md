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

1. Hash is created. Has no types specified for the key and value.
using the insert method, i clone two things: 

****** THE HASH HAS THE APP_LAUNCH AS A KEY
****** AND ICON_BOX AS THE VALUE, WHICH CONTAINS THE WIDGET_NAME AND ICON

   1. app_name(AppInfo.all().display_name().tostring() variable and the 
   2. app_launch(AppInfo.create_from_commandline("xterm, Some(&app_name), Flags) variable  

but i'm running into an error where it's saying theres a type mismatch for the app_launch? 
Presumably with the application name?

Now i get the issue. On line 84, I am setting the key_value_of_child_widget_name to specific_row_child.widget_name().to_string();
This means that key_value_of_child_widget_name has the type of String, whereas my hash key value is of app_name && app_launch

## Suggestions: 
1. Use gtk's menu, and menu items. Ex: 
https://github.com/gtk-rs/gtk3-rs/blob/master/examples/menu_bar/

## Regarding customization, I really don't know how I should approach the problem.

