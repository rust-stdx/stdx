# Regenerate the Stripe client from the vendored OpenAPI specification.
#
# Paths are resolved against this Makefile, so the targets work from the
# repository root (`make -f stripe/Makefile codegen`) and from here
# (`make -C stripe codegen`).
HERE := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

.PHONY: codegen
codegen:
	cargo run -p stripe_codegen -- --spec $(HERE)/spec/openapi.spec3.sdk.json --out $(HERE)

# Verify the committed generated code matches the vendored specification.
.PHONY: codegen_check
codegen_check:
	rm -rf /tmp/stripe_codegen_check
	mkdir -p /tmp/stripe_codegen_check
	cp stripe.rs /tmp/stripe_codegen_check/stripe.rs
	cp Cargo.toml /tmp/stripe_codegen_check/Cargo.toml
	cargo run -p stripe_codegen -- --spec $(HERE)/spec/openapi.spec3.sdk.json --out /tmp/stripe_codegen_check
	diff -r api /tmp/stripe_codegen_check/api
	diff stripe.rs /tmp/stripe_codegen_check/stripe.rs

# Run the opt-in Stripe integration tests against a test-mode account.
# Requires both variables; see stripe.rs for details.
.PHONY: integration_test
integration_test:
	@echo "Running Stripe integration tests against the account in STRIPE_API_KEY (must be a test key)"
	@echo "Required: STRIPE_API_KEY=sk_test_... STRIPE_INTEGRATION_TESTS=1"
	# `balance` is needed for the account-mode safety check; it is not in `core`.
	cargo test -p stripe --features balance --test integration -- --nocapture --test-threads=1
