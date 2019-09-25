.PHONY: build

build:
	cargo build --target=armv7-unknown-linux-gnueabihf --release

.PHONY: upload

upload: build
	scp target/armv7-unknown-linux-gnueabihf/release/clock-firmware clock.local:


.PHONY: test
test:
	scp src/main.c clock.local:
	ssh clock.local gcc -Os -o test main.c
