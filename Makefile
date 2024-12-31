# Variables
DOCKER_IMAGE = rust-pi
TARGET = aarch64-unknown-linux-gnu
PROJECT_DIR = $(shell pwd)
BUILD_DIR = target/$(TARGET)/release

# Default target
all: build
	
shell:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-it -w /project \
		$(DOCKER_IMAGE) \
		/bin/bash

build:
	cargo build --release

build_docker: 
	./docker/build_rust-pi_docker.sh

build_rpi64:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		$(DOCKER_IMAGE) \
		cargo build --release --target $(TARGET)


# Clean target
clean_rpi64:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		$(DOCKER_IMAGE) \
		cargo clean

# Run target
run:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		$(DOCKER_IMAGE) \
		$(BUILD_DIR)/your-binary-name

# Help target
help:
	@echo "Usage:"
	@echo "  make build  - Compile the project"
	@echo "  make clean  - Clean the
