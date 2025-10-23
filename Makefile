PROJECT_NAME := walrus

install:
	cargo fetch
	cargo install sqlx-cli --no-default-features --features postgres

build-container:
	docker build -t walrus .

build-db-queries:
	sqlx migrate run
	cargo sqlx prepare -- --lib

run:
	docker compose --profile dev up --watch

run-prod:
	docker compose --profile prod up

test:
	cargo test

.PHONY: install run run-prod test build-container build-db-queries
