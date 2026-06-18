.PHONY: fmt
fmt:
	cargo +nightly fmt


.PHONY: clean
clean:
	rm -rf target


.PHONY: update_deps
update_deps:
	cargo update


.PHONY: s3_integration_test
s3_integration_test:
	$(MAKE) -C s3 integration-test


.PHONY: check
check:
	cargo check
	cargo check --all-features
	cargo check --no-default-features


.PHONY: check_all
check_all: check
	RUSTFLAGS="-C target-feature=-avx2,-avx512f" cargo check --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=-avx2,-avx512f" cargo check --target=x86_64-unknown-linux-gnu --no-default-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f" cargo check --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f" cargo check --target=x86_64-unknown-linux-gnu --no-default-features

	# aarch64 assumes that NEON instructions are always present
	cargo check --target=aarch64-unknown-linux-gnu --all-features
	cargo check --target=aarch64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-C target-feature=-simd128" cargo check --target=wasm32-wasip1 --all-features
	RUSTFLAGS="-C target-feature=-simd128" cargo check --target=wasm32-wasip1--no-default-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check --target=wasm32-wasip1 --all-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check --target=wasm32-wasip1 --no-default-features


.PHONY: docs
docs:
	RUSTDOCFLAGS='--cfg docsrs' cargo +nightly doc --no-deps --all-features
	node tools/docs-index/docs_index.js
