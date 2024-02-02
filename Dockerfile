FROM rust:buster as builder

RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /usr/src/app/target/release/fastlem-random-terrain /usr/local/bin/fastlem-random-terrain
