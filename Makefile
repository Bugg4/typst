BINARY := target/release/typst
INSTALL_PATH := /usr/local/bin/typstex

.PHONY: install build

build:
	cargo build --release --package typst-cli

install: build
	sudo ln -sf $(CURDIR)/$(BINARY) $(INSTALL_PATH)
	@echo "Installed: $(INSTALL_PATH) -> $(CURDIR)/$(BINARY)"
