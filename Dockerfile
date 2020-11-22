FROM rust:1.48 as builder

USER root

COPY ./Cargo.* ./
COPY src ./src

RUN cargo build --release

FROM debian:stretch-slim

RUN apt-get update && apt-get install -y openssl && apt-get clean &&  rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/pokebeth ./
