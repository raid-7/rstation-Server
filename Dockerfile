FROM rust:1.72-slim-buster

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y curl htop protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

CMD ["r_station"]
