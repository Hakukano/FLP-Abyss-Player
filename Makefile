COVERAGE_DIRECTORY = coverage

TARGET_COVERAGE_SERVER = $(COVERAGE_DIRECTORY)/tarpaulin-report.html

.PHONY: usage clean

usage:
	echo "Usage: make [coverage] [clean]"

FORCE: ;

clean:
	rm -rf $(COVERAGE_DIRECTORY)

$(TARGET_COVERAGE_SERVER): FORCE
	cargo tarpaulin --workspace --all-features --out='Html' --output-dir=$(COVERAGE_DIRECTORY)

coverage: $(TARGET_COVERAGE_SERVER);
