# The AAGUID directory is a frozen snapshot of the community
# https://github.com/passkeydeveloper/passkey-authenticator-aaguids list, pinned
# to a commit: upstream accepts no new submissions, returns an empty document
# from 2027-01-06 and is archived on 2027-07-06. Regenerating from the pin is
# therefore reproducible, and this target is a verification of that pin rather
# than a way to pick up new authenticators. Migrating the directory to the FIDO
# Metadata Service is tracked as a TODO in `passkey_aaguids.rs`.
PINNED_COMMIT := d53c0f358deeec6ba23d6c6e1f25f32ce45c22f9
SOURCE := https://raw.githubusercontent.com/passkeydeveloper/passkey-authenticator-aaguids/$(PINNED_COMMIT)/combined_aaguid.json


.PHONY: update_data
update_data:
	@set -eu; \
	tmp=$$(mktemp); \
	trap 'rm -f "$$tmp"' EXIT; \
	curl -fsSL '$(SOURCE)' -o "$$tmp"; \
	node tools/update_data.js "$$tmp" aaguids.rs; \
	cargo +nightly fmt -p passkey_aaguids
