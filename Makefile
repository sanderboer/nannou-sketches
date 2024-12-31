# Variables
DOCKER_IMAGE = rust-pi
TARGET_64 = aarch64-unknown-linux-gnu
TARGET_32 = armv7-unknown-linux-gnueabihf
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

docker_image: 
	./docker/build_rust-pi_docker.sh


build_rpi64:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		--env PKG_CONFIG_PATH=/usr/aarch64-linux-gnu/lib/pkgconfig:/usr/lib/aarch64-linux-gnu/pkgconfig \
		--env PKG_CONFIG_ALLOW_CROSS=1 \
		$(DOCKER_IMAGE) \
		./build_rpi64.sh

		# rustup target add $(TARGET_64) &&\
		cargo build --release --target $(TARGET_64)


build_rpi32:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		$(DOCKER_IMAGE) \
		env PKG_CONFIG_PATH=/usr/arm-linux-gnueabihf/lib/pkgconfig:/usr/lib/arm-linux-gnueabihf/pkgconfig \
		rustup target add $(TARGET_32) && \
		cargo build --release --target $(TARGET_32)


# Clean target
clean:
	docker run --rm \
		-v $(PROJECT_DIR):/project \
		-w /project \
		$(DOCKER_IMAGE) \
		cargo clean

# Help target
help:
	@echo "Usage:"
	@echo "  make build  - Compile the project natively"
	@echo "  make build_rpi64  - Compile the project for RPi64"
	@echo "  make clean  - Clean the build directory"
	@echo "  make shell  - Open a shell in the container"
	@echo "  make help   - Show this help"

