.PHONY: install build

install: build
	install -Dm 755 $(CURDIR)/target/release/lymap /usr/local/bin/lymap	
	
	mkdir -p /usr/share/lymap
	cp -r $(CURDIR)/assets /usr/share/lymap/
	
	cp $(CURDIR)/lymap.desktop /usr/share/applications/

	cargo clean

build:
	cargo build --release

uninstall:
	rm /usr/local/bin/lymap
	rm -rf /usr/share/lymap
	rm /usr/share/applications/lymap.desktop

	@echo "Do not forget to uninstall the config file."
	@echo "It is either located in $XDG_CONFIG_HOME/lymap or in ~/.config/lymap"

binary_install:
	install -Dm 755 $(CURDIR)/lymap /usr/local/bin/lymap	
	
	mkdir -p /usr/share/lymap
	cp -r $(CURDIR)/assets /usr/share/lymap/
	
	cp $(CURDIR)/lymap.desktop /usr/share/applications/

