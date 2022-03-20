FROM rust:1.57.0-buster
COPY . .
RUN apt-get update -qq && apt-get install -y \
    git \
    cmake \
    g++ \
    pkg-config \
    libssl-dev \
    curl \
    llvm \
    clang \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release