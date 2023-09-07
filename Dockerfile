# Use Debian old stable as a parent image
FROM debian:oldstable

# Set environment variables to non-interactive (this prevents some prompts)
ENV DEBIAN_FRONTEND=non-interactive
# Set environment variables for Rust
ENV CC=gcc

# Run package updates and install packages
RUN apt-get update \
    && apt-get -y install curl \
    libwebkit2gtk-4.0-dev \
    build-essential \
    apt-transport-https \
    ca-certificates \
    git \
    gcc \
    libgtk-3-dev \
    libssl-dev \
    pkg-config \
    libasound2 \
    libx11-xcb1 \
    libxcb-dri3-0 \
    libdrm2 \
    libgbm1 \
    libxshmfence1 \
    && apt-get clean

# Install Node.js and npm for esbuild
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Tauri CLI, WASM target, Trunk, and wasm-bindgen
RUN cargo install --locked tauri-cli \
    && rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk \
    && cargo install --locked wasm-bindgen-cli

# Install esbuild
RUN npm install --global --save-exact esbuild

# Clone your repository
RUN git clone https://github.com/JamesClarke7283/ChartCharm /ChartCharm

# Navigate to the project directory
WORKDIR /ChartCharm

# Build the Tauri application
#RUN cargo tauri build

# Specify the output directory where you want to have the built binaries
VOLUME ["/ChartCharm/target"]
