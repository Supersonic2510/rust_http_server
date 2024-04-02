# Your project's name
PROJECT_NAME = YOUR_PROJECT_NAME

# The path to copy the html folder
PROJECT_BUILD_PATH = /path/to/your/folder

# The release folder
RELEASE_FOLDER = ./target/release

build:
	@echo "Building $(PROJECT_NAME)..."
	@cargo build --release

copy-html:
	# copies the html folder to the given path
	@cp -r ./html $(PROJECT_BUILD_PATH)

clean:
	# delete the target directory
	@cargo clean

.PHONY: all build copy-html clean