FROM rust:1.40-slim as builder

WORKDIR /app

RUN apt-get update
RUN apt-get -y install musl-tools
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1

COPY Cargo.toml Cargo.toml
RUN mkdir src/
COPY src/* src/

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.11

WORKDIR /root/

RUN mkdir tekster
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/syfotekster-rs .

EXPOSE 8080
CMD ["./syfotekster-rs"]
