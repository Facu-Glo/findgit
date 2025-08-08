# 🔍 FindGit

A Bash script to quickly find and navigate Git repositories using `fzf`.
![Image](https://github.com/user-attachments/assets/d388eb13-2208-465b-9d42-bbf536937341)
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
sudo apt install jq rust-fd-find fzf rust-eza

# Fedora
sudo dnf install jq rust-fd-find fzf rust-eza

# Arch Linux
sudo pacman -S jq fd fzf eza
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
chmod +x findgit
```

3. Create the configuration directory:
```bash
mkdir -p "${XDG_CONFIG_HOME:-$HOME/.config}/findgit"

```

4. Create the configuration file:
```bash
cat > "${XDG_CONFIG_HOME:-$HOME/.config}/findgit/config.json" <<EOF
{
  "search_paths": [
    "~/Paths/To/Your/Projects",
    "~/HOME"
  ],
  "ignore_paths": [
    "path/to/ignore",
    "yay",
    "node_modules",
    ".venv",
    "__pycache__",
    ".git/objects"
  ]
}
EOF
```

5. Create a hardlink for global usage:
```bash
ln "$PWD/findgit" /.local/bin/findgit
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

## 🎯 Use

```bash
# If installed globally
findgit

# If running directly
./findgit
```

The script displays an interactive list of Git repositories where:

- Repos with uncommitted changes appear in red and are shown first
- Clean repos appear in **gray**
- The preview shows Git status and directory contents


### FZF controls:
- `↑/↓` o `Ctrl+j/k`: Navigate the list
- `Enter`: Select 
- `Esc` o `Ctrl+c`: Exit without selecting

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


