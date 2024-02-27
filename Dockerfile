FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM ubuntu:latest as runner
WORKDIR /app
# make sure libssl.so.1.1 is available
COPY --from=builder /usr/local/cargo/bin/wkz-rs /usr/local/bin/wkz-rs
COPY . .

CMD ["wkz-rs"]