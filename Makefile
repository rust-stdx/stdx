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


.PHONY: lint
lint:
	cargo +nightly fmt -- --check
	cargo clippy --workspace --all-targets -- -Dwarnings


# Check the workspace against the host target.
.PHONY: check
check:
	RUSTFLAGS="-D warnings" cargo check
	RUSTFLAGS="-D warnings" cargo check --all-features
	RUSTFLAGS="-D warnings" cargo check --no-default-features


# Check Tier 1 platofrms.
.PHONY: check_tier1
check_tier1:
	RUSTFLAGS="-D warnings" cargo check --target=x86_64-unknown-linux-gnu
	RUSTFLAGS="-D warnings" cargo check --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-D warnings" cargo check --target=x86_64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-D warnings" cargo check --target=aarch64-unknown-linux-gnu
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-unknown-linux-gnu --all-features
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-darwin
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-darwin --all-features
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-darwin --no-default-features


.PHONY: check_tier2
check_tier2:
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-linux-android
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-linux-android --all-features
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-linux-android --no-default-features

	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-ios
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-ios --all-features
	RUSTFLAGS="-D warnings" cargo check --target=aarch64-apple-ios --no-default-features


# Check that every package that declares `no_std` compiles for the bare-metal
# Tier 1 targets (riscv32imac-unknown-none-elf and thumbv7em-none-eabi).
.PHONY: check_no_std
check_no_std:
	node tools/check_no_std/check_no_std.js


.PHONY: check_all
# Crates with wasm `simd128` kernels. Some packages in the workspace can't be checked for
# wasm32-wasip1: it pulls in tokio (`full` features are unsupported on wasm) and socket2
# (which has no `sys` backend for the target). General wasm compatibility is covered by
# `node tools/wasm_test/wasm_test.js` in CI.
# TODO: find a better solution to check more packages
WASM_SIMD_PKGS = -p base64 -p hex -p xxhash -p crypto

check_all:
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check --target=x86_64-unknown-linux-gnu
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check --target=x86_64-unknown-linux-gnu --no-default-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check --target=x86_64-unknown-linux-gnu
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check --target=x86_64-unknown-linux-gnu --no-default-features

	# aarch64 assumes that NEON instructions are always present
	RUSTFLAGS="-C target-feature=-aes,-sha2,-sha3" cargo check --target=aarch64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=-aes,-sha2,-sha3" cargo check --target=aarch64-unknown-linux-gnu --no-default-features
	RUSTFLAGS="-C target-feature=+aes,+sha2,+sha3" cargo check --target=aarch64-unknown-linux-gnu --all-features
	RUSTFLAGS="-C target-feature=+aes,+sha2,+sha3" cargo check --target=aarch64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-C target-feature=-simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS)
	RUSTFLAGS="-C target-feature=-simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS) --all-features
	RUSTFLAGS="-C target-feature=-simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS) --no-default-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS)
	RUSTFLAGS="-C target-feature=+simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS) --all-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check --target=wasm32-wasip1 $(WASM_SIMD_PKGS) --no-default-features


.PHONY: docs
docs:
	RUSTDOCFLAGS='--cfg docsrs' cargo +nightly doc --no-deps --all-features
	node tools/docs-index/docs_index.js


.PHONY: sql
sql:
	psql $(shell cat .env | grep DATABASE_URL= | cut -d'"' -f2)
