#!/usr/bin/env bash

CONFIG="${XDG_CONFIG_HOME:-$HOME/.config}/findgit/config.json"

# Verificar dependencias
for cmd in jq fd fzf; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "Error: $cmd no está instalado" >&2
        exit 1
    fi
done

if [[ ! -f "$CONFIG" ]]; then
    echo "Error: Archivo de configuración no encontrado: $CONFIG" >&2
    echo "Crea el archivo con: mkdir -p \"\$(dirname \"$CONFIG\")\" && echo '{\"search_paths\": [\"$HOME\"], \"ignore_paths\": []}' > \"$CONFIG\""
    exit 1
fi

if ! search_paths=($(jq -r '.search_paths[]' "$CONFIG" 2>/dev/null)); then
    echo "Error: No se pudieron leer las rutas de búsqueda del archivo de configuración" >&2
    exit 1
fi

if ! ignore_paths=($(jq -r '.ignore_paths[]' "$CONFIG" 2>/dev/null | sed 's/^/-E /')); then
    ignore_paths=()
fi

if [[ ${#search_paths[@]} -eq 0 ]]; then
    echo "Error: No hay rutas de búsqueda configuradas" >&2
    exit 1
fi

_findgit() {
    local dir
    dir=$(
        fd -t d -H .git "${search_paths[@]}" "${ignore_paths[@]}" --exec dirname {} \; 2>/dev/null | \
        sort -u | \
        while read -r d; do
            if git -C "$d" rev-parse --is-inside-work-tree &>/dev/null && \
               [[ "$(git -C "$d" rev-parse --show-toplevel 2>/dev/null)" == "$d" ]]; then
                if [[ -n $(git -C "$d" status --short 2>/dev/null) ]]; then
                    echo -e "1\t$d\t\033[31m$d\033[0m"
                else
                    echo -e "2\t$d\t\033[37m$d\033[0m"
                fi
            fi
        done | \
        sort | \
        cut -f2,3 | \
        fzf --ansi --with-nth=2 --preview '
            echo -e "\033[32m󰊢 Git Status:\033[0m"
            status_output=$(git -C {1} -c color.status=always status --short 2>/dev/null)
            if [[ -n "$status_output" ]]; then
                echo "$status_output"
            else
                echo "No hay modificaciones"
            fi
            echo -e "\n📁 Contenido:"
            if command -v eza &> /dev/null; then
                eza --color=always -l {1} 2>/dev/null || ls -la {1}
            else
                ls -la {1}
            fi
        ' | cut -f1
    )
    
    if [[ -n $dir ]]; then
        echo "$dir"  
    fi
}

_findgit
