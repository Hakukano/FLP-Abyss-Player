.PHONY: usage clean audit lint test

usage:
	echo "Usage: make [coverage] [clean]"

FORCE: ;

clean:
	rm -rf $(COVERAGE_DIRECTORY)

audit: FORCE
	cargo deny --all-features check bans

lint: FORCE
	cargo clippy --all-features

test: FORCE
	cargo test --all-features -- --nocapture
