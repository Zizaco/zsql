.PHONY: test lint recompile release

.DEFAULT_GOAL = help

help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

test: ## launch tests (and doc example tests)
	cargo test

lint: ## lint and format code
	cargo fmt && cargo check && cargo clippy

build: ## compiles executable in target directory
	cargo build

target/strip.lock:
	cargo install --force cargo-strip
	touch target/strip.lock

release: target/strip.lock ## create release executables
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	strip target/x86_64-unknown-linux-gnu/release/tmsql
	strip target/x86_64-pc-windows-gnu/release/tmsql.exe

recompile: ## compiles executable in target directory even if it already exists
	cargo clean && cargo build

install: ## install zsql locally
	rustc -V && cargo -V && cargo install --path .
