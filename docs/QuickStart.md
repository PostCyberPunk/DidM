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
[profiles.all]
source_path = "~/myDotfiles"
target_path = "$XDG_CONFIG_HOME"

[plans.basic]
profiles = ["all"]
```

Running `didm deploy basic` inside the `myDotfiles` directory results in:

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

# Configuration priority order:
# profile.override_behaviour > plan.override_behaviour > behaviour
[behaviour]
# Whether to overwrite an existing file or directory.
# Default: false (do not overwrite)
overwrite_existed = false

# If overwrite_existed is true, whether to back up the original file or directory before overwriting.
# Default: true (perform backup)
backup_existed = true

# Whether to update existing symlinks.
# Default: true (update symlinks)
update_symlink = true

# Whether to stop execution upon encountering a command error.
# Default: false (continue execution)
stop_at_commands_error = false

[profiles.basic]
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

# Exceptions to the default unit behavior:
# If `unit` is `"file"`, then link the entire directory instead.
# If `unit` is `"dir"`, then link all files inside the directory.
# Default: []
exceptions = ["foo", "bar.conf"]

# Directories or files to ignore.
# Default: []
ignore = ["README.md"]

# Whether to respect `.gitignore` rules.
# Default: true (respect `.gitignore`)
respect_gitignore = true

# Files that will be linked to `/dev/null`.
# Relative path from `target_path`.
# Default: []
null_files = []

# Files that should be created as empty files.
# Relative path resolved from `target_path`.
# Default: []
empty_files = []

# Commands to run before applying the profile.
# Default: []
pre_build_commands = [
    'echo -e "Deploying dotfiles for $(whoami)@$(hostname) \nusing theme: $target_theme"',
]

# Commands to run after applying the profile.
# Default: []
post_build_commands = ['reboot']

# Overrides global behavior or plan behavior.
# Default: {}
override_behaviour = { overwrite_existed = true }

# Extra rules for specific files or directories.
extra_rules = [
    { source_path = "./bar.conf", target_path = "bar.conf" },
    { source_path = "./foo.conf", target_path = "foo.conf", mode = "copy" }, # Overrides default mode
]

# Environment variables for commands
# Default: {}
# if you have multiple variables you can do this:
# NOTICE:ALWAYS PUT THIS IN THE END OF A SECTION!
# [profiles.basic.environment]
# "icon_theme" = "Catppuccin-Mocha"
# "foo" = "bar"
environment = { "icon_theme" = "Catppuccin-Mocha" }

[plans.basic]
#***
# Profiles to execute
profiles = ["basic", "defaultTheme"]

# Commands to run before deploying the plan.
pre_build_commands = []

# Commands to run after deploying the plan.
post_build_commands = []

# Overrides global behavior.
override_behaviour = {}

# Environment variables for plan commands.
environment = []
```

## Advice

1. Use a main configuration file to define your primary profiles and plans. Separate detailed settings into separate included files for better organization.
2. Avoid nested included files, as this can cause issues. Instead, place configuration files outside your source directory and organize them into separate folders.

## Upcoming Features

### Color Palettes

Managing a unified color scheme across different programs can be difficult, but `DidM` will introduce a color palette switcher to make life easier. Here's how it works:

First, define a color palette:

```toml
[profiles.all]
source_path = "~/myDotfiles"
target_path = "$XDG_CONFIG_HOME"

[plans.basic]
profiles = ["all"]

[palettes.my_palette]
base01 = "#000000"
# ...
base0F = "#BE5046"
extra01 = "#97E8A699"
```

Then, mark colors in your configuration files:

```css
/* ~/myDotfiles/waybar/style.css */
@define-color base   $<didm_color_base01_hex>;
```

Running `didm deploy some_plan --colorpalette my_palette` will replace variables in a copy of the relevant files and then link them to the target location.

```css
/* ~/.config/waybar/style.css */
/* link to ~/myDotfiles/didm_palelete_output/waybar/style.css */
@define-color base   #000000;
/* ... */
```

### Switcher

If you have configuration files with multiple variants—for instance, differing hardware settings between a laptop and desktop—you can use the switcher:

```hyprlang
# ~/myDotfiles/hypr/hyprland.conf
source=$XDG_CONFIG_HOME/hypr/hardware.conf
```

Organize files accordingly:

```
~/myDotfiles
└── hypr
    ├── _didsw_hardware.conf
    │   ├── desktop.conf
    │   └── laptop.conf
    └── hyprland.conf
```

Running `didm deploy some_plan --switch laptop` will result in:

```
~/.config
└── hypr
    ├─ hardware.conf -> ~/myDotfiles/hypr/_didsw_hardware/laptop.conf
    └─ hyprland.conf -> ~/myDotfiles/hypr/hyprland.conf
```

### Merger

This planned feature will allow merging configurations when a program does not support includes. Details are still being worked out.
