default:

# Some packages (looking at you, minimp3-sys) don't seem to respect the Cargo target.
# This seems to force it to use the right compiler.
CC=arm-none-linux-gnueabihf-gcc

build:
	cargo build --target armv7-unknown-linux-gnueabihf

clean:
	cargo clean --target armv7-unknown-linux-gnueabihf