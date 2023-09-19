FROM rust:bookworm as builder
WORKDIR /usr/src/hp-scanto
COPY . .
RUN cargo install --path .

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies openssl & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-hp /usr/local/bin/rust-hp
CMD ["rust-hp"]
