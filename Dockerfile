FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM ubuntu:latest as runner
WORKDIR /app
# make sure libssl.so.1.1 is available
RUN apt-get update && apt-get install -y libssl1.1
COPY --from=builder /usr/local/cargo/bin/wkz-rs /usr/local/bin/wkz-rs
COPY . .

CMD ["wkz-rs"]