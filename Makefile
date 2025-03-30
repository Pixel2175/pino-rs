all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/pino-rs /bin/pino

uninstall:
	sudo rm -f /bin/walrs
	sudo rm -rf ~/.config/pino

clean:
	cargo clean

.PHONY: all build install uninstall clean
