
# Base image with Rust
FROM rust:latest

# Ensure Rust binaries are added to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN dpkg --add-architecture armhf && \
    apt-get update && apt-get install -y --no-install-recommends \
    libxcb1-dev:armhf \
    libxcb-render0-dev:armhf \
    libxcb-shape0-dev:armhf \
    libxcb-xfixes0-dev:armhf \
    build-essential \
    gcc-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    qemu-user \
    binutils-arm-linux-gnueabihf \
    libx11-dev:armhf \
    libx11-dev \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    && rustup target add arm-unknown-linux-gnueabihf

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf
ENV PKG_CONFIG_PATH=/usr/arm-linux-gnueabihf/lib/pkgconfig:/usr/lib/arm-linux-gnueabihf/pkgconfig

RUN cargo install cross

# Set up the working directory inside the container
WORKDIR /app

