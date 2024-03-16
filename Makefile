.PHONY: usage clean audit lint dev test

usage:
	echo "Usage: make [clean] [audit] [lint] [dev] [test]"

FORCE: ;

clean:
	rm -rf $(COVERAGE_DIRECTORY)

audit: FORCE
	cargo deny --all-features check bans

lint: FORCE
	cargo clippy --all-features

dev: FORCE
	cargo tauri dev

test: FORCE
	cargo test --all-features -- --nocapture
