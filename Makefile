VERSION=v0.1.0
NAME=boilertex
EXEC=boilertex
INSTALL_DIR=$(HOME)/.local/bin
ZSH_COMP_DIR=/usr/share/zsh/site-functions/
MAN_DIR=/usr/share/man/man1/

default: build_release

clean:
	@echo "Removing build artifacts..."
	@cargo clean --quiet
	@echo "Done!"

build_release:
	@echo "Building $(NAME) version $(VERSION)..."
	@cargo build --release
	@echo "Finished building $(NAME) version $(VERSION)"

install: build_release
	@echo "Installing $(NAME) version $(VERSION)..."
	@cp target/release/$(EXEC) $(INSTALL_DIR)
	@sudo cp etc/shell_comp/_boilertex $(ZSH_COMP_DIR)
	@sudo cp etc/man/{boilertex.1,boilertex-generate.1,boilertex-preview.1} $(MAN_DIR)
	@echo "$(NAME) version $(VERSION) successfully installed"

uninstall:
	@echo "Uninstalling $(NAME) version $(VERSION)..."
	@rm $(INSTALL_DIR)/$(EXEC)
	@sudo rm $(ZSH_COMP_DIR)/_boilertex
	@sudo rm $(MAN_DIR)/{boilertex.1,boilertex-generate.1,boilertex-preview.1}
	@echo "$(NAME) version $(VERSION) uninstalled"
