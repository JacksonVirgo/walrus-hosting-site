ARG RUST_VERSION=1.89.0
ARG APP_NAME=walrus

FROM rust:${RUST_VERSION}-alpine AS build

RUN apk add musl-dev nodejs npm bash

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
COPY ./public ./public

RUN npm install tailwindcss @tailwindcss/cli
RUN npx @tailwindcss/cli -i ./public/styles.css -o ./public/output.css #--minify

RUN cargo build --release

CMD ./target/release/walrus
