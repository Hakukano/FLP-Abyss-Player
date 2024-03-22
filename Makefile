.PHONY: usage clean audit lint test dev

usage:
	echo "Usage: make [clean] [audit] [lint] [test] [dev]"

FORCE: ;

clean:
	rm -rf $(COVERAGE_DIRECTORY)

audit: FORCE
	yarn && yarn audit && cargo deny --all-features check bans

lint: FORCE
	yarn && yarn lint && cargo clippy --all-features

test: FORCE
	yarn && yarn test && cargo test --all-features -- --nocapture

dev: FORCE
	cargo tauri dev
