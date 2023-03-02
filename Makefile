OUTPUT_DIRECTORY = out

TARGET_BUNDLE_ASSETS = $(OUTPUT_DIRECTORY)/assets.zip
SRC_BUNDLE_ASSETS = assets

TARGET_BUNDLE_SCRIPTS = $(OUTPUT_DIRECTORY)/scripts.zip
SRC_BUNDLE_SCRIPTS = scripts

.PHONY: usage bundle clean

usage:
	echo "Usage: make [bundle] [clean]"

clean:
	rm -rf $(OUTPUT_DIRECTORY)

bundle: clean $(TARGET_BUNDLE_ASSETS) $(TARGET_BUNDLE_SCRIPTS)

$(TARGET_BUNDLE_ASSETS):
	mkdir -p $(OUTPUT_DIRECTORY)
	zip -r $(TARGET_BUNDLE_ASSETS) $(SRC_BUNDLE_ASSETS)

$(TARGET_BUNDLE_SCRIPTS):
	mkdir -p $(OUTPUT_DIRECTORY)
	zip -r $(TARGET_BUNDLE_SCRIPTS) $(SRC_BUNDLE_SCRIPTS)