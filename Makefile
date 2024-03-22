OUTPUT_DIRECTORY = out

TARGET_BUNDLE_ASSETS = $(OUTPUT_DIRECTORY)/assets.zip
SRC_BUNDLE_ASSETS = assets

TARGET_BUNDLE_SCRIPTS = $(OUTPUT_DIRECTORY)/scripts.zip
SRC_BUNDLE_SCRIPTS = scripts

COVERAGE_DIRECTORY = coverage

TARGET_COVERAGE_SERVER = $(COVERAGE_DIRECTORY)/tarpaulin-report.html

.PHONY: usage clean dev audit lint test server client bundle

usage:
	echo "Usage: make [usage] [clean] [dev] [audit] [lint] [test] [coverage] [server] [client] [bundle]"

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)

dev:
	cargo run

audit:
	cargo deny check bans

lint:
	cargo clippy

test:
	cargo test

$(TARGET_COVERAGE_SERVER): FORCE
	cargo tarpaulin --workspace --all-features --out='Html' --output-dir=$(COVERAGE_DIRECTORY)

coverage: $(TARGET_COVERAGE_SERVER);

server:
	cargo build

client:
	cd ./client && yarn build && cd ..
	rm -rf ./assets/static
	cp -r ./client/out ./assets/static

$(TARGET_BUNDLE_ASSETS):
	mkdir -p $(OUTPUT_DIRECTORY)
	zip -r $(TARGET_BUNDLE_ASSETS) $(SRC_BUNDLE_ASSETS)

$(TARGET_BUNDLE_SCRIPTS):
	mkdir -p $(OUTPUT_DIRECTORY)
	zip -r $(TARGET_BUNDLE_SCRIPTS) $(SRC_BUNDLE_SCRIPTS)

bundle: clean $(TARGET_BUNDLE_ASSETS) $(TARGET_BUNDLE_SCRIPTS)
