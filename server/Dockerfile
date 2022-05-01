FROM rust:1.60 as builder

WORKDIR /usr/src/darklight

RUN mkdir -p src && echo "fn main() {}" > ./src/main.rs
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN python3 -m pip install -U yt-dlp

COPY --from=builder /usr/local/cargo/bin/darklight /usr/local/bin/darklight

ENV ROCKET_ENV production
ENV ROCKET_ADDRESS 0.0.0.0

CMD ["darklight"]