FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo install --path .

CMD ["wkz-rs"]