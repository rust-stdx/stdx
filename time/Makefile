# Regenerates the embedded IANA time zone database.
#
# Requires `zic` and a copy of the IANA `tzdata.zi` source, typically found at
# /usr/share/zoneinfo/tzdata.zi on Linux systems.

ZIC ?= zic
TZDATA_ZI ?= /usr/share/zoneinfo/tzdata.zi

.PHONY: gen-tzdb fetch-data

# Compiles the IANA source to "slim" TZif and regenerates tzdb.bin + tzdb_data.rs.
gen-tzdb:
	rustc --edition=2021 -O -o /tmp/stdx-gen-tzdb tools/gen_tzdb.rs
	@tmp=$$(mktemp -d) && \
		$(ZIC) -b slim -d $$tmp $(TZDATA_ZI) && \
		/tmp/stdx-gen-tzdb $(TZDATA_ZI) $$tmp tzdb.bin tzdb_data.rs && \
		rm -rf $$tmp

# Updates the local copy of the IANA source (data-only release).
fetch-data:
	curl -sSL https://data.iana.org/time-zones/tzdata-latest.tar.gz -o /tmp/tzdata.tar.gz
	mkdir -p /tmp/stdx-tzdata && tar -xzf /tmp/tzdata.tar.gz -C /tmp/stdx-tzdata
	$(MAKE) gen-tzdb TZDATA_ZI=/tmp/stdx-tzdata/$(shell tar -tzf /tmp/tzdata.tar.gz | head -1 | cut -d/ -f1)/tzdata.zi
