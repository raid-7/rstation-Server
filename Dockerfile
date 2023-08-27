FROM rust:1.72-slim-buster

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["app"]
