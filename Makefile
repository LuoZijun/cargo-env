SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc

.PHONY: all
all:
	cargo build --release \
	&& cargo install --path . \
	&& cargo clean \
	&& echo "ok cargo-env installed" 

.PHONY: build
build:
	cargo build --release

.PHONY: run
run:
	cargo run env

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean