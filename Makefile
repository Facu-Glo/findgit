CYAN  := \033[36m
GREEN := \033[32m
RESET := \033[0m
BOLD  := \033[1m

BINARY_NAME=findgit_rs
FINAL_NAME=findgit
INSTALL_DIR=$(HOME)/.local/bin
CONFIG_DIR=$(HOME)/.config/findgit
SOURCE_FILES=$(shell find src -type f -name "*.rs")

.PHONY: all build install clean help

all: build 

build: target/release/$(BINARY_NAME) 

target/release/$(BINARY_NAME): $(SOURCE_FILES)
	@echo "Building in release mode..."
	@cargo build --release

install: build 
	@echo "Installing binary to $(INSTALL_DIR)/$(FINAL_NAME)..."
	@mkdir -p $(INSTALL_DIR)
	@cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(FINAL_NAME)
	@chmod +x $(INSTALL_DIR)/$(FINAL_NAME)
	@echo "Checking configuration..."
	@mkdir -p $(CONFIG_DIR)
	@if [ ! -f $(CONFIG_DIR)/config.json ]; then \
		echo '{"search_paths": ["$(HOME)"], "ignore_paths": ["node_modules", ".venv", "target"]}' > $(CONFIG_DIR)/config.json; \
		echo "Initial configuration created."; \
	fi
	@echo "Installation completed."

quick-install: 
	@if [ -f target/release/$(BINARY_NAME) ]; then \
		cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(FINAL_NAME); \
		echo "Existing binary installed."; \
	else \
		echo "Error: No compiled binary found. Run 'make install'."; \
	fi

uninstall:
	@echo "Removing binary and configuration..."
	@rm -f $(INSTALL_DIR)/$(FINAL_NAME)
	@rm -rf $(CONFIG_DIR)
	@echo "findgit has been uninstalled."

clean: 
	@cargo clean
	@echo "Cleanup completed."

help:
	@echo -e "Usage: make $(CYAN)[command]$(RESET)"
	@echo -e ""
	@echo -e "$(CYAN)Build Commands:$(RESET)"
	@echo -e "  $(GREEN)build$(RESET)          Compile the project in release mode (if changes detected)"
	@echo -e "  $(GREEN)clean$(RESET)          Remove Cargo build artifacts"
	@echo -e ""
	@echo -e "$(CYAN)Installation Commands:$(RESET)"
	@echo -e "  $(GREEN)install$(RESET)        Compile and install binary + initial config"
	@echo -e "  $(GREEN)quick-install$(RESET)  Install already compiled binary (skips build check)"
	@echo -e "  $(GREEN)uninstall$(RESET)      Remove binary and configuration folder"
	@echo -e ""
	@echo -e "$(CYAN)Others:$(RESET)"
	@echo -e "  $(GREEN)help$(RESET)           Show this help menu"
	@echo -e ""
	@echo -e "Remember to add the alias to your $(BOLD).zshrc$(RESET):"
	@echo -e "  $(CYAN)alias fgit='cd \$$(findgit)'$(RESET)"
