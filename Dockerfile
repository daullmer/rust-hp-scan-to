FROM rust:1.72 as builder
WORKDIR /usr/src/hp-scanto
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-hp /usr/local/bin/rust-hp
CMD ["rust-hp"]
