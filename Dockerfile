FROM rust:1.57.0-bullseye
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
    liblz4-dev \
    librdkafka-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release