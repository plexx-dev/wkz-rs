FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:stable-slim as runner
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wkz-rs /usr/local/bin/wkz-rs
COPY . .

CMD ["wkz-rs"]