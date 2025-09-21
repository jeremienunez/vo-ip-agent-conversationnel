CARGO ?= cargo

.PHONY: fmt lint test test-integration miri build bootstrap check-memory

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --all-targets

test-integration:
	$(CARGO) test --workspace --tests

miri:
	$(CARGO) miri test --workspace

build:
	$(CARGO) build --workspace

check-memory:
	heaptrack --version >/dev/null 2>&1 || { echo "heaptrack not installed"; exit 1; }
	$(CARGO) build --release --workspace

bootstrap:
	rustup toolchain install $(shell awk -F '"' '/channel/ { print $$2 }' rust-toolchain.toml)
	rustup component add rustfmt clippy miri
