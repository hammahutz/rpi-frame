FROM rust:1-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY public ./public

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /app

COPY --from=builder /app/target/release/rpi-frame /usr/local/bin/rpi-frame
COPY --from=builder /app/public ./public

EXPOSE 3000

CMD [ "rpi-frame" ]
