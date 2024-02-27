FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:latest as runner
WORKDIR /app
# make sure libssl.so.1.1 is available
RUN apt-get update && apt-get install -y libssl && apt clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/wkz-rs /usr/local/bin/wkz-rs
COPY . .

CMD ["wkz-rs"]