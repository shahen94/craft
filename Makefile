define log
	@echo "\n\n\n"
	@echo  "\033[31m $1"
endef

.PHONY: build
build:
	$(call log, "Building craft")
	@cargo build --bin craft --release

clean:
	cargo clean

test:
	$(call log, "Running tests")
	@cargo test $(ARGS)

test-all: fmt test lint

lint:
	$(call log, "Running clippy")
	@cargo clippy --all-targets --all-features -- -D warnings

fmt:
	$(call log, "Running fmt")
	@cargo fmt --all
