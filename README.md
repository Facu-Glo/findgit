#  FindGit

A Bash script to quickly find and navigate Git repositories using `fzf`.

![Image](https://github.com/user-attachments/assets/2697bebf-fc34-46f4-8f01-36fb58366ecf)

## Motivation
As the number of projects on my computer grew, I started noticing two recurring problems:

1. **Time loss:** Remembering and typing deep file system paths became tedious.
2. **Lack of visibility:** I would forget which projects had pending changes or uncommitted work.

**FindGit** was created to solve this by combining the speed of `fd` with the interactive interface of `fzf`, allowing me to visualize the state of my repositories and jump between them in milliseconds.

##  Features

-  Blazing Fast Search: Scans multiple directories in parallel.
-  Smart Sorting: Prioritizes repositories with pending changes, then by last commit date.
-  Interactive Interface: Powered by fzf with a rich, colored preview.
-  Highly Configurable: Simple JSON configuration for search and ignore paths.

##  Dependencies

### Required
- [`fzf`](https://github.com/junegunn/fzf) – Interactive selector  
- [`fd`](https://github.com/sharkdp/fd) – Fast file search  
- **Rust Toolchain** (to build from source).

### Recommended (for Previews)
- [`eza`](https://github.com/eza-community/eza) Modern `ls` replacement for directory icons and colors.

### Installing dependencies

```bash
# Arch Linux
sudo pacman -S fd fzf eza rustup
```

##  Installation

### 1. Clone the repository
```bash
git clone https://github.com/Facu-Glo/findgit.git
cd findgit
```
### 2. Build and Install using Makefile
The project includes a Makefile to handle everything.
```bash
make help      # See all available commands
make install   # Compiles in release mode and installs to ~/.local/bin/findgit
```
### 3. Shell Integration (Zsh/Bash)
To allow Findgit to change your shell directory, add this alias to your `.zshrc` or `.bashrc`:
```bash
# Primary Function
fgit() {
    local selected_dir
    selected_dir=$(findgit)
    if [[ -n "$selected_dir" ]]; then
        cd "$selected_dir"
    fi
}

# Optional: Zsh Widget (Ctrl+G)
fgit_widget() {
  local selected_dir
  selected_dir=$(findgit)
  if [[ -n "$selected_dir" ]]; then
    cd "$selected_dir"
  fi
  zle reset-prompt
}
zle -N fgit_widget
bindkey '^G' fgit_widget
```

##  Configuration

The config file is automatically created at:
```
~/.config/findgit/config.json
```

### Example config:
```json
{
  "search_paths": [
    "~/Repos",
    "~/Documents/Projects"
  ],
  "ignore_paths": [
    "node_modules",
    ".venv",
    "target",
    "build"
  ]
}
```

### Parameters:

- **`search_paths`**: Array of directories to search for Git repositories
- **`ignore_paths`**: Array of directories to ignore during search

##  Usage

Simply type `fgit` (if using the alias) or `findgit`.

- `↑/↓` or `Ctrl+j/k`: Navigate the list.
- `Enter`: Change directory to the selected repo.
- `Esc` or `Ctrl+c`: Exit.

The interface highlights:

- Red: Repositories with uncommitted changes.
- Gray: Clean repositories.
- Preview Top: git status summary.
- Preview Bottom: File listing with icons.
