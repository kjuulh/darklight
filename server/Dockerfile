FROM rust:1.60 as builder

WORKDIR /usr/src/darklight

COPY . .
RUN --mount=type=cache,target=/usr/src/darklight/target cargo build --release


RUN --mount=type=cache,target=/usr/src/darklight/target cargo install --path darklight


FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN python3 -m pip install -U yt-dlp

COPY --from=builder /usr/local/cargo/bin/darklight /usr/local/bin/darklight

ENV ROCKET_ENV production
ENV ROCKET_ADDRESS 0.0.0.0

CMD ["darklight"]