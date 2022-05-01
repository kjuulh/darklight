FROM rust:1.60 as builder

WORKDIR /usr/src/darklight

RUN mkdir -p src && echo "fn main() {}" > ./src/main.rs
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/darklight /usr/local/bin/darklight
CMD ["darklight"]