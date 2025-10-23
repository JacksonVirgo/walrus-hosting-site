ARG RUST_VERSION=1.89.0
ARG APP_NAME=walrus

FROM rust:${RUST_VERSION}-alpine AS build

RUN apk add --no-cache \
    musl-dev \
    nodejs \
    npm \
    bash \
    build-base \
    pkgconf \
    openssl-dev \
    openssl-libs-static

WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./tailwind.config.js ./
COPY ./src ./src
COPY ./public ./public
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN npm install tailwindcss @tailwindcss/cli
RUN npx @tailwindcss/cli -i ./public/styles.css -o ./public/output.css #--minify

RUN cargo build --release

CMD ["sh", "-c", "sqlx migrate run && ./target/release/walrus"]
