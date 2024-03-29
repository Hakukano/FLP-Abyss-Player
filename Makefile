OUTPUT_DIRECTORY = out

EXEUTABLE_NAME = flp-abyss-player

SERVER_BUILD = target/release/${EXEUTABLE_NAME}
SERVER_OUT = ${OUTPUT_DIRECTORY}/${EXEUTABLE_NAME}

CLIENT_STATIC_NAME = public

CLIENT_BUILD_DIRECTORY = client/dist
CLIENT_OUT_DIRECTORY = ${OUTPUT_DIRECTORY}/${CLIENT_STATIC_NAME}

COVERAGE_DIRECTORY = coverage
TARGET_COVERAGE_SERVER = $(COVERAGE_DIRECTORY)/tarpaulin-report.html

.PHONY: usage clean audit lint test client dev-server dev-client build

usage:
	echo "Usage: make [usage] [clean] [audit] [lint] [test] [coverage] [client] [dev-server] [dev-client] [build]"

FORCE: ;

clean:
	rm -rf $(OUTPUT_DIRECTORY)
	mkdir -p ${OUTPUT_DIRECTORY}

audit:
	cargo deny check bans
	cd ./client && yarn && yarn audit

lint:
	cargo clippy
	cd ./client && yarn && yarn lint

test:
	cargo test
	cd ./client && yarn && yarn test

$(TARGET_COVERAGE_SERVER): FORCE
	cargo tarpaulin --workspace --all-features --out='Html' --output-dir=$(COVERAGE_DIRECTORY)

coverage: $(TARGET_COVERAGE_SERVER);

dev-server:
	cargo run

dev-client:
	cd ./client && yarn && yarn dev

build: clean
	cd ./client && yarn && yarn build
	cp -r ${CLIENT_BUILD_DIRECTORY} ${CLIENT_OUT_DIRECTORY}

	cargo build --release
	cp ${SERVER_BUILD} ${SERVER_OUT}

	cp LICENSE README.md VERSION ${OUTPUT_DIRECTORY}
