# Regenerate the Stripe client from the vendored OpenAPI specification.
.PHONY: codegen
codegen:
	cargo run -p stripe_codegen -- --spec spec/openapi.spec3.sdk.json --out stripe

# Verify the committed generated code matches the vendored specification.
.PHONY: codegen_check
codegen_check:
	rm -rf /tmp/stripe_codegen_check
	mkdir -p /tmp/stripe_codegen_check
	cp stripe.rs /tmp/stripe_codegen_check/stripe.rs
	cp Cargo.toml /tmp/stripe_codegen_check/Cargo.toml
	cargo run -p stripe_codegen -- --spec spec/openapi.spec3.sdk.json --out /tmp/stripe_codegen_check
	diff -r api /tmp/stripe_codegen_check/api
	diff stripe.rs /tmp/stripe_codegen_check/stripe.rs

# Run the opt-in Stripe integration tests against a test-mode account.
# Requires both variables; see stripe.rs for details.
.PHONY: integration_test
integration_test:
	@echo "Running Stripe integration tests against the account in STRIPE_API_KEY (must be a test key)"
	@echo "Required: STRIPE_API_KEY=sk_test_... STRIPE_INTEGRATION_TESTS=1"
	cargo test -p stripe --test integration -- --nocapture --test-threads=1
