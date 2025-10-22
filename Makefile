PROJECT_NAME := walrus

install:
	cargo fetch

build-container:
	docker build -t walrus .

run:
	docker compose --profile dev up --watch

run-prod:
	docker compose --profile prod up

test:
	cargo test

.PHONY: install run run-prod test build-container
