# DidM Quick Start

## How Does DidM Work?

---

`DidM` essentially recreates the source file structure in the target directory by creating a symlink (or a copy, depending on your configuration) between the source folder and the target folder.

For example, let's say you have the following file tree in the source directory:

```
myDotfiles
├── didm.toml
├── fish
│   ├── completions
│   │   └── kitty.fish
│   └── config.fish
└── kitty
    └── kitty.conf
```

With a minimal configuration like this:

```toml
[sketch.all]
source_path = "~/myDotfiles"
target_path = "$XDG_CONFIG_HOME"

[composition.basic]
sketches = ["all"]
```

Running `DidM render basic` inside the `myDotfiles` directory results in:

```
~/.config
├── fish
│   ├── completions
│   │   └─ kitty.fish -> ~/myDotfiles/fish/completions/kitty.fish
│   └─ config.fish -> ~/myDotfiles/fish/config.fish
└── kitty
    └─ kitty.conf -> ~/myDotfiles/kitty/kitty.conf
```

This makes it easy to manage dotfiles using Git.

## Configuration Details

```toml
### *** indicates a required option.
### Any relative paths inside this file are resolved based on the current working directory (PWD).
#######################################
# You can include additional configuration files.
# !! Notice:
# Any relative paths inside an included file are resolved based on that file’s location.
# Default: []
include = ["./themes/palettes.toml"]

[skip_check]
#whether to skip check if the current directory is a git repository
#Default: false
is_git_workspace = false
#whether to skip check if the current directory is a symlink
#Default: false
is_working_dir_symlink = false
#whether to skip check if the path contains unresolved environment variables
#Default: false
unresolved_env = false

# Configuration priority order:
# sketch.override_behaviour > composition.override_behaviour > behaviour
[behaviour]
# Whether to overwrite an existing file or directory.
# Default: false (do not overwrite)
overwrite_existed = false

# If overwrite_existed is true, whether to back up the original file or directory before overwriting.
# Default: true (perform backup)
backup_existed = true

# Whether to stop execution upon encountering a command error.
# Default: false (continue execution)
stop_at_commands_error = false

[sketch.basic]
#***
# Source directory path
source_path = "./."

#***
# Target directory path
target_path = "$XDG_CONFIG_HOME"

# Whether to create a symlink or copy the files.
# Default: symlink
# Available options: symlink | copy
mode = "symlink"

# Only applicable in symlink mode.
# `dir`: link the entire directory.
# `file`: link individual files based on their relative path to the source directory.
# Available options: file | dir
# Default: file (link individual files)
unit = "file"

# Directories or files to ignore.
# Default: []
ignore = ["README.md"]

#only deal with files in ignore
#default:false
only_ignore=false

# Whether to respect `.gitignore` rules.
# Default: true (respect `.gitignore`)
respect_gitignore = true

# Whether to ignore hidden files.
# Default: false
ignore_hidden = false

# Files that will be linked to `/dev/null`.
# Relative path from `target_path`.
# !This will never overwrite existed files
# Default: []
null_files = []

# Files that should be created as empty files.
# Relative path resolved from `target_path`.
# !This will never overwrite existed files
# Default: []
empty_files = []

# Commands to run before applying the sketch.
# Default: []
pre_build_commands = [
    'echo -e "Deploying dotfiles for $(whoami)@$(hostname) \nusing theme: $target_theme"',
]

# Commands to run after applying the sketch.
# Default: []
post_build_commands = ['reboot']

# Overrides global behavior or composition behavior.
# Default: {}
override_behaviour = { overwrite_existed = true }

# extra_entries rules for specific files or directories.
extra_entries = [
    { source_path = "./bar.conf", target_path = "bar.conf" },
    { source_path = "./foo.conf", target_path = "foo.conf", mode = "copy" }, # Overrides default mode
]

# Environment variables for commands
# Default: {}
# if you have multiple variables you can do this:
# NOTICE:ALWAYS PUT THIS IN THE END OF A SECTION!
# [sketches.basic.environment]
# "icon_theme" = "Catppuccin-Mocha"
# "foo" = "bar"
environment = { "icon_theme" = "Catppuccin-Mocha" }

[composition.basic]
#***
# Sketches to execute
sketches = ["basic", "defaultTheme"]

# Commands to run before deploying the composition.
pre_build_commands = []

# Commands to run after deploying the composition.
post_build_commands = []

#Where will the commands be executed
#NOTICE: relative path will be resolved based on the main configuration directory
# Default:the main configuration directory
commands_path = ""

# Overrides global behavior.
override_behaviour = {}

# Environment variables for composition commands.
environment = []
```

## Advice

1. Use a main configuration file to define your primary sketches and compositions. Separate detailed settings into separate included files for better organization.
2. Avoid nested included files, as this can cause issues. Instead, place configuration files outside your source directory and organize them into separate folders.

## Known Issues

1. `DidM` will ignore any `symlinks` from source directory.

## Upcoming Features

### Variants

If you have configuration files with multiple variants—for instance, differing hardware settings between a laptop and desktop—you can use the switcher:

```hyprlang
# ~/myDotfiles/hypr/hyprland.conf
source=$XDG_CONFIG_HOME/hypr/hardware.conf
```

Organize files accordingly:

```
~/myDotfiles
└── hypr
    ├── didm_va_hardware.conf
    │   ├── desktop.conf
    │   └── laptop.conf
    └── hyprland.conf
```

Running `DidM render some_composition --variants laptop` will result in:

```
~/.config
└── hypr
    ├─ hardware.conf -> ~/myDotfiles/hypr/didm_va_hardware/laptop.conf
    └─ hyprland.conf -> ~/myDotfiles/hypr/hyprland.conf
```

### Color Palettes

Managing a unified color scheme across different programs can be difficult, but `DidM` will introduce a color palette switcher to make life easier. Here's how it works:

First, define a color palette:

```toml
[sketch.all]
source_path = "~/myDotfiles"
target_path = "$XDG_CONFIG_HOME"

[composition.basic]
sketches = ["all"]

[palette.my_palette]
base01 = "#000000"
# ...
base0F = "#BE5046"
extra01 = "#97E8A699"
```

Then, mark colors in your configuration files:

```css
/* ~/myDotfiles/waybar/style.css */
@define-color base   $<didm_palette_base01_hex>;
```

Running `DidM render some_composition --colorpalette my_palette` will replace variables in a copy of the relevant files and then link them to the target location.

```css
/* ~/.config/waybar/style.css */
/* link to ~/myDotfiles/didm_palelte_output/waybar/style.css */
@define-color base   #000000;
/* ... */
```

### Merger

This planned feature will allow merging configurations when a program does not support includes. Details are still being worked out.
