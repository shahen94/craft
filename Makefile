.PHONY: build
build:
	cargo build --bin craft --release

clean:
	cargo clean

test:
	cargo test $(ARGS)

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all
