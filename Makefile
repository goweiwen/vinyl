ROOT_DIR := $(shell pwd)
BUILD_DIR := target/arm-unknown-linux-gnueabihf/release

CROSS_TARGET_TRIPLE := arm-unknown-linux-gnueabihf

.PHONY: all
all: build

.PHONY: clean
clean:
	cross clean

.PHONY: build
build:
	cross build --release --target=$(CROSS_TARGET_TRIPLE) --features=miyoo

.PHONY: simulator
simulator:
	WAYLAND_DISPLAY= RUST_BACKTRACE=1 cargo run --features simulator

.PHONY: lint
lint:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged --all-targets
