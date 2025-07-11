# 🔍 FindGit

A Bash script to quickly find and navigate Git repositories using `fzf`.

## ✨ Features

- 🔍 Search for Git repositories in user-defined directories  
- 🎨 Interactive interface with `fzf` and colored output  
- 📊 Preview showing Git status and directory content  
- ⚡ Prioritizes repositories with uncommitted changes  
- 🚫 Ignores specific directories to optimize search  
- 🛠️ Flexible configuration via JSON  

## 📋 Dependencies

- `jq` – JSON parsing  
- `fd` – Fast file search  
- `fzf` – Interactive selector  
- `eza` (optional) – Enhanced file listing  

### Installing dependencies

```bash
# Ubuntu/Debian
sudo apt install jq fd-find fzf

# Fedora/CentOS
sudo dnf install jq fd-find fzf

# Arch Linux
sudo pacman -S jq fd fzf
```

## 🚀 Installation

### Automatic installation

```bash
git clone https://github.com/Facu-Glo/findgit.git
cd findgit
chmod +x install.sh
./install.sh
```

### Manual installation

1. Clone the repository:
```bash
git clone https://github.com/Facu-Glo/findgit.git
cd findgit
```

2. Make the script executable:
```bash
chmod +x findgit.sh
```

3. Create the configuration directory:
```bash
mkdir -p "${XDG_CONFIG_HOME:-$HOME/.config}/findgit"
```

4. Create the configuration file:
```bash
cp config.json "${XDG_CONFIG_HOME:-$HOME/.config}/findgit/config.json"
```

5. (Optional) Create a symlink for global usage:
```bash
ln -sf "$PWD/findgit.sh" /.local/bin/findgit
```

## ⚙️ Configuration

The configuration file is located at:
```
${XDG_CONFIG_HOME:-$HOME/.config}/findgit/config.json
```

### Example config:
```json
{
  "search_paths": [
    "~/Proyectos",
    "~/dev",
    "~/workspace",
    "~/Documents/code"
  ],
  "ignore_paths": [
    "node_modules",
    ".venv",
    "__pycache__",
    ".git/objects",
    "vendor",
    "target",
    "build",
    "dist"
  ]
}
```

### Parameters:

- **`search_paths`**: Array of directories to search for Git repositories
- **`ignore_paths`**: Array of directories to ignore during search

## 🎯 Uso

```bash
# If installed globally
findgit

# If running directly
./findgit.sh
```

The script displays an interactive list of Git repositories where:

- Repos with uncommitted changes appear in red and are shown first
- Clean repos appear in **gray**
- The preview shows Git status and directory contents


### FZF controls:
- `↑/↓` o `Ctrl+j/k`: Navigate the list
- `Enter`: Select and print path
- `Esc` o `Ctrl+c`: Exit without selecting
- `Tab`: Select multiple entries (if enabled)

## 🔧 Customization

### Shell integration

You can create a function in your `.bashrc` or `.zshrc` to navigate directly:

```bash
# Function to integrate findgit with zsh
findgit_widget() {
  local selected_dir
  selected_dir=$("$HOME/.local/bin/findgit")
  if [[ -n "$selected_dir" ]]; then
    cd "$selected_dir"
  fi
  zle reset-prompt
}

# Register as zsh widget and bind to Ctrl+G
zle -N findgit_widget
bindkey '^G' findgit_widget
```


