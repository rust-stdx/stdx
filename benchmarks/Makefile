.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: lint
lint:
	cargo +nightly fmt -- --check
	cargo clippy --workspace --all-targets -- -Dwarnings
