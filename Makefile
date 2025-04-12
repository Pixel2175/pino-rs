all: build

build:
	CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo build --release 

install: build
	sudo install -m755 target/release/pino /bin/pino

uninstall:
	sudo rm -f /bin/pino
	sudo rm -rf ~/.config/pino

clean:
	cargo clean

.PHONY: all build install uninstall clean
