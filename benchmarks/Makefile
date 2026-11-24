# Toolchain used for formatting; override to match the root Makefile or a newer
# nightly (`make lint NIGHTLY=nightly`).
NIGHTLY ?= nightly

# Verify the committed lockfile instead of silently updating it.
LOCKED ?= --locked

.PHONY: fmt
fmt:
	cargo +$(NIGHTLY) fmt

.PHONY: lint
lint:
	cargo +$(NIGHTLY) fmt -- --check
	cargo clippy $(LOCKED) --workspace --all-targets -- -Dwarnings
