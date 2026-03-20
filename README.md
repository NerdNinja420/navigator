# Navigator - Fuzzy Directory Navigation Tool

<video controls width="100%">
    <source src="./assets/navigator.mp4" type="video/mp4">
    Your browser does not support the video tag.
</video>

## Purpose

- Provides fuzzy directory navigation and completion for shell environments.
- Simple prefix-based matching with ambiguity detection.

## Usage

```sh
./target/release/navigator [--complete] <base_dir> [args...]
```

### Arguments

| Argument      | Description                                      |
|---------------|--------------------------------------------------|
| `--complete`  | Output tab-completion matches                     |
| `<base_dir>`  | The root directory to navigate from              |
| `[args...]`   | Directory name prefixes to match                 |

### Examples

```sh
./target/release/navigator /home/user/documents pro
./target/release/navigator --complete /home/user/documents pr
```

## Usage Example 

Add this to your `~/.zshrc`:

```zsh
# source ~/.config/zshrc/scripts/logging.sh
local _NAV='PATH to the executable'

function conf() {
  local dir=$($_NAV "$HOME/.config/" "$@") || return 1
  clear && cd "$dir"
  # cmd "eza $dir"
  eza --long --header -a --icons --git --group-directories-first --group
}
```

Then use it like:
- `conf zs` - Navigate to `~/.config/zshrc`
- `conf nv` - Navigate to `~/.config/nvim`
- `conf hy` - Navigate to `~/.config/hypr`

### Tab Completion

```zsh
#compdef conf

local -a opts
local target_dir=$HOME/.config
local _NAV='Path to executable'

opts=($(_NAV --complete $target_dir "${(@)words[2,CURRENT]}"))
compadd -S '' -a opts
```

## Requirements

- Rust 1.75 or later
- logging crate (git dependency: git@github.com:NerdNinja420/logging.git)

## Installation

### Arch Linux

```sh
```

## License

This project is licensed under the MIT License.
