# Always pass `--target`, even for the host. A single artifact directory means
# the host entries in `check_tier1` are cache hits instead of full rebuilds, and
# the lint/test/simd commands reuse the same layout.
HOST_TARGET := $(shell rustc -vV | sed -n 's/^host: //p')

# Pass `--locked` to every command that resolves dependencies so CI and local
# runs agree with the committed lockfiles. Override it (`make check LOCKED=`)
# while intentionally updating dependencies.
LOCKED ?= --locked

# Toolchain used for formatting and documentation. CI pins an exact nightly so a
# rustfmt or rustdoc change cannot turn an unrelated change red; override it
# locally (`make lint NIGHTLY=nightly`) to check against a newer nightly.
NIGHTLY ?= nightly

.PHONY: fmt
fmt:
	cargo +$(NIGHTLY) fmt
	cargo +$(NIGHTLY) fmt --manifest-path benchmarks/Cargo.toml


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
lint: lock_check
	cargo +$(NIGHTLY) fmt -- --check
	cargo +$(NIGHTLY) fmt --manifest-path benchmarks/Cargo.toml -- --check
	cargo clippy $(LOCKED) --target=$(HOST_TARGET) --workspace --all-targets -- -Dwarnings
	cargo clippy $(LOCKED) --target=$(HOST_TARGET) --workspace --all-targets --all-features -- -Dwarnings


# Verify that both committed lockfiles match their manifests without building.
.PHONY: lock_check
lock_check:
	cargo metadata $(LOCKED) --format-version 1 > /dev/null
	cargo metadata $(LOCKED) --manifest-path benchmarks/Cargo.toml --format-version 1 > /dev/null


# Check the workspace against the host target.
.PHONY: check
check: lock_check
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=$(HOST_TARGET)
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=$(HOST_TARGET) --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=$(HOST_TARGET) --no-default-features


# Check Tier 1 platofrms.
.PHONY: check_tier1
check_tier1:
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu --no-default-features

	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-darwin
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-darwin --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-darwin --no-default-features


.PHONY: check_tier2
check_tier2:
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-linux-android
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-linux-android --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-linux-android --no-default-features

	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-ios
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-ios --all-features
	RUSTFLAGS="-D warnings" cargo check $(LOCKED) --target=aarch64-apple-ios --no-default-features


# Check that every package that declares `no_std` compiles for the bare-metal
# Tier 1 targets (riscv32imac-unknown-none-elf and thumbv7em-none-eabi).
.PHONY: check_no_std
check_no_std:
	node tools/check_no_std/check_no_std.js


# Packages with SIMD kernels, detected from the source tree so a new crate with
# SIMD code is covered without editing a list.
SIMD_PKGS := $(shell node tools/ci/test_simd_native.js --list)
SIMD_ARGS := $(addprefix -p ,$(SIMD_PKGS))

# The native checks that force SIMD features on or off only need the packages
# that actually contain SIMD kernels; the baseline x86 checks below stay
# workspace-wide because they are a portability guarantee.
.PHONY: check_cpu_features_x86_64
check_cpu_features_x86_64:
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS)
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=-avx2,-avx512f,-avx512bw,-vaes,-vpclmulqdq,-pclmulqdq,-aes,-sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS) --no-default-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS)
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=+avx2,+avx512f,+avx512bw,+vaes,+vpclmulqdq,+pclmulqdq,+ssse3,+sse4.1,+aes,+sha512" cargo check $(LOCKED) --target=x86_64-unknown-linux-gnu $(SIMD_ARGS) --no-default-features

.PHONY: check_cpu_features_aarch64
check_cpu_features_aarch64:
	# aarch64 assumes that NEON instructions are always present
	RUSTFLAGS="-C target-feature=-aes,-sha2,-sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS)
	RUSTFLAGS="-C target-feature=-aes,-sha2,-sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=-aes,-sha2,-sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS) --no-default-features
	RUSTFLAGS="-C target-feature=+aes,+sha2,+sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS)
	RUSTFLAGS="-C target-feature=+aes,+sha2,+sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=+aes,+sha2,+sha3" cargo check $(LOCKED) --target=aarch64-unknown-linux-gnu $(SIMD_ARGS) --no-default-features

# The wasm checks use the same detected packages. Some packages may not build
# for wasm32-wasip1 for reasons unrelated to SIMD (tokio, socket2, ...); list
# those in WASM_SIMD_EXCLUDE rather than maintaining a separate allowlist.
WASM_SIMD_EXCLUDE ?=
WASM_SIMD_ARGS = $(if $(WASM_SIMD_EXCLUDE),$(addprefix -p ,$(shell node tools/ci/test_simd_native.js --list $(addprefix --exclude ,$(WASM_SIMD_EXCLUDE)))),$(SIMD_ARGS))
.PHONY: check_cpu_features_wasm32
check_cpu_features_wasm32:
	RUSTFLAGS="-C target-feature=-simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS)
	RUSTFLAGS="-C target-feature=-simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=-simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS) --no-default-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS)
	RUSTFLAGS="-C target-feature=+simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS) --all-features
	RUSTFLAGS="-C target-feature=+simd128" cargo check $(LOCKED) --target=wasm32-wasip1 $(WASM_SIMD_ARGS) --no-default-features


# Run the workspace tests with the strongest SIMD feature set the host CPU can
# execute. Falls back to compile-only checks for features the host lacks.
.PHONY: test_simd_native
test_simd_native:
	node tools/ci/test_simd_native.js --target=$(HOST_TARGET)

# Run every crate that builds for wasm32-wasip1 with simd128 enabled.
.PHONY: test_wasm_simd
test_wasm_simd:
	RUSTFLAGS="-C target-feature=+simd128" node tools/wasm_test/wasm_test.js --all-features

# Explicit all-features test run (catches feature-unification gaps).
.PHONY: test_all_features
test_all_features:
	cargo test $(LOCKED) --target=$(HOST_TARGET) --workspace --all-features

# Focused Miri runs for the unsafe-heavy crates. This is a test tool only:
# install the component with `rustup component add miri --toolchain <nightly>`.
MIRIFLAGS ?= -Zmiri-strict-provenance -Zmiri-disable-isolation

.PHONY: test_miri
test_miri:
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p anyerr --test audit_downcast_mut --test audit_unsafe
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p zeroize --all-features --test audit_zeroize_uninit
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p constant_time_eq --test audit_constant_time_eq
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p indexmap --all-features --test audit_unsafe
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p small_collections --all-features --test audit_unsafe
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p json --test audit_json_miri
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p alloc_counter --test audit_alloc_counter
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p hyper_utils --test audit_hyper_io
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p utf8 --test audit_utf8 -- --exact miri_scalar_path
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) --manifest-path pin_project_lite/Cargo.toml --test audit_projection --test drop_order --test audit_macro_edge --test audit_unpin_matrix
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p yaml --test audit_raw_emitter -- --exact write_after_flush_error_overflows_raw_buffer
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p yaml --test audit_raw_emitter -- --exact dumper_shared_node_at_depth_two
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p yaml --test audit_raw_emitter -- --exact loader_second_load_after_error_returns_fail
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p yaml --test audit_raw_emitter -- --exact parser_rejects_input_longer_than_c_int_range
	MIRIFLAGS="$(MIRIFLAGS)" cargo +$(NIGHTLY) miri test $(LOCKED) -p yaml --test audit_parser_safe -- --exact every_truncation_of_representative_document


# Build the documentation warning-free and generate the landing page / index.
.PHONY: docs
docs:
	RUSTDOCFLAGS='--cfg docsrs -D warnings' cargo +$(NIGHTLY) doc $(LOCKED) --no-deps --all-features
	node tools/docs-index/docs_index.js


.PHONY: sql
sql:
	psql $(shell cat .env | grep DATABASE_URL= | cut -d'"' -f2)
