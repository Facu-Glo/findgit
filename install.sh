#!/usr/bin/env bash

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}📦 Instalando findgit...${NC}"

# Crear directorio de configuración
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/findgit"
mkdir -p "$CONFIG_DIR"

# Crear archivo de configuración si no existe
CONFIG_FILE="$CONFIG_DIR/config.json"
if [[ ! -f "$CONFIG_FILE" ]]; then
    echo -e "${YELLOW}📝 Creando archivo de configuración...${NC}"
    
    SEARCH_PATHS="\"$HOME\""
    
    cat > "$CONFIG_FILE" << EOF
{
  "search_paths": [
    $SEARCH_PATHS
  ],
  "ignore_paths": [
    "node_modules",
    ".venv",
    "__pycache__",
    ".local"
  ]
}
EOF
    echo -e "${GREEN}✅ Archivo de configuración creado en: $CONFIG_FILE${NC}"
else
    echo -e "${YELLOW}📄 Archivo de configuración ya existe en: $CONFIG_FILE${NC}"
fi

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

ln -sf "$PWD/findgit.sh" "$INSTALL_DIR/findgit"
echo -e "${GREEN}✅ Script instalado en: $INSTALL_DIR/findgit${NC}"

# Verificar si ~/.local/bin está en PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "${YELLOW}⚠️  $HOME/.local/bin no está en tu PATH${NC}"
    echo "Agrega esta línea a tu ~/.bashrc o ~/.zshrc:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo "Luego recarga tu shell con: source ~/.bashrc"
fi

echo
echo -e "${GREEN}🎉 ¡Instalación completada!${NC}"
echo "Puedes editar la configuración en: $CONFIG_FILE"
echo "Uso: findgit"
