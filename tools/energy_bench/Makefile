.PHONY: run build restore check-root

GOVERNOR_DIR := /sys/devices/system/cpu/cpufreq
SAVED_GOVERNOR := /tmp/energy_bench-saved-governor

build:
	cargo build -p energy_bench --release

check-root:
	@if [ "$$(id -u)" != "0" ]; then \
		echo "error: 'make run' requires root to set the CPU governor."; \
		echo "  Run: sudo make run"; \
		exit 1; \
	fi

run: check-root build
	@echo "=== energy_bench ==="
	@echo ""
	@if [ -f "$(GOVERNOR_DIR)/policy0/scaling_governor" ]; then \
		cat "$(GOVERNOR_DIR)/policy0/scaling_governor" > $(SAVED_GOVERNOR); \
		echo "saved governor: $$(cat $(SAVED_GOVERNOR 2>/dev/null) || echo unknown)"; \
	fi
	@for f in $(GOVERNOR_DIR)/policy*/scaling_governor; do \
		if [ -f "$$f" ]; then \
			echo performance > "$$f"; \
		fi; \
	done
	@echo "governor set to: performance"
	@echo ""
	taskset -c 0 ./target/release/energy_bench; EXIT_CODE=$$?
	@echo ""
	@if [ -f $(SAVED_GOVERNOR) ]; then \
		GOV=$$(cat $(SAVED_GOVERNOR)); \
		rm -f $(SAVED_GOVERNOR); \
		if [ -n "$$GOV" ]; then \
			for f in $(GOVERNOR_DIR)/policy*/scaling_governor; do \
				if [ -f "$$f" ]; then \
					echo "$$GOV" > "$$f"; \
				fi; \
			done; \
			echo "restored governor to: $$GOV"; \
		fi; \
	fi; \
	exit $$EXIT_CODE
