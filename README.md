# TAU. 
An application launcher made for wlroots based compositors. 

## Instructions:

### Requirements:
1. gtk4 installed
2. gtk4-layer-shell installed


### Building steps:

1. Clone this repo `git clone https://github.com/l-snq/tau`
2. `cd` into the repo and run: ``cargo run`

### TODOS: 

High-level approach:

Parse all .desktop files once at startup into some kind of searchable data structure (like a Vec of structs containing name, description, exec path, etc.)
Keep that list in memory
When the user types, filter the existing list based on their input instead of re-scanning files

For the filtering/search part, you could:

Do simple string matching on app names/descriptions
Maybe implement fuzzy matching if you want something fancier
Keep it responsive by filtering on each keystroke

Architecture-wise, you'd probably want:

One function/module to handle the initial .desktop file parsing
Another to handle the search/filtering logic
The GTK UI just calls the search function with the current input


## Contributions:

Open a PR! This is my first open sourced project, so feel free to contribute :)
