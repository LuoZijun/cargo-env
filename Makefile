SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc

.PHONY: all
all:
	cargo build --release 
	strip target/release/cargo-env 
	upx target/release/cargo-env 
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/cargo-env 
	upx target/release/cargo-env 

.PHONY: run
run:
	cargo run env

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean