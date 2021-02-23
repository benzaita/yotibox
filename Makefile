default:

# Using gnueabi because it seems to be required for linking libasound (used by alsa-sys)
TARGET=armv7-unknown-linux-gnueabihf

# Some packages (looking at you, minimp3-sys) don't seem to respect the Cargo target.
# This seems to force it to use the right compiler.
CC=arm-none-linux-gnueabihf-gcc

yotibox-builder-image:
	docker build -t yotibox-builder:latest .

build: yotibox-builder-image
	cross build --target $(TARGET)

clean: yotibox-builder-image
	cross clean --target $(TARGET)

run: yotibox-builder-image
	cross run --target $(TARGET)

scp-to-rpi: build target/rpi-metadata/ip
	scp target/armv7-unknown-linux-gnueabihf/debug/app pi@$(shell cat target/rpi-metadata/ip):./yotibox/app
