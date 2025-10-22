ARG RUST_VERSION=1.89.0
ARG APP_NAME=walrus

FROM rust:${RUST_VERSION}-alpine AS build

RUN apk add musl-dev

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src

RUN cargo build --release

CMD ./target/release/walrus
