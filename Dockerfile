FROM rust:1.90-bullseye as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    perl \
    pkg-config \
    libssl-dev \
    gcc \
    make \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install --locked cargo-leptos
RUN rustup target add wasm32-unknown-unknown

COPY Cargo.toml Cargo.lock ./
COPY bots ./bots
COPY game ./game
COPY tournament ./tournament

EXPOSE 3000
EXPOSE 3001