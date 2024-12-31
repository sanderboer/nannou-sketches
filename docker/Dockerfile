# Base image with Rust
FROM rust:latest

# Create a non-root user with UID 1000 and GID 1000
RUN groupadd -g 1000 developer && \
    useradd -m -u 1000 -g 1000 -s /bin/bash developer

USER root
# Add armhf and arm64 architectures and install dependencies
RUN dpkg --add-architecture armhf && \
    dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y --no-install-recommends \
    sudo \
    mesa-vulkan-drivers \
    vulkan-tools \
    libvulkan1 \
    libvulkan-dev \
    libgl1-mesa-dri \
    libgl1-mesa-dev \
    libgles2-mesa-dev \
    mesa-common-dev \
    build-essential \
    libgl1 \
    libgles2 \
    libglx-mesa0 \
    libegl1 \
    libegl-mesa0 \
    gcc-arm-linux-gnueabihf \
    libc6-armhf-cross \
    libc6-dev-armhf-cross \
    gcc-aarch64-linux-gnu \
    libc6-arm64-cross \
    libc6-dev-arm64-cross \
    qemu-user \
    binutils-arm-linux-gnueabihf \
    binutils-aarch64-linux-gnu \
    mesa-vulkan-drivers:armhf \
    libvulkan1:armhf \
    libvulkan-dev:armhf \
    libgl1-mesa-dri:armhf \
    libgl1-mesa-dev:armhf \
    libgles2-mesa-dev:armhf \
    mesa-common-dev:armhf \
    libegl1:armhf \
    libegl-mesa0:armhf \
    libgles2:armhf \
    libglx-mesa0:armhf \
    mesa-vulkan-drivers:arm64 \
    libvulkan1:arm64 \
    libvulkan-dev:arm64 \
    libgl1-mesa-dri:arm64 \
    libgl1-mesa-dev:arm64 \
    libgles2-mesa-dev:arm64 \
    mesa-common-dev:arm64 \
    libegl1:arm64 \
    libegl-mesa0:arm64 \
    libgles2:arm64 \
    libglx-mesa0:arm64 \
    
    libx11-dev:armhf \
    libx11-dev:arm64 \
    libxcb1-dev:armhf \
    libxcb1-dev:arm64 \
    libxcb-render0-dev:armhf \
    libxcb-render0-dev:arm64 \
    libxcb-shape0-dev:armhf \
    libxcb-shape0-dev:arm64 \
    libxcb-xfixes0-dev:armhf \
    libxcb-xfixes0-dev:arm64 \
    pkg-config \
    && echo "developer ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set up environment variables for pkg-config for cross-compilation
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_SYSROOT_DIR_armv7=/usr/arm-linux-gnueabihf
ENV PKG_CONFIG_SYSROOT_DIR_aarch64=/usr/aarch64-linux-gnu
ENV PKG_CONFIG_PATH_armv7=/usr/arm-linux-gnueabihf/lib/pkgconfig:/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_PATH_aarch64=/usr/aarch64-linux-gnu/lib/pkgconfig:/usr/lib/aarch64-linux-gnu/pkgconfig

# Ensure Rust binaries are added to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install the Rust cross-compilation helper (cross) for convenience
RUN cargo install cross

# Change ownership of the cargo directory to the developer user
RUN mkdir -p /usr/local/cargo /usr/local/rustup && \
    chown -R developer:developer /usr/local/cargo /usr/local/rustup

# Switch to the new user
USER developer

# Set up the working directory
WORKDIR /project

# Optionally, pre-fetch dependencies for a smoother experience
# RUN mkdir -p /project && echo '[dependencies]' > /project/Cargo.toml
# RUN cargo fetch

# Set default command to drop into a shell for manual compilation
CMD ["/bin/bash"]

